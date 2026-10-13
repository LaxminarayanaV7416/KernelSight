use super::utils::{PIDActions, ProcFSPID};
use crate::common::procfs_constants::PROC_FS_ROOT_PATH;
use crate::configs::config::Config;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

/*
 * Here I will write all functions that basically consumes
 * MPSC channels transmitter as input which monitors and
 * send them to the channel and there will be only one reciever
 * since rust compiler doesnt support Multi Reciever, this fits
 * our case as well, so we recieve the pid from that and then
 * spawn if process is not present.
 */

fn procfs_scrapper_sensor(
    transmitter: Sender<PIDActions>,
    config: &Config,
    signaller: Arc<AtomicBool>, // dont clone this we should clone and send this
) -> Result<JoinHandle<()>, Box<dyn Error>> {
    let mut prev_scrapper_state: HashSet<u64> = HashSet::new();
    let mut current_scrapper_state: HashSet<u64> = HashSet::new();
    let global_heart_beat = Box::new(config.heart_beat.clone());
    let thread_handler = thread::spawn(move || {
        while !signaller.load(Ordering::Relaxed) {
            for entry in fs::read_dir(PROC_FS_ROOT_PATH)
                .into_iter()
                .flatten()
                .flatten()
            {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let pid: Option<u64> = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .and_then(|s| s.parse::<u64>().ok());
                let pid_val = match pid {
                    Some(v) => v,
                    None => continue,
                };
                // check if the pid_value is present inside the hashset
                if !current_scrapper_state.contains(&pid_val) {
                    current_scrapper_state.insert(pid_val);
                }
            }

            for added_items in current_scrapper_state.difference(&prev_scrapper_state) {
                let add_msg = PIDActions::Add(ProcFSPID { pid: *added_items });
                transmitter.send(add_msg).unwrap();
            }
            for removed_item in prev_scrapper_state.difference(&current_scrapper_state) {
                let remove_msg = PIDActions::Remove(ProcFSPID { pid: *removed_item });
                transmitter.send(remove_msg).unwrap();
            }
            std::mem::swap(&mut prev_scrapper_state, &mut current_scrapper_state);
            current_scrapper_state.clear();
            thread::sleep(Duration::from_millis(*global_heart_beat));
        }
    });
    Ok(thread_handler)
}

pub fn start_pids_observer(
    transmitter: Sender<PIDActions>,
    config: &Config,
    signaller: Arc<AtomicBool>,
) -> Result<Vec<JoinHandle<()>>, Box<dyn Error>> {
    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    // let transmitter_clone = transmitter.clone();
    // let signaller_clone = Arc::clone(&signaller);
    // start all the observers here
    let folder_scrapper = procfs_scrapper_sensor(transmitter, &config, signaller).unwrap();
    threads.push(folder_scrapper);
    // dont forget to clone the transmitter and signaler as well, last one doesnot need any clone

    Ok(threads)
}
