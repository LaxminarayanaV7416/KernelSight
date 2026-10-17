use crate::common::procfs_constants::{
    PROC_FS_ROOT_PATH, PROC_PID_SCHEDSTAT_FILE_SLUG, PROC_PID_STAT_FILE_SLUG,
    PROC_PID_STATM_FILE_SLUG, PROC_PID_STATUS_FILE_SLUG,
};
use crate::common::system_call_utils::monotonic;
use crate::configs::config::{Config, ProcFSConfigTrait};
use crate::procfs::pidfs::cachable_reads::{
    parse_procfs_pid_cgroup, parse_procfs_pid_cmdline, parse_procfs_pid_comm,
};
use crate::procfs::pidfs::schedstat::ProcFSPIDSchedStatReader;
use crate::procfs::pidfs::stat::ProcFSPIDStatReader;
use crate::procfs::pidfs::statm::ProcFSPIDStatmReader;
use crate::procfs::pidfs::status::ProcFSPIDStatusReader;
use crate::writer::writer_utils::{WriteData, WriteDataMsg};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;
use std::sync::mpsc::Sender;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ProcFSPID {
    pub pid: u64,
}

impl ProcFSPID {
    pub fn new(pid: u64) -> Self {
        Self { pid: pid }
    }
}

// 4. for borrowing usign the PID number, this
// will enable the take method to get what struct value
impl Borrow<u64> for ProcFSPID {
    fn borrow(&self) -> &u64 {
        &self.pid
    }
}

pub struct ProcFSPIDData {
    pub pid: u64,
    pub comm: String,
    pub cmdline: String,
    pub cgroup: String,
    pub schedstat: Option<ProcFSPIDSchedStatReader>,
    pub stat: Option<ProcFSPIDStatReader>,
    pub statm: Option<ProcFSPIDStatmReader>,
    pub status: Option<ProcFSPIDStatusReader>,
}

impl ProcFSPIDData {
    pub fn new(
        pid: u64,
        config: &'static Config,
        transmitter_writer: &Sender<WriteDataMsg>,
    ) -> Self {
        let pid_comm = parse_procfs_pid_comm(pid as usize);
        let pid_cmdline = parse_procfs_pid_cmdline(pid as usize);
        let pid_cgroup = parse_procfs_pid_cgroup(pid as usize);
        // Sending the Cgroup, Comm and Cmdline to the writer
        let time = monotonic();
        transmitter_writer
            .send(WriteDataMsg::PIDCgroup(WriteData::new(
                time,
                pid_cgroup.clone(),
                Some(pid),
                None,
            )))
            .unwrap();
        transmitter_writer
            .send(WriteDataMsg::PIDComm(WriteData::new(
                time,
                pid_comm.clone(),
                Some(pid),
                None,
            )))
            .unwrap();
        transmitter_writer
            .send(WriteDataMsg::PIDCmdline(WriteData::new(
                time,
                pid_cmdline.clone(),
                Some(pid),
                None,
            )))
            .unwrap();
        // load the paths and keep the files open
        let schedstat_path = Path::new(PROC_FS_ROOT_PATH)
            .join(pid.to_string())
            .join(PROC_PID_SCHEDSTAT_FILE_SLUG);
        let stat_path = Path::new(PROC_FS_ROOT_PATH)
            .join(pid.to_string())
            .join(PROC_PID_STAT_FILE_SLUG);
        let statm_path = Path::new(PROC_FS_ROOT_PATH)
            .join(pid.to_string())
            .join(PROC_PID_STATM_FILE_SLUG);
        let status_path = Path::new(PROC_FS_ROOT_PATH)
            .join(pid.to_string())
            .join(PROC_PID_STATUS_FILE_SLUG);
        let is_pid_schedstat_allowed: Box<bool> =
            Box::new(config.procfs.pid.schedstat.is_enabled());
        let schedstat: Option<ProcFSPIDSchedStatReader> = if *is_pid_schedstat_allowed {
            match schedstat_path.to_str() {
                Some(path) => match ProcFSPIDSchedStatReader::new(path, false, false) {
                    Ok(reader) => Some(reader),
                    Err(error) => {
                        eprintln!("Failed to initialize stat reader for PID {pid}: {error}");
                        None
                    }
                },
                None => {
                    eprintln!(
                        "The schedstat path for PID {pid} is not valid UTF-8: {}",
                        stat_path.display()
                    );
                    None
                }
            }
        } else {
            None
        };

        let is_pid_stat_allowed: Box<bool> = Box::new(config.procfs.pid.stat.is_enabled());
        let stat: Option<ProcFSPIDStatReader> = if *is_pid_stat_allowed {
            match stat_path.to_str() {
                Some(path) => match ProcFSPIDStatReader::new(path, false, false) {
                    Ok(reader) => Some(reader),
                    Err(error) => {
                        eprintln!("Failed to initialize stat reader for PID {pid}: {error}");
                        None
                    }
                },
                None => {
                    eprintln!(
                        "The stat path for PID {pid} is not valid UTF-8: {}",
                        stat_path.display()
                    );
                    None
                }
            }
        } else {
            None
        };

        let is_pid_statm_allowed: Box<bool> = Box::new(config.procfs.pid.statm.is_enabled());
        let statm: Option<ProcFSPIDStatmReader> = if *is_pid_statm_allowed {
            match statm_path.to_str() {
                Some(path) => match ProcFSPIDStatmReader::new(path, false, false) {
                    Ok(reader) => Some(reader),
                    Err(error) => {
                        eprintln!("Failed to initialize statm reader for PID {pid}: {error}");
                        None
                    }
                },
                None => {
                    eprintln!(
                        "The statm path for PID {pid} is not valid UTF-8: {}",
                        stat_path.display()
                    );
                    None
                }
            }
        } else {
            None
        };
        let is_pid_status_allowed: Box<bool> = Box::new(config.procfs.pid.status.is_enabled());
        let status: Option<ProcFSPIDStatusReader> = if *is_pid_status_allowed {
            match status_path.to_str() {
                Some(path) => match ProcFSPIDStatusReader::new(path, false, false) {
                    Ok(reader) => Some(reader),
                    Err(error) => {
                        eprintln!("Failed to initialize status reader for PID {pid}: {error}");
                        None
                    }
                },
                None => {
                    eprintln!(
                        "The status path for PID {pid} is not valid UTF-8: {}",
                        stat_path.display()
                    );
                    None
                }
            }
        } else {
            None
        };
        Self {
            pid: pid,
            comm: pid_comm,
            cmdline: pid_cmdline,
            cgroup: pid_cgroup,
            schedstat: schedstat,
            stat: stat,
            statm: statm,
            status: status,
        }
    }

