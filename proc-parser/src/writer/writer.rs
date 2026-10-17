use super::writer_utils::WriteDataMsg;
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
