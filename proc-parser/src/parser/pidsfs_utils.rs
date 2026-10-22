use crate::common::parser_error::Result;
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

/*
 * This struct holds the pid
 * its usually more readable by user so i have maintained
 * a struct, but let me compare the memory profile of
 * maintaining the struct and then revert back if something is wrong
 * As per google I can read as "When you compile your code,
 * the struct wrapper is completely optimized away. The CPU sees
 * the exact same assembly instructions whether the data is
 * wrapped in a struct or stored as a plain variable."
 */
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ProcFSPID {
    pub pid: u64,
}

impl ProcFSPID {
    pub fn new(pid: u64) -> Self {
        Self { pid: pid }
    }
}

impl Borrow<u64> for ProcFSPID {
    // for borrowing usign the PID number, this
    // will enable the take method to get what struct value
    fn borrow(&self) -> &u64 {
        &self.pid
    }
}

pub struct ProcFSPIDReaders {
    pub pid: u64,
    pub comm: String,
    pub cmdline: String,
    pub cgroup: String,
    pub schedstat: Option<ProcFSPIDSchedStatReader>,
    pub stat: Option<ProcFSPIDStatReader>,
    pub statm: Option<ProcFSPIDStatmReader>,
    pub status: Option<ProcFSPIDStatusReader>,
}

impl ProcFSPIDReaders {
    pub fn new(
        pid: u64,
        config: &'static Config,
        transmitter_writer: &Sender<WriteDataMsg>,
    ) -> Result<Self> {
        let time = monotonic();

        // Without a cgroup value there is nothing meaningful to send to the
        // cgroup orchestrator. Return the original error to the caller.
        let pid_cgroup = parse_procfs_pid_cgroup(pid as usize)?;

        // Keep pid_cgroup outside this closure so it remains available for
        // cleanup if any later setup step fails.
        let pid_comm = parse_procfs_pid_comm(pid as usize)?;
        let pid_cmdline = parse_procfs_pid_cmdline(pid as usize)?;

        let pid_root = Path::new(PROC_FS_ROOT_PATH).join(pid.to_string());
        let schedstat_path = pid_root.join(PROC_PID_SCHEDSTAT_FILE_SLUG);
        let stat_path = pid_root.join(PROC_PID_STAT_FILE_SLUG);
        let statm_path = pid_root.join(PROC_PID_STATM_FILE_SLUG);
        let status_path = pid_root.join(PROC_PID_STATUS_FILE_SLUG);
        let schedstat = if config.procfs.pid.schedstat.is_enabled() {
            Some(ProcFSPIDSchedStatReader::new(&schedstat_path)?)
        } else {
            None
        };
        let stat = if config.procfs.pid.stat.is_enabled() {
            Some(ProcFSPIDStatReader::new(&stat_path)?)
        } else {
            None
        };

        let statm = if config.procfs.pid.statm.is_enabled() {
            Some(ProcFSPIDStatmReader::new(&statm_path)?)
        } else {
            None
        };

        let status = if config.procfs.pid.status.is_enabled() {
            Some(ProcFSPIDStatusReader::new(&status_path)?)
        } else {
            None
        };

        // Reader setup succeeded. Only now publish metadata.
        match transmitter_writer.send(WriteDataMsg::PIDCgroup(WriteData::new(
            time,
            pid_cgroup.clone(),
            Some(pid),
            None,
        ))) {
            Ok(_) => (),
            Err(_) => {
                eprintln!(
                    "PROC_PID_PARSER::Sending Message from /proc/{}/cgroup at {} failed!!",
                    pid, time
                );
            }
        };

        match transmitter_writer.send(WriteDataMsg::PIDComm(WriteData::new(
            time,
            pid_comm.clone(),
            Some(pid),
            None,
        ))) {
            Ok(_) => (),
            Err(_) => {
                eprintln!(
                    "PROC_PARSER::Sending Message from /proc/{}/comm at {} failed!!",
                    pid, time
                );
            }
        };

        match transmitter_writer.send(WriteDataMsg::PIDCmdline(WriteData::new(
            time,
            pid_cmdline.clone(),
            Some(pid),
            None,
        ))) {
            Ok(_) => (),
            Err(_) => {
                eprintln!(
                    "PROC_PARSER::Sending Message from /proc/{}/cmdline at {} failed!!",
                    pid, time
                );
            }
        };

        Ok(Self {
            pid,
            comm: pid_comm,
            cmdline: pid_cmdline,
            cgroup: pid_cgroup.clone(),
            schedstat,
            stat,
            statm,
            status,
        })
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
            match transmitter_writer.send(WriteDataMsg::PIDSchedStat(WriteData::new(
                time,
                *reader.values(),
                Some(self.pid),
                None,
            ))) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!(
                        "PROC_PARSER::Sending Message from /proc/{}/schedstat at {} failed!!",
                        self.pid, time
                    );
                }
            };
        }

        if *stat_enabled {
            let Some(reader) = self.stat.as_mut() else {
                return false;
            };
            if !reader.read_and_parse(stat_map) {
                return false;
            }
            match transmitter_writer.send(WriteDataMsg::PIDStat(WriteData::new(
                time,
                reader.values().clone(),
                Some(self.pid),
                None,
            ))) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!(
                        "PROC_PARSER::Sending Message from /proc/{}/stat at {} failed!!",
                        self.pid, time
                    );
                }
            };
        }

        if *statm_enabled {
            let Some(reader) = self.statm.as_mut() else {
                return false;
            };

            if !reader.read_and_parse(statm_map) {
                return false;
            }
            match transmitter_writer.send(WriteDataMsg::PIDStatm(WriteData::new(
                time,
                *reader.values(),
                Some(self.pid),
                None,
            ))) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!(
                        "PROC_PARSER::Sending Message from /proc/{}/statm at {} failed!!",
                        self.pid, time
                    );
                }
            };
        }
        if *status_enabled {
            let Some(reader) = self.status.as_mut() else {
                return false;
            };

            if !reader.read_and_parse(status_map) {
                return false;
            }
            match transmitter_writer.send(WriteDataMsg::PIDStatus(WriteData::new(
                time,
                reader.values().clone(),
                Some(self.pid),
                None,
            ))) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!(
                        "PROC_PARSER::Sending Message from /proc/{}/status at {} failed!!",
                        self.pid, time
                    );
                }
            };
        }
        true
    }
}

pub enum PIDActions {
    Add(ProcFSPID),
    Remove(u64),
}