    pub fn read_and_parse(
        &mut self,
        debug: &bool,
        start_time: &'static f64,
        transmitter_writer: &Sender<WriteDataMsg>,
        schedstat_map: &HashMap<usize, bool>,
        stat_map: &HashMap<usize, bool>,
        statm_map: &HashMap<usize, bool>,
        status_map: &HashMap<&'static str, (usize, bool)>,
        schedstat_enabled: &bool,
        stat_enabled: &bool,
        statm_enabled: &bool,
        status_enabled: &bool,
    ) -> bool {
        let time = monotonic();
        if *schedstat_enabled {
            let Some(reader) = self.schedstat.as_mut() else {
                return false;
            };
            if !reader.read_and_parse(schedstat_map) {
                return false;
            }
            transmitter_writer
                .send(WriteDataMsg::PIDSchedStat(WriteData::new(
                    time,
                    *reader.values(),
                    Some(self.pid),
                    None,
                )))
                .unwrap();
            // if *debug {
            //     println!("/proc/{}/schedstat: {:?}", self.pid, reader.values());
            // }
        }

        if *stat_enabled {
            let Some(reader) = self.stat.as_mut() else {
                return false;
            };

            if !reader.read_and_parse(stat_map) {
                return false;
            }
            transmitter_writer
                .send(WriteDataMsg::PIDStat(WriteData::new(
                    time,
                    reader.values().clone(),
                    Some(self.pid),
                    None,
                )))
                .unwrap();
            // if *debug {
            //     println!("/proc/{}/stat: {:?}", self.pid, reader.values());
            // }
        }

        if *statm_enabled {
            let Some(reader) = self.statm.as_mut() else {
                return false;
            };

            if !reader.read_and_parse(statm_map) {
                return false;
            }
            transmitter_writer
                .send(WriteDataMsg::PIDStatm(WriteData::new(
                    time,
                    *reader.values(),
                    Some(self.pid),
                    None,
                )))
                .unwrap();
            // if *debug {
            //     println!("/proc/{}/statm: {:?}", self.pid, reader.values());
            // }
        }
        if *status_enabled {
            let Some(reader) = self.status.as_mut() else {
                return false;
            };

            if !reader.read_and_parse(status_map) {
                return false;
            }
            transmitter_writer
                .send(WriteDataMsg::PIDStatus(WriteData::new(
                    time,
                    reader.values().clone(),
                    Some(self.pid),
                    None,
                )))
                .unwrap();
            // if *debug {
            //     println!("/proc/{}/status: {:?}", self.pid, reader.values());
            // }
        }
        true
    }
}

pub enum PIDActions {
    Add(ProcFSPID),
    Remove(u64),
}
