use super::writer_utils::WriteCgroupsDataMsg;
use super::writer_utils::WriteDataMsg;
use super::writer_utils::WriteGPUDataMsg;
use crate::common::parser_error::ProcParserError;
use crate::common::parser_error::Result;
use crate::configs::config::Config;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use std::fmt::Debug;
use std::sync::Mutex;

fn print_locked<T>(label: &str, data: &Arc<Mutex<T>>)
where
    T: Debug,
{
    match data.lock() {
        Ok(guard) => {
            println!("{label}: {:?}", &*guard);
        }

        Err(error) => {
            eprintln!("Failed to lock {label} message: {error}");
        }
    }
}

pub fn start_writer(
    config: &'static Config,
    signaller: Arc<AtomicBool>,
    receiver: Receiver<WriteDataMsg>,
) -> Result<JoinHandle<()>> {
    let thread_builder = thread::Builder::new().name("ProcParserWriterThread".to_string());

    thread_builder
        .spawn(move || {
            while !signaller.load(Ordering::Relaxed) {
                match receiver.recv_timeout(Duration::from_secs(1)) {
                    Ok(WriteDataMsg::LoadAvg(msg)) => {
                        // print_locked("LoadAvg", &msg.data);
                    }

                    Ok(WriteDataMsg::MemInfo(msg)) => {
                        // print_locked("MemInfo", &msg.data);
                    }

                    Ok(WriteDataMsg::Stat(msg)) => {
                        // print_locked("Stat", &msg.data);
                    }

                    Ok(WriteDataMsg::Uptime(msg)) => {
                        // print_locked("Uptime", &msg.data);
                    }

                    Ok(WriteDataMsg::VmStat(msg)) => {
                        // print_locked("VmStat", &msg.data);
                    }

                    Ok(WriteDataMsg::DiskStats(msg)) => {
                        // print_locked("DiskStats", &msg.data);
                    }

                    Ok(WriteDataMsg::Pressure(msg)) => {
                        // print_locked("Pressure", &msg.data);
                    }

                    Ok(WriteDataMsg::PIDCgroup(msg)) => {
                        // print_locked("PID Cgroup", &msg.data);
                    }

                    Ok(WriteDataMsg::PIDComm(msg)) => {
                        // print_locked("PID Comm", &msg.data);
                    }

                    Ok(WriteDataMsg::PIDCmdline(msg)) => {
                        // print_locked("PID Cmdline", &msg.data);
                    }

                    Ok(WriteDataMsg::PIDSchedStat(msg)) => {
                        // print_locked("PID SchedStat", &msg.data);
                    }

                    Ok(WriteDataMsg::PIDStat(msg)) => {
                        // print_locked("PID Stat", &msg.data);
                    }

                    Ok(WriteDataMsg::PIDStatm(msg)) => {
                        // print_locked("PID Statm", &msg.data);
                    }

                    Ok(WriteDataMsg::PIDStatus(msg)) => {
                        // print_locked("PID Status", &msg.data);
                    }

                    Err(RecvTimeoutError::Timeout) => {
                        // Normal: no message arrived.
                    }

                    Err(RecvTimeoutError::Disconnected) => {
                        break;
                    }
                }
            }

            if config.debug {
                println!("Procfs writer thread stopped");
            }
        })
        .map_err(|io_error| ProcParserError::ThreadSpawningError(io_error.to_string()))
}

pub fn start_gpu_writer(
    config: &'static Config,
    signaller: Arc<AtomicBool>,
    receiver: Receiver<WriteGPUDataMsg>,
) -> Result<JoinHandle<()>> {
    let thread_builder = thread::Builder::new().name("GPUWriterThread".to_string());

    thread_builder
        .spawn(move || {
            while !signaller.load(Ordering::Relaxed) {
                match receiver.recv_timeout(Duration::from_secs(1)) {
                    Ok(WriteGPUDataMsg::NvidiaDeviceInfo(msg)) => match msg.data.lock() {
                        Ok(device_info) => {
                            // println!("NVIDIA DeviceInfo: {:?}", &*device_info);
                        }

                        Err(error) => {
                            eprintln!(
                                "Failed to lock NVIDIA device-info \
                                     message: {error}"
                            );
                        }
                    },

                    Ok(WriteGPUDataMsg::NvidiaProcs(msg)) => match msg.data.lock() {
                        Ok(process_info) => {
                            // println!("NVIDIA GPU-DATA: {:?}", &*process_info);
                        }

                        Err(error) => {
                            eprintln!(
                                "Failed to lock NVIDIA process \
                                     message: {error}"
                            );
                        }
                    },

                    Err(RecvTimeoutError::Timeout) => {
                        // Normal: no message arrived during this interval.
                    }

                    Err(RecvTimeoutError::Disconnected) => {
                        break;
                    }
                }
            }

            if config.debug {
                println!("GPU writer thread stopped");
            }
        })
        .map_err(|io_error| ProcParserError::ThreadSpawningError(io_error.to_string()))
}

pub fn start_cgroup_writer(
    config: &'static Config,
    signaller: Arc<AtomicBool>,
    receiver: Receiver<WriteCgroupsDataMsg>,
) -> Result<JoinHandle<()>> {
    let thread_builder = thread::Builder::new().name("CgroupFileParserThread".to_string());
    thread_builder
        .spawn(move || {
            while !signaller.load(Ordering::Relaxed) {
                match receiver.recv_timeout(Duration::from_secs(1)) {
                    Ok(data) => match data {
                        WriteCgroupsDataMsg::CgroupEvent(msg) => {
                            // println!("Cgroup Event: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::CgroupProcs(msg) => {
                            // println!("Cgroup Procs: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::CpuPressure(msg) => {
                            // println!("CPU Pressure: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::CpuStat(msg) => {
                            // println!("CPU Stat: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::IoPressure(msg) => {
                            // println!("IO Pressure: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::MemoryCurrent(msg) => {
                            // println!("Memory Current: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::MemoryEvent(msg) => {
                            // println!("Memory Event: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::MemoryPeak(msg) => {
                            // println!("Memory Peak: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::MemoryPressure(msg) => {
                            // println!("Memory Pressure: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::MemoryStat(msg) => {
                            // println!("Memory Stat: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::PidsCurrent(msg) => {
                            // println!("PIDS Current: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::PidsEvent(msg) => {
                            // println!("PIDS Event: {:?}", msg);
                        }
                        WriteCgroupsDataMsg::PidsPeak(msg) => {
                            // println!("PIDS Peak: {:?}", msg);
                        }
                    },

                    Err(RecvTimeoutError::Timeout) => {
                        // Normal: no message arrived during this interval.
                    }

                    Err(RecvTimeoutError::Disconnected) => {
                        break;
                    }
                }
            }
        })
        .map_err(|io_error| ProcParserError::ThreadSpawningError(io_error.to_string()))
}
