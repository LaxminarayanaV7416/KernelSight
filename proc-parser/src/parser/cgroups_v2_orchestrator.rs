use crate::cgroupfs::v2::cgroup_events::CgroupFSV2CgroupEventReader;
use crate::cgroupfs::v2::cgroup_procs::CgroupFSV2CgroupProcsReader;
use crate::cgroupfs::v2::cpu_stat::CgroupFSV2CPUStatReader;
use crate::cgroupfs::v2::memory_current::CgroupFSV2MemoryCurrentReader;
use crate::cgroupfs::v2::memory_events::CgroupFSV2MemoryEventReader;
use crate::cgroupfs::v2::memory_peak::CgroupFSV2MemoryPeakReader;
use crate::cgroupfs::v2::memory_stat::CgroupFSV2MemoryStatReader;
use crate::cgroupfs::v2::pids_current::CgroupFSV2PidsCurrentReader;
use crate::cgroupfs::v2::pids_events::CgroupFSV2PidsEventReader;
use crate::cgroupfs::v2::pids_peak::CgroupFSV2PidsPeakReader;
use crate::common::cgroup_constants::{CGROUP_CPU_PRESSURE_FILE_SLUG, CGROUP_V2_MOUNT_PATH};
use crate::configs::config::Config;
use crate::procfs::pressure::pressure_util::ProcProcessGeneralReader;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub enum CgroupsV2Actions {
    Add(String),
    Remove(String),
}

pub struct CgroupsV2Readers {
    pub cgroup_events: CgroupFSV2CgroupEventReader,
    pub cgroup_procs: CgroupFSV2CgroupProcsReader,
    pub cpu_pressure: ProcProcessGeneralReader,
    pub cpu_stat: CgroupFSV2CPUStatReader,
    pub io_pressure: ProcProcessGeneralReader,
    pub memory_current: CgroupFSV2MemoryCurrentReader,
    pub memory_events: CgroupFSV2MemoryEventReader,
    pub memory_pressure: ProcProcessGeneralReader,
    pub memory_peak: CgroupFSV2MemoryPeakReader,
    pub memory_stat: CgroupFSV2MemoryStatReader,
    pub pids_current: CgroupFSV2PidsCurrentReader,
    pub pids_event: CgroupFSV2PidsEventReader,
    pub pids_peak: CgroupFSV2PidsPeakReader,
}

impl CgroupsV2Readers {
    pub fn new() {}
}

pub fn cgroup_initiater(
    config: &'static Config,
    signaller: Arc<AtomicBool>, // dont clone this we should clone and send this
    receiver: Receiver<CgroupsV2Actions>,
) -> Result<JoinHandle<()>, Box<dyn Error>> {
    // let global_heart_beat = Box::new(config.heart_beat);
    let cgroups_data: Arc<Mutex<HashMap<String, CgroupsV2Readers>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let parser_worker = thread::spawn(move || {
        while !signaller.load(Ordering::Relaxed) {
            match receiver.recv_timeout(Duration::from_secs(1)) {
                Ok(CgroupsV2Actions::Add(cgroup)) => {
                    {
                        let mut data = cgroups_data.lock().unwrap();
                        // data.insert(cgroup, CgroupsV2Readers::new())
                        println!("RECIVED CGROUP: {:?}", cgroup);
                    }
                }

                Ok(CgroupsV2Actions::Remove(cgroup)) => {
                    {
                        let mut data = cgroups_data.lock().unwrap();
                        // data.insert(cgroup, CgroupsV2Readers::new())
                        println!("REMOVED CGROUP: {:?}", cgroup);
                    }
                }

                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }
    });
    Ok(parser_worker)
}
