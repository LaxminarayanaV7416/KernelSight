use crate::common::procfs_constants::{
    PROC_FS_ROOT_PATH, PROC_PID_SCHEDSTAT_FILE_SLUG, PROC_PID_STAT_FILE_SLUG,
    PROC_PID_STATM_FILE_SLUG, PROC_PID_STATUS_FILE_SLUG,
};
use crate::configs::config::{Config, ProcFSConfigTrait};
use crate::procfs::pidfs::cachable_reads::{
    parse_procfs_pid_cgroup, parse_procfs_pid_cmdline, parse_procfs_pid_comm,
};
use crate::procfs::pidfs::schedstat::ProcFSPIDSchedStatReader;
use crate::procfs::pidfs::stat::ProcFSPIDStatReader;
use crate::procfs::pidfs::statm::ProcFSPIDStatmReader;
use crate::procfs::pidfs::status::ProcFSPIDStatusReader;
use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::Path;

pub struct ProcFSPID {
    pub pid: u64,
    pub comm: String,
    pub cmdline: String,
    pub cgroup: String,
    pub schedstats: Option<ProcFSPIDSchedStatReader>,
    pub stat: Option<ProcFSPIDStatReader>,
    pub statm: Option<ProcFSPIDStatmReader>,
    pub status: Option<ProcFSPIDStatusReader>,
}

// 1. Manually implement PartialEq using only the `pid` field
impl PartialEq for ProcFSPID {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}

// 2. Derive Eq (since our PartialEq implementation satisfies the total equivalence relation)
impl Eq for ProcFSPID {}

// 3. Manually implement Hash using only the `pid` field
impl Hash for ProcFSPID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pid.hash(state);
    }
}

// 4. for borrowing usign the PID number, this
// will enable the take method to get what struct value
impl Borrow<u64> for ProcFSPID {
    fn borrow(&self) -> &u64 {
        &self.pid
    }
}

// 5. reimplemetning the debug trait
impl fmt::Debug for ProcFSPID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcFSPID")
            .field("PID", &self.pid)
            .field("comm", &self.comm)
            .field("cmdline", &self.cmdline)
            .finish()
    }
}

impl ProcFSPID {
    pub fn new(pid: u64, config: &Config) -> Self {
        let pid_comm = parse_procfs_pid_comm(pid as usize);
        let pid_cmdline = parse_procfs_pid_cmdline(pid as usize);
        let pid_cgroup = parse_procfs_pid_cgroup(pid as usize);
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
        let mut schedstat: Option<ProcFSPIDSchedStatReader> = None;
        if *is_pid_schedstat_allowed {
            schedstat = Some(*Box::new(
                ProcFSPIDSchedStatReader::new(&schedstat_path.to_str().unwrap(), false, false)
                    .unwrap(),
            ));
        }

        let is_pid_stat_allowed: Box<bool> = Box::new(config.procfs.pid.stat.is_enabled());
        let mut stat: Option<ProcFSPIDStatReader> = None;
        if *is_pid_stat_allowed {
            stat = Some(*Box::new(
                ProcFSPIDStatReader::new(&stat_path.to_str().unwrap(), false, false).unwrap(),
            ));
        }

        let is_pid_statm_allowed: Box<bool> = Box::new(config.procfs.pid.statm.is_enabled());
        let mut statm: Option<ProcFSPIDStatmReader> = None;
        if *is_pid_statm_allowed {
            statm = Some(*Box::new(
                ProcFSPIDStatmReader::new(&statm_path.to_str().unwrap(), false, false).unwrap(),
            ));
        }
        let is_pid_status_allowed: Box<bool> = Box::new(config.procfs.pid.status.is_enabled());
        let mut status: Option<ProcFSPIDStatusReader> = None;
        if *is_pid_status_allowed {
            status = Some(*Box::new(
                ProcFSPIDStatusReader::new(&status_path.to_str().unwrap(), false, false).unwrap(),
            ));
        }
        Self {
            pid: pid,
            comm: pid_comm,
            cmdline: pid_cmdline,
            cgroup: pid_cgroup,
            schedstats: schedstat,
            stat: stat,
            statm: statm,
            status: status,
        }
    }
}

pub enum PIDActions {
    Add(ProcFSPID),
    Remove(u64),
}
