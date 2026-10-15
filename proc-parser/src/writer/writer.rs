use super::writer_utils::{WriteData, WriteDataMsg};
use crate::configs::config::Config;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};
use std::time::Duration;

// we will send copies to the writer thread such that it writes them to the respective file
pub fn start_writer(
    config: &'static Config,
    signaller: Arc<AtomicBool>,
    receiver: Receiver<WriteDataMsg>,
) {
    while !signaller.load(Ordering::Relaxed) {
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(data) => match data {
                WriteDataMsg::LoadAvg(msg) => {
                    println!("LoadAvg {:?}", msg);
                }
                WriteDataMsg::MemInfo(msg) => {
                    println!("MemInfo {:?}", msg);
                }
                WriteDataMsg::Stat(msg) => {
                    println!("Stat {:?}", msg);
                }
                WriteDataMsg::Uptime(msg) => {
                    println!("Uptime {:?}", msg);
                }
                WriteDataMsg::VmStat(msg) => {
                    println!("VmStat {:?}", msg);
                }
                WriteDataMsg::DiskStats(msg) => {
                    println!("Diskstats {:?}", msg);
                }
                WriteDataMsg::Pressure(msg) => {
                    println!("Pressure {:?}", msg);
                }
            },
            Err(data) => println!("Error Occured {}", data),
        }
    }
}
