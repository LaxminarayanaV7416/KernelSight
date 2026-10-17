use super::pidsfs_utils::{PIDActions, ProcFSPID, ProcFSPIDData};
use super::procfs_sensors::start_pids_observer;
use crate::configs::config::{Config, ProcFSConfigTrait};
use crate::writer::writer_utils::WriteDataMsg;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug)]
struct PIDSRoundRobin {
    threads: HashSet<ProcFSPID>,
}

impl PIDSRoundRobin {
    fn new() -> Self {
        let threads = HashSet::new();
        Self { threads }
    }

    fn add(&mut self, pid: ProcFSPID) -> bool {
        // Ensure that a PID belongs to at most one worker.
        if self.threads.contains(&pid) {
            return false;
        }
        self.threads.insert(pid)
    }

    fn remove(&mut self, pid: u64) -> Option<ProcFSPID> {
        if let Some(removed_pid) = self.threads.take(&pid) {
            return Some(removed_pid);
        }
        None
    }
}

pub struct ProcFSRoundRobinData {
    pub pids_data: HashMap<u64, ProcFSPIDData>,
}

impl ProcFSRoundRobinData {
    pub fn new() -> Self {
        Self {
            pids_data: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        pid: u64,
        config: &'static Config,
        transmitter_writer: &Sender<WriteDataMsg>,
    ) {
        self.pids_data
            .entry(pid)
            .or_insert_with(|| ProcFSPIDData::new(pid, config, &transmitter_writer));
    }

    pub fn remove(&mut self, pid: u64) -> Option<ProcFSPIDData> {
        self.pids_data.remove(&pid)
    }
}

fn start_tracking_pids(
    round_robin: &Arc<RwLock<PIDSRoundRobin>>,
    data: &mut ProcFSRoundRobinData,
    config: &'static Config,
    transmitter_writer: &Sender<WriteDataMsg>,
    start_time: &'static i64,
    debug: &bool,
    schedstat_map: &HashMap<usize, bool>,
    stat_map: &HashMap<usize, bool>,
    statm_map: &HashMap<usize, bool>,
    status_map: &HashMap<&'static str, (usize, bool)>,
    schedstat_enabled: &bool,
    stat_enabled: &bool,
    statm_enabled: &bool,
    status_enabled: &bool,
) {
    // Take a snapshot of the PIDs currently assigned to workers.
    //
    // Do not hold the round-robin lock while reading /proc files because
    // filesystem reads could block add/remove events.
    let active_pids: HashSet<u64> = {
        let round_robin = match round_robin.read() {
            Ok(round_robin) => round_robin,
            Err(error) => {
                eprintln!("PID round-robin lock was poisoned: {error}");
                return;
            }
        };

        round_robin.threads.iter().map(|pid| pid.pid).collect()
    };

    // Remove reader state for PIDs that are no longer assigned to any worker.
    data.pids_data.retain(|pid, _| active_pids.contains(pid));

    for pid in active_pids {
        // The receiver worker adds PID identities to the round-robin.
        // The parser worker owns and initializes the mutable file readers.
        if !data.pids_data.contains_key(&pid) {
            data.add(pid, config, &transmitter_writer);
        }

        // Limit this mutable borrow to its own scope so the entry can be
        // removed afterward when read_and_parse returns false.
        let keep_reader = {
            let Some(record) = data.pids_data.get_mut(&pid) else {
                continue;
            };

            record.read_and_parse(
                debug,
                start_time,
                &transmitter_writer,
                schedstat_map,
                stat_map,
                statm_map,
                status_map,
                schedstat_enabled,
                stat_enabled,
                statm_enabled,
                status_enabled,
            )
        };

        if !keep_reader {
            data.remove(pid);
        }
    }
}

fn round_robin_thread(
    receiver_round_robin: Arc<RwLock<PIDSRoundRobin>>,
    receiver_signaller: Arc<AtomicBool>,
    receiver: Receiver<PIDActions>,
    config: &'static Config,
) -> JoinHandle<()> {
    thread::spawn(move || {
        while !receiver_signaller.load(Ordering::Relaxed) {
            match receiver.recv_timeout(Duration::from_secs(1)) {
                Ok(PIDActions::Add(pid)) => {
                    let mut round_robin = match receiver_round_robin.write() {
                        Ok(round_robin) => round_robin,
                        Err(error) => {
                            eprintln!("PID round-robin lock was poisoned: {error}");
                            return;
                        }
                    };

                    if !round_robin.add(pid) {
                        eprintln!(
                            "PID was not added because it already exists \
                             or the worker pool is empty"
                        );
                    }
                }

                Ok(PIDActions::Remove(pid)) => {
                    let mut round_robin = match receiver_round_robin.write() {
                        Ok(round_robin) => round_robin,
                        Err(error) => {
                            eprintln!("PID round-robin lock was poisoned: {error}");
                            return;
                        }
                    };

                    match round_robin.remove(pid) {
                        Some(removed_pid) => {
                            if config.debug {
                                println!("Removing PID {:?}", removed_pid);
                            }
                        }
                        None => {
                            if config.debug {
                                eprintln!("PID {pid} was not assigned to any worker");
                            }
                        }
                    }
                }

                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }
    })
}

pub fn orchestrate_pids(
    config: &'static Config,
    signaller: Arc<AtomicBool>,
    start_time: &'static i64,
    transmitter: Sender<PIDActions>,
    receiver: Receiver<PIDActions>,
    transmitter_writer: Sender<WriteDataMsg>,
) -> Result<Vec<JoinHandle<()>>, Box<dyn Error>> {
    let signaller_clone = Arc::clone(&signaller);
    let mut observers = match start_pids_observer(transmitter, &config, signaller_clone) {
        Ok(observer) => observer,
        Err(err) => panic!("Error occured {}", err),
    };
    let round_robin_pids = Arc::new(RwLock::new(PIDSRoundRobin::new()));

    let receiver_worker = round_robin_thread(
        Arc::clone(&round_robin_pids),
        Arc::clone(&signaller),
        receiver,
        &config,
    );

    let parser_round_robin = Arc::clone(&round_robin_pids);
    let parser_signaller = Arc::clone(&signaller);
    let mut round_robin_data = ProcFSRoundRobinData::new();
    let global_heart_beat = config.heart_beat;
    let debug = config.debug;
    let schedstat_filter_map = config.procfs.pid.schedstat.get_hashmap();
    let stat_filter_map = config.procfs.pid.stat.get_hashmap();
    let statm_filter_map = config.procfs.pid.statm.get_hashmap();
    // Status is a key-based file, so it needs the string-to-field map.
    let status_filter_map = config.procfs.pid.status.get_field_string_to_struct_ids();
    let is_schedstat_allowed = config.procfs.pid.schedstat.is_enabled();
    let is_stat_allowed = config.procfs.pid.stat.is_enabled();
    let is_statm_allowed = config.procfs.pid.statm.is_enabled();
    let is_status_allowed = config.procfs.pid.status.is_enabled();
    let parser_worker = thread::spawn(move || {
        while !parser_signaller.load(Ordering::Relaxed) {
            start_tracking_pids(
                &parser_round_robin,
                &mut round_robin_data,
                config,
                &transmitter_writer,
                start_time,
                &debug,
                &schedstat_filter_map,
                &stat_filter_map,
                &statm_filter_map,
                &status_filter_map,
                &is_schedstat_allowed,
                &is_stat_allowed,
                &is_statm_allowed,
                &is_status_allowed,
            );

            thread::sleep(Duration::from_millis(global_heart_beat));
        }
    });

    // Decision made, linux does allocate the PIDS sequentially and after few mins of break
    // if the submission of processes are like somany within a second, then its 1 hour
    // so we can safely use the previous current approach to remove and keep the tasks
    // this avoids the race condition and make sure the round_robin data is mutablly shared
    // across the threads.
    observers.push(receiver_worker);
    observers.push(parser_worker);
    Ok(observers)
}
