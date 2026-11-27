use super::kinsight_utils::KinsightHashBuild;
use kinsight_common::KinsightDataParserTrait;
use kinsight_common::ProcParserError;
use kinsight_common::Result;
use kinsight_common::monotonic;
use kinsight_common::procfs_constants::{
    PROC_FS_ROOT_PATH, PROC_PID_STAT_FILE_SLUG, PROC_PID_STATUS_FILE_SLUG, PROC_SCHEDSTAT_FILE_SLUG,
};
use kinsight_proc_pid_parser::ProcFSPIDSchedStatReader;
use kinsight_proc_pid_parser::ProcFSPIDStatReader;
use kinsight_proc_pid_parser::ProcFSPIDStatusReader;
use kinsight_proc_pid_parser::parse_procfs_pid_cgroup;
use kinsight_proc_pid_parser::parse_procfs_pid_cmdline;
use kinsight_proc_pid_parser::parse_procfs_pid_comm;
use kinsight_writer::CpuPIDMetrics;
use kinsight_writer::MessagePool;
use papaya::HashMap;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap as StdHashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct PidReaders {
    stat: RefCell<ProcFSPIDStatReader>,
    status: RefCell<ProcFSPIDStatusReader>,
    schedstat: RefCell<ProcFSPIDSchedStatReader>,
    cmdline: String,
    comm: String,
    cgroup: String,

    // Per-PID timing state belongs to the PID reader, not a pooled
    // transport message that can be reused for another PID.
    previous_timestamp: Cell<f64>,
}

impl PidReaders {
    fn new(pid: u64) -> Result<Self> {
        let pid_root = Path::new(PROC_FS_ROOT_PATH).join(pid.to_string());
        let pid_stat_path = pid_root.join(PROC_PID_STAT_FILE_SLUG);
        let pid_status_path = pid_root.join(PROC_PID_STATUS_FILE_SLUG);
        let pid_schedstat_path = pid_root.join(PROC_SCHEDSTAT_FILE_SLUG);
        let stat_reader: RefCell<ProcFSPIDStatReader> =
            RefCell::new(ProcFSPIDStatReader::new(&pid_stat_path)?);
        let status_reader: RefCell<ProcFSPIDStatusReader> =
            RefCell::new(ProcFSPIDStatusReader::new(&pid_status_path)?);
        let schedstat_reader: RefCell<ProcFSPIDSchedStatReader> =
            RefCell::new(ProcFSPIDSchedStatReader::new(&pid_schedstat_path)?);
        Ok(Self {
            stat: stat_reader,
            status: status_reader,
            schedstat: schedstat_reader,
            cmdline: parse_procfs_pid_cmdline(pid)?,
            comm: parse_procfs_pid_comm(pid)?,
            cgroup: parse_procfs_pid_cgroup(pid)?,
            previous_timestamp: Cell::new(monotonic()),
        })
    }
}

// the below struct doesnt give any reference back
// its self contained, so we cannot distribute it safely
#[derive(Debug, Default)]
pub struct PidsProcFS {
    pub pids: HashMap<u64, PidReaders, KinsightHashBuild>,
}

impl PidsProcFS {
    pub fn new() -> Self {
        Self {
            pids: HashMap::with_hasher(KinsightHashBuild::default()),
        }
    }

    pub fn add(&self, pid: u64) -> bool {
        // here if error occurs the pid have vanished from memory
        // before we catch it, so no need to worry just return false
        let pinned = self.pids.pin();
        match PidReaders::new(pid) {
            Ok(reader) => {
                pinned.insert(pid, reader);
                return true;
            }
            Err(_) => false,
        }
    }

