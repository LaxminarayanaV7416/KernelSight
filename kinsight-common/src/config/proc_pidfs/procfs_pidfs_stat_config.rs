use super::super::config::ProcFSConfig;
use super::super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ProcPidfsStatConfigFields {
    pub pid: bool,
    pub comm: bool,
    pub state: bool,
    pub ppid: bool,
    pub pgrp: bool,
    pub session: bool,
    pub tty_nr: bool,
    pub tpgid: bool,
    pub flags: bool,
    pub minflt: bool,
    pub cminflt: bool,
    pub majflt: bool,
    pub cmajflt: bool,
    pub utime: bool,
    pub stime: bool,
    pub cutime: bool,
    pub cstime: bool,
    pub priority: bool,
    pub nice: bool,
    pub num_threads: bool,
    pub itrealvalue: bool,
    pub starttime: bool,
    pub vsize: bool,
    pub rss: bool,
    pub rsslim: bool,
    pub startcode: bool,
    pub endcode: bool,
    pub startstack: bool,
    pub kstkesp: bool,
    pub kstkeip: bool,
    pub signal: bool,
    pub blocked: bool,
    pub sigignore: bool,
    pub sigcatch: bool,
    pub wchan: bool,
    pub nswap: bool,
    pub cnswap: bool,
    pub exit_signal: bool,
    pub processor: bool,
    pub rt_priority: bool,
    pub policy: bool,
    pub delayacct_blkio_ticks: bool,
    pub guest_time: bool,
    pub cguest_time: bool,
    pub start_data: bool,
    pub end_data: bool,
    pub start_brk: bool,
    pub arg_start: bool,
    pub arg_end: bool,
    pub env_start: bool,
    pub env_end: bool,
    pub exit_code: bool,
}

impl Default for ProcPidfsStatConfigFields {
    fn default() -> Self {
        Self {
            pid: true,
            comm: true,
            state: true,
            ppid: true,
            pgrp: true,
            session: true,
            tty_nr: true,
            tpgid: true,
            flags: true,
            minflt: true,
            cminflt: true,
            majflt: true,
            cmajflt: true,
            utime: true,
            stime: true,
            cutime: true,
            cstime: true,
            priority: true,
            nice: true,
            num_threads: true,
            itrealvalue: true,
            starttime: true,
            vsize: true,
            rss: true,
            rsslim: true,
            startcode: true,
            endcode: true,
            startstack: true,
            kstkesp: true,
            kstkeip: true,
            signal: true,
            blocked: true,
            sigignore: true,
            sigcatch: true,
            wchan: true,
            nswap: true,
            cnswap: true,
            exit_signal: true,
            processor: true,
            rt_priority: true,
            policy: true,
            delayacct_blkio_ticks: true,
            guest_time: true,
            cguest_time: true,
            start_data: true,
            end_data: true,
            start_brk: true,
            arg_start: true,
            arg_end: true,
            env_start: true,
            env_end: true,
            exit_code: true,
        }
    }
}

pub type ProcPidfsStatConfig = ProcFSConfig<ProcPidfsStatConfigFields>;

impl ProcFSConfigTrait for ProcPidfsStatConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.pid),
            (2, self.fields.comm),
            (3, self.fields.state),
            (4, self.fields.ppid),
            (5, self.fields.pgrp),
            (6, self.fields.session),
            (7, self.fields.tty_nr),
            (8, self.fields.tpgid),
            (9, self.fields.flags),
            (10, self.fields.minflt),
            (11, self.fields.cminflt),
            (12, self.fields.majflt),
            (13, self.fields.cmajflt),
            (14, self.fields.utime),
            (15, self.fields.stime),
            (16, self.fields.cutime),
            (17, self.fields.cstime),
            (18, self.fields.priority),
            (19, self.fields.nice),
            (20, self.fields.num_threads),
            (21, self.fields.itrealvalue),
            (22, self.fields.starttime),
            (23, self.fields.vsize),
            (24, self.fields.rss),
            (25, self.fields.rsslim),
            (26, self.fields.startcode),
            (27, self.fields.endcode),
            (28, self.fields.startstack),
            (29, self.fields.kstkesp),
            (30, self.fields.kstkeip),
            (31, self.fields.signal),
            (32, self.fields.blocked),
            (33, self.fields.sigignore),
            (34, self.fields.sigcatch),
            (35, self.fields.wchan),
            (36, self.fields.nswap),
            (37, self.fields.cnswap),
            (38, self.fields.exit_signal),
            (39, self.fields.processor),
            (40, self.fields.rt_priority),
            (41, self.fields.policy),
            (42, self.fields.delayacct_blkio_ticks),
            (43, self.fields.guest_time),
            (44, self.fields.cguest_time),
            (45, self.fields.start_data),
            (46, self.fields.end_data),
            (47, self.fields.start_brk),
            (48, self.fields.arg_start),
            (49, self.fields.arg_end),
            (50, self.fields.env_start),
            (51, self.fields.env_end),
            (52, self.fields.exit_code),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow
            & self.fields.pid
            & self.fields.comm
            & self.fields.state
            & self.fields.ppid
            & self.fields.pgrp
            & self.fields.session
            & self.fields.tty_nr
            & self.fields.tpgid
            & self.fields.flags
            & self.fields.minflt
            & self.fields.cminflt
            & self.fields.majflt
            & self.fields.cmajflt
            & self.fields.utime
            & self.fields.stime
            & self.fields.cutime
            & self.fields.cstime
            & self.fields.priority
            & self.fields.nice
            & self.fields.num_threads
            & self.fields.itrealvalue
            & self.fields.starttime
            & self.fields.vsize
            & self.fields.rss
            & self.fields.rsslim
            & self.fields.startcode
            & self.fields.endcode
            & self.fields.startstack
            & self.fields.kstkesp
            & self.fields.kstkeip
            & self.fields.signal
            & self.fields.blocked
            & self.fields.sigignore
            & self.fields.sigcatch
            & self.fields.wchan
            & self.fields.nswap
            & self.fields.cnswap
            & self.fields.exit_signal
            & self.fields.processor
            & self.fields.rt_priority
            & self.fields.policy
            & self.fields.delayacct_blkio_ticks
            & self.fields.guest_time
            & self.fields.cguest_time
            & self.fields.start_data
            & self.fields.end_data
            & self.fields.start_brk
            & self.fields.arg_start
            & self.fields.arg_end
            & self.fields.env_start
            & self.fields.env_end
            & self.fields.exit_code
    }
}
