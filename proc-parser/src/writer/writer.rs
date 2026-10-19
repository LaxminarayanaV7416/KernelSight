use super::writer_utils::WriteCgroupsDataMsg;
use super::writer_utils::WriteDataMsg;
use super::writer_utils::WriteGPUDataMsg;
use crate::configs::config::Config;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn start_writer(
    config: &'static Config,
    signaller: Arc<AtomicBool>,
    receiver: Receiver<WriteDataMsg>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        while !signaller.load(Ordering::Relaxed) {
            match receiver.recv_timeout(Duration::from_secs(1)) {
                Ok(data) => match data {
                    WriteDataMsg::LoadAvg(msg) => {
                        // println!("LoadAvg {:?}", msg);
                    }
                    WriteDataMsg::MemInfo(msg) => {
                        // println!("MemInfo {:?}", msg);
                    }
                    WriteDataMsg::Stat(msg) => {
                        // println!("Stat {:?}", msg);
                    }
                    WriteDataMsg::Uptime(msg) => {
                        // println!("Uptime {:?}", msg);
                    }
                    WriteDataMsg::VmStat(msg) => {
                        // println!("VmStat {:?}", msg);
                    }
                    WriteDataMsg::DiskStats(msg) => {
                        // println!("Diskstats {:?}", msg);
                    }
                    WriteDataMsg::Pressure(msg) => {
                        // println!("Pressure {:?}", msg);
                    }
                    WriteDataMsg::PIDCgroup(msg) => {
                        // println!("Cgroup {:?}", msg);
                    }
                    WriteDataMsg::PIDComm(msg) => {
                        // println!("comm {:?}", msg);
                    }
                    WriteDataMsg::PIDCmdline(msg) => {
                        // println!("cmdline {:?}", msg);
                    }
                    WriteDataMsg::PIDSchedStat(msg) => {
                        // println!("Schedstat {:?}", msg);
                    }
                    WriteDataMsg::PIDStat(msg) => {
                        // println!("PID Stat {:?}", msg);
                    }
                    WriteDataMsg::PIDStatm(msg) => {
                        // println!("Statm {:?}", msg);
                    }
                    WriteDataMsg::PIDStatus(msg) => {
                        // println!("Status {:?}", msg);
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
            println!("Writer thread stopped");
        }
    })
}

pub fn start_gpu_writer(
    config: &'static Config,
    signaller: Arc<AtomicBool>,
    receiver: Receiver<WriteGPUDataMsg>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        while !signaller.load(Ordering::Relaxed) {
            match receiver.recv_timeout(Duration::from_secs(1)) {
                Ok(data) => match data {
                    WriteGPUDataMsg::NvidiaDeviceInfo(msg) => {
                        // println!("NVIDIA DeviceInfo {:?}", msg);
                    }
                    WriteGPUDataMsg::NvidiaProcs(msg) => {
                        // println!("NVIDIA GPU-DATA {:?}", msg);
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
            println!("Writer thread stopped");
        }
    })
}

pub fn start_cgroup_writer(
    config: &'static Config,
    signaller: Arc<AtomicBool>,
    receiver: Receiver<WriteCgroupsDataMsg>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        while !signaller.load(Ordering::Relaxed) {
            match receiver.recv_timeout(Duration::from_secs(1)) {
                Ok(data) => match data {
                    WriteCgroupsDataMsg::CgroupEvent(msg) => {
                        println!("Cgroup Event: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::CgroupProcs(msg) => {
                        println!("Cgroup Procs: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::CpuPressure(msg) => {
                        println!("CPU Pressure: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::CpuStat(msg) => {
                        println!("CPU Stat: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::IoPressure(msg) => {
                        println!("IO Pressure: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::MemoryCurrent(msg) => {
                        println!("Memory Current: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::MemoryEvent(msg) => {
                        println!("Memory Event: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::MemoryPeak(msg) => {
                        println!("Memory Peak: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::MemoryPressure(msg) => {
                        println!("Memory Pressure: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::MemoryStat(msg) => {
                        println!("Memory Stat: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::PidsCurrent(msg) => {
                        println!("PIDS Current: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::PidsEvent(msg) => {
                        println!("PIDS Event: {:?}", msg);
                    }
                    WriteCgroupsDataMsg::PidsPeak(msg) => {
                        println!("PIDS Peak: {:?}", msg);
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
            println!("Writer thread stopped");
        }
    })
}