    pub fn remove(&self, pid: u64) -> bool {
        let pinned = self.pids.pin();
        match pinned.remove(&pid) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn broadcast_metrics(
        &self,
        message_pool: &MessagePool<CpuPIDMetrics>,
        transmitter: &Sender<Arc<RwLock<CpuPIDMetrics>>>,
        stat_config: &StdHashMap<usize, bool>,
        status_config: &StdHashMap<&'static str, (usize, bool)>,
        schedstat_config: &StdHashMap<usize, bool>,
    ) -> Result<()> {
        let mut pids_to_remove = Vec::new();
        {
            let pinned = self.pids.pin();
            for (pid, pid_reader) in pinned.iter() {
                let Some(slot) = message_pool.acquire() else {
                    // Every pool slot is still held by the consumer or queued in
                    // the channel. This sample is dropped.
                    continue;
                };
                let populated = match slot.write() {
                    Ok(mut message) => self.gather_pid_metrics(
                        pid_reader,
                        &mut message,
                        stat_config,
                        status_config,
                        schedstat_config,
                    ),

                    Err(_) => {
                        return Err(ProcParserError::ParseError(
                            "PID metric message-pool lock was poisoned".into(),
                        ));
                    }
                };
                if !populated {
                    pids_to_remove.push(*pid);
                    continue;
                }
                // The write guard was dropped at the end of the match above.
                // The consumer now temporarily owns the second Arc reference.
                if transmitter.send(slot).is_err() {
                    return Err(ProcParserError::ThreadSpawningError(
                        "PID metrics receiver disconnected".to_string(),
                    ));
                }
            }
        }
        for pid in pids_to_remove {
            self.remove(pid);
        }
        Ok(())
    }

    pub fn gather_pid_metrics(
        &self,
        pid_reader: &PidReaders,
        message: &mut CpuPIDMetrics,
        stat_config: &StdHashMap<usize, bool>,
        status_config: &StdHashMap<&'static str, (usize, bool)>,
        schedstat_config: &StdHashMap<usize, bool>,
    ) -> bool {
        let stat_read_status = {
            let mut reader = pid_reader.stat.borrow_mut();
            reader.read_and_parse(stat_config)
        };

        if !stat_read_status {
            return false;
        }

        let status_read_status = {
            let mut reader = pid_reader.status.borrow_mut();
            reader.read_and_parse(status_config)
        };

        if !status_read_status {
            return false;
        }

        let schedstat_read_status = {
            let mut reader = pid_reader.schedstat.borrow_mut();

            reader.read_and_parse(schedstat_config)
        };

        if !schedstat_read_status {
            return false;
        }

        let current_time = monotonic();
        let previous_time = pid_reader.previous_timestamp.replace(current_time);

        message.time = current_time;
        message.time_delta = (current_time - previous_time).max(0.0);

        // Reuse String allocations retained in the pool slot.
        message.cgroup.clear();
        message.cgroup.push_str(&pid_reader.cgroup);

        message.comm.clear();
        message.comm.push_str(&pid_reader.comm);

        message.cmdline.clear();
        message.cmdline.push_str(&pid_reader.cmdline);

        {
            let stat_reader = pid_reader.stat.borrow();
            let values = stat_reader.values();

            message.pid = values.pid as u64;
            message.ppid = values.ppid as u64;
            message.process_state = values.state;

            message.user_cpu_time = values.utime;
            message.system_cpu_time = values.stime;

            message.total_cpu_time = message
                .user_cpu_time
                .saturating_add(message.system_cpu_time);

            message.num_threads = values.num_threads;
            message.minor_page_fault = values.minflt;
            message.major_page_fault = values.majflt;
            message.last_cpu_used = values.processor;
            message.scheduling_policy = values.policy;
            message.process_start_time = values.starttime;
        }

        {
            let status_reader = pid_reader.status.borrow();
            let values = status_reader.values();

            message.voluntary_ctxt = values.voluntary_ctxt_switches;

            message.non_voluntary_ctxt = values.nonvoluntary_ctxt_switches;

            message.total_program_size = values.vm_size;
            message.resident_set_size = values.vm_rss;

            message.shared_data = values.rss_file.saturating_add(values.rss_shmem);

            message.cpu_affinity = values.cpus_allowed_list;
        }

        {
            let schedstat_reader = pid_reader.schedstat.borrow();

            let values = schedstat_reader.values();

            message.scheduling_runtime = values.time_on_cpu;

            message.runqueue_wait_time = values.time_waiting;

            message.timeslices = values.timeslices_run_count;
        }

        true
    }
}
