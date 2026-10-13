use super::pidsfs_utils::{PIDActions, ProcFSPID};
use super::procfs_sensors::start_pids_observer;

use crate::configs::config::Config;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug)]
struct PIDSRoundRobin {
    threads: Vec<HashSet<ProcFSPID>>,
}

impl PIDSRoundRobin {
    fn new(thread_pool_count: usize) -> Self {
        let mut threads = Vec::with_capacity(thread_pool_count);
        threads.resize_with(thread_pool_count, HashSet::new);
        Self { threads }
    }

    fn add(&mut self, pid: ProcFSPID) -> bool {
        // Ensure that a PID belongs to at most one worker.
        if self.threads.iter().any(|thread| thread.contains(&pid)) {
            return false;
        }

        let Some(thread) = self.threads.iter_mut().min_by_key(|thread| thread.len()) else {
            // The thread pool is empty.
            return false;
        };

        thread.insert(pid)
    }

    fn remove(&mut self, pid: u64) -> Option<ProcFSPID> {
        for thread in &mut self.threads {
            if let Some(removed_pid) = thread.take(&pid) {
                return Some(removed_pid);
            }
        }
        None
    }
}

pub fn orchestrate_pids(
    config: &'static Config,
    signaller: Arc<AtomicBool>,
) -> Result<(Vec<JoinHandle<()>>, Sender<PIDActions>), Box<dyn Error>> {
    let (transmitter, receiver): (Sender<PIDActions>, Receiver<PIDActions>) =
        channel::<PIDActions>();
    let transmitter_clone = transmitter.clone();
    let signaller_clone = Arc::clone(&signaller);
    let observers = start_pids_observer(transmitter_clone, &config, signaller_clone);
    let mut observers = match observers {
        Ok(observer) => observer,
        Err(err) => panic!("Error occured {}", err),
    };
    let thread_pool = usize::try_from(config.thread_pool_count)?;
    let data = Arc::new(RwLock::new(PIDSRoundRobin::new(thread_pool)));
    // now lets add the pid to the round robin
    let reciever_worker = thread::spawn(move || {
        while !signaller.load(Ordering::Relaxed) {
            let possible_msg = receiver.recv_timeout(Duration::from_secs(1));
            match possible_msg {
                Ok(msg) => match msg {
                    PIDActions::Add(pid) => {
                        println!("Adding PID {:?}", pid);
                        let mut round_robin = data.write().unwrap();
                        round_robin.add(pid);
                    }
                    PIDActions::Remove(pid) => {
                        let mut round_robin = data.write().unwrap();
                        match round_robin.remove(pid) {
                            Some(removed_pid) => {
                                println!("Removing PID {:?}", removed_pid);
                            }
                            None => {
                                eprintln!("PID {pid} was not assigned to any worker");
                            }
                        }
                    }
                },
                Err(_) => (),
            }
        }
    });
    observers.push(reciever_worker);
    Ok((observers, transmitter))
}
