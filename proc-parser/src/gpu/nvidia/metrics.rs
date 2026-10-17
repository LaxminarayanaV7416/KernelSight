use crate::common::system_call_utils::monotonic;
use crate::configs::config::Config;
use crate::writer::writer_utils::{WriteData, WriteDataMsg};
use nvml_wrapper::Nvml;
use nvml_wrapper::enums::device::UsedGpuMemory;
use nvml_wrapper::error::NvmlError;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Default, Debug, Copy, Clone)]
pub struct GPUMetrics {}

#[derive(Debug, Default)]
pub struct GpuDeviceInfo {
    pub gpu_index: u32,
    pub uuid: String,
    pub name: String,
    pub pci_bus_id: String,
    pub total_memory_bytes: u64,
    pub sm_count: u32,
    pub compute_capability_major: i32,
    pub compute_capability_minor: i32,
}

#[derive(Debug, Default)]
pub struct GpuProcessSample {
    pub pid: u32,
    pub gpu_memory_bytes: u64,
    pub sm_utilization_pct: u32,
    pub memory_utilization_pct: u32,
}

#[derive(Debug, Default)]
pub struct GpuDeviceMessage {
    pub timestamp: f64,
    pub gpu_index: u32,
    // Utilization
    pub gpu_utilization_pct: u32,
    pub memory_utilization_pct: u32,
    // Memory
    pub memory_used_bytes: u64,
    pub memory_free_bytes: u64,
    // Processes
    pub compute_process_count: usize,
    // detailed processes
    pub processes: Vec<GpuProcessSample>,
}

pub struct NvidiaGPUMetrics {
    pub nvml: Option<Nvml>,
    pub values: GPUMetrics,
    pub gpu_count: u32,
    pub gpu_devices_info: Vec<GpuDeviceInfo>,
}

impl NvidiaGPUMetrics {
    pub fn new() -> Self {
        let nvml = Nvml::init().unwrap();
        let gpu_count = nvml.device_count().unwrap();
        let mut gpu_device_info: Vec<GpuDeviceInfo> = Vec::new();

        for index in 0..gpu_count {
            let device = nvml.device_by_index(index).unwrap();
            let name = match device.name() {
                Ok(name) => name,
                Err(err) => panic!("Error occured in the NVML library {}", err),
            };
            let uuid = match device.uuid() {
                Ok(uuid) => uuid,
                Err(err) => panic!("Error occured in the NVML library {}", err),
            };
            let pci_info = match device.pci_info() {
                Ok(info) => info,
                Err(err) => panic!("Error occured in the NVML library {}", err),
            };
            let sm_count = device.attributes().ok().map(|a| a.multiprocessor_count);
            let memory_info = match device.memory_info() {
                Ok(mem) => mem,
                Err(err) => panic!("Error occured in the NVML library {}", err),
            };
            let cuda_comp = match device.cuda_compute_capability() {
                Ok(att) => att,
                Err(err) => panic!("Error occured in the NVML library {}", err),
            };
            gpu_device_info.push(GpuDeviceInfo {
                gpu_index: index,
                uuid: uuid,
                name: name,
                pci_bus_id: pci_info.bus_id,
                total_memory_bytes: memory_info.total,
                sm_count: sm_count.unwrap_or(0),
                compute_capability_major: cuda_comp.major,
                compute_capability_minor: cuda_comp.minor,
            });
        }

        Self {
            nvml: Some(nvml),
            values: GPUMetrics::default(),
            gpu_count,
            gpu_devices_info: gpu_device_info,
        }
    }

    pub fn get_device_details(
        &mut self,
        config: &'static Config,
        // transmitter_writer: &Sender<WriteDataMsg>,
        signaller: Arc<AtomicBool>,
    ) -> Result<JoinHandle<()>, Box<dyn Error>> {
        let global_heart_beat = Box::new(config.heart_beat);
        let nvml = match self.nvml.take() {
            Some(nvml) => nvml,
            None => panic!("somethign is wrong!"),
        };
        let gpu_count = self.gpu_count;
        let mut prev_timestamp = monotonic();
        let thread_spawned = thread::spawn(move || {
            while !signaller.load(Ordering::Relaxed) {
                let time = monotonic();
                for index in 0..gpu_count {
                    let device = nvml.device_by_index(index).unwrap();
                    let memory = match device.memory_info() {
                        Ok(mem) => mem,
                        Err(err) => panic!("Error occured! {}", err),
                    };
                    // Utilization may not be supported.
                    let utilization_rate = device.utilization_rates().ok();
                    let utilization = device
                        .process_utilization_stats(Some(prev_timestamp as u64 / 1000))
                        .ok();
                    let processes = match device.running_compute_processes() {
                        Ok(proc) => proc,
                        Err(err) => panic!("Error occured! {}", err),
                    };
                    let mut util_by_pid = HashMap::new();
                    let mut max_timestamp = time as u64;
                    if let Some(samples) = utilization {
                        for sample in samples {
                            max_timestamp = max_timestamp.max(sample.timestamp);
                            // Keep the most recent sample for each PID.
                            let replace = util_by_pid
                                .get(&sample.pid)
                                .map(|old: &nvml_wrapper::struct_wrappers::device::ProcessUtilizationSample| {
                                    sample.timestamp > old.timestamp
                                })
                                .unwrap_or(true);

                            if replace {
                                util_by_pid.insert(sample.pid, sample);
                            }
                        }
                    }
                    let process_count = processes.len();
                    let mut result = Vec::with_capacity(process_count);
                    for process in processes {
                        let util = util_by_pid.get(&process.pid);

                        let memory_bytes = match process.used_gpu_memory {
                            UsedGpuMemory::Used(bytes) => bytes,
                            UsedGpuMemory::Unavailable => 0,
                        };
                        result.push(GpuProcessSample {
                            pid: process.pid,
                            gpu_memory_bytes: memory_bytes,
                            sm_utilization_pct: util.map(|u| u.sm_util).unwrap_or(0),
                            memory_utilization_pct: util.map(|u| u.mem_util).unwrap_or(0),
                        });
                    }

                    let msg = GpuDeviceMessage {
                        timestamp: time,
                        gpu_index: index,
                        gpu_utilization_pct: utilization_rate.as_ref().map(|u| u.gpu).unwrap_or(0),
                        memory_utilization_pct: utilization_rate
                            .as_ref()
                            .map(|u| u.memory)
                            .unwrap_or(0),
                        memory_used_bytes: memory.used,
                        memory_free_bytes: memory.free,
                        compute_process_count: process_count,
                        processes: result,
                    };
                    println!("NVIDIA Msg: {:?}", msg);
                }
                prev_timestamp = time;
                thread::sleep(Duration::from_millis(*global_heart_beat));
            }
        });
        Ok(thread_spawned)
    }

    pub fn close_nvml(&mut self) -> Result<(), NvmlError> {
        if let Some(nvml) = self.nvml.take() {
            nvml.shutdown()?;
        }
        Ok(())
    }
}
