use crate::common::parser_error::{ProcParserError, Result};
use crate::common::system_call_utils::monotonic;
use crate::configs::config::Config;
use crate::writer::writer_utils::{WriteGPUData, WriteGPUDataMsg};
use nvml_wrapper::Nvml;
use nvml_wrapper::enums::device::UsedGpuMemory;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug, Default, Clone)]
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
    pub pid: Option<u32>,
    pub pid_memory_bytes: Option<u64>,
    pub pid_sm_utilization_pct: Option<u32>,
    pub pid_memory_utilization_pct: Option<u32>,
}

pub struct NvidiaGPUMetrics {
    pub nvml: Option<Nvml>,
    pub gpu_count: u32,
    pub gpu_devices_info: Vec<GpuDeviceInfo>,
}

impl NvidiaGPUMetrics {
    pub fn new() -> Self {
        let nvml = Nvml::init().unwrap();
        let gpu_count = nvml.device_count().unwrap();
        let mut gpu_device_info: Vec<GpuDeviceInfo> = Vec::with_capacity(gpu_count as usize);

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
            gpu_count,
            gpu_devices_info: gpu_device_info,
        }
    }

    pub fn get_device_details(
        &mut self,
        config: &'static Config,
        transmitter_writer: &Sender<WriteGPUDataMsg>,
        signaller: Arc<AtomicBool>,
    ) -> Result<JoinHandle<()>> {
        let global_heart_beat = Box::new(config.gpu_scan_heart_beat);
        let nvml = match self.nvml.take() {
            Some(nvml) => nvml,
            None => {
                eprintln!("Failed to take the NVML instance!!");
                return Err(ProcParserError::GPUError);
            }
        };
        let gpu_count = self.gpu_count;
        let mut prev_timestamp = monotonic();
        // intially push the device info
        for item in &self.gpu_devices_info {
            transmitter_writer
                .send(WriteGPUDataMsg::NvidiaDeviceInfo(WriteGPUData::new(
                    Arc::new(Mutex::new(item.clone())),
                )))
                .unwrap();
        }
        let writer = transmitter_writer.clone();
        let thread_builder = thread::Builder::new().name("GpuParsingThread".to_string());
        let thread_spawned = thread_builder.spawn(move || {
            while !signaller.load(Ordering::Relaxed) {
                let time = monotonic();
                // dont clone the arc, its not what we want, out goal is create a message pool of
                // size 10 and reuse them again and again like a ring buffer
                let mut message_pool: [Arc<Mutex<GpuDeviceMessage>>; 10] = std::array::from_fn(
                    |_| Arc::new(Mutex::new(GpuDeviceMessage::default())));
                let mut message_pool_counter = 0;
                for index in 0..gpu_count {
                    let device = nvml.device_by_index(index).unwrap();
                    let memory = match device.memory_info() {
                        Ok(mem) => mem,
                        Err(err) => panic!("Error occured! {}", err),
                    };
                    // Utilization may not be supported.
                    let utilization_rate = device.utilization_rates().ok();
                    //TODO fix this calculation here!!!
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
                    if process_count > 0 {
                        for process in processes {
                            let util = util_by_pid.get(&process.pid);
                            // reset check we wont allow if grow more stay to the assigend bound
                            if message_pool_counter >= 10 {
                                message_pool_counter = 0;
                            }
                            if let Some(msg) = message_pool.get(message_pool_counter) {
                                if let Ok(mut message) = msg.lock() {
                                    message.timestamp = time;
                                    message.gpu_index= index;
                                    message.gpu_utilization_pct= utilization_rate
                                        .as_ref()
                                        .map(|u| u.gpu)
                                        .unwrap_or(0);
                                    message.memory_utilization_pct= utilization_rate
                                        .as_ref()
                                        .map(|u| u.memory)
                                        .unwrap_or(0);
                                    message.memory_used_bytes= memory.used;
                                    message.memory_free_bytes= memory.free;
                                    message.compute_process_count= process_count;
                                    message.pid= Some(process.pid);
                                    message.pid_memory_bytes= match process.used_gpu_memory {
                                        UsedGpuMemory::Used(bytes) => Some(bytes),
                                        UsedGpuMemory::Unavailable => None,
                                    };
                                    message.pid_sm_utilization_pct= util.map(|u| u.sm_util);
                                    message.pid_memory_utilization_pct= util.map(|u| u.mem_util);
                                }
                                // lets send the message after releasing the lock
                                writer.send(WriteGPUDataMsg::NvidiaProcs(WriteGPUData::new(msg.clone()))).unwrap();
                                message_pool_counter+=1;
                            }
                        }
                    } else {
                        if message_pool_counter >= 10 {
                            message_pool_counter = 0;
                        }
                        if let Some(msg) = message_pool.get(message_pool_counter) {
                            if let Ok(mut message) = msg.lock() {
                                message.timestamp = time;
                                message.gpu_index= index;
                                message.gpu_utilization_pct= utilization_rate
                                    .as_ref()
                                    .map(|u| u.gpu)
                                    .unwrap_or(0);
                                message.memory_utilization_pct= utilization_rate
                                    .as_ref()
                                    .map(|u| u.memory)
                                    .unwrap_or(0);
                                message.memory_used_bytes= memory.used;
                                message.memory_free_bytes= memory.free;
                                message.compute_process_count= process_count;
                                message.pid= None;
                                message.pid_memory_bytes= None;
                                message.pid_sm_utilization_pct= None;
                                message.pid_memory_utilization_pct= None;
                            }
                            // lets send the message after releasing the lock
                            writer.send(WriteGPUDataMsg::NvidiaProcs(WriteGPUData::new(msg.clone()))).unwrap();
                            message_pool_counter+=1;
                    }
                }
                }
                prev_timestamp = time;
                thread::sleep(Duration::from_millis(*global_heart_beat));
            }
        }).map_err(|io_error| ProcParserError::ThreadSpawningError(io_error.to_string()));
        thread_spawned
    }

    pub fn close_nvml(&mut self) -> Result<()> {
        if let Some(nvml) = self.nvml.take() {
            nvml.shutdown()
                .map_err(|err| ProcParserError::ParseError(err.into()))?;
        }
        Ok(())
    }
}
