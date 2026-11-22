use super::super::kinsight_utils::{KinsightHashBuild, PROCFS_PIDS_PARSER_THREAD_NAME};
use kinsight_common::ProcParserError;
use kinsight_common::Result;
use kinsight_common::procfs_constants::PROC_FS_ROOT_PATH;
use kinsight_writer::PIDActions;
use std::collections::HashSet;
use std::fs::read_dir;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn procfs_scrapper_sensor(
    transmitter: Sender<PIDActions>,
    global_heart_beat: &'static u64,
    signaller: Arc<AtomicBool>, // dont clone we already cloned this sent it over here
) -> Result<JoinHandle<()>> {
    let mut prev_scrapper_state: HashSet<u64, KinsightHashBuild> = HashSet::default();
    let mut current_scrapper_state: HashSet<u64, KinsightHashBuild> = HashSet::default();
    let thread_builder = thread::Builder::new().name(PROCFS_PIDS_PARSER_THREAD_NAME.to_string());
    let thread_handler = thread_builder
        .spawn(move || {
            while !signaller.load(Ordering::Relaxed) {
                for entry in read_dir(PROC_FS_ROOT_PATH).into_iter().flatten().flatten() {
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
                // println!("Sensor PIDS: {}", current_scrapper_state.len());
                for added_items in current_scrapper_state.difference(&prev_scrapper_state) {
                    let add_msg = PIDActions::Add(*added_items);
                    transmitter.send(add_msg).unwrap();
                }
                // ===============================================================|
                // UPDATE: We are not going to delete them, they will be vanished |
                // as long the handle is removed by the kernel;                   |
                // ===============================================================|
                // for removed_item in prev_scrapper_state.difference(&current_scrapper_state) {
                //     let remove_msg = PIDActions::Remove(*removed_item);
                //     transmitter.send(remove_msg).unwrap();
                // }
                std::mem::swap(&mut prev_scrapper_state, &mut current_scrapper_state);
                current_scrapper_state.clear();
                thread::sleep(Duration::from_millis(*global_heart_beat));
            }
        })
        .map_err(|_| {
            ProcParserError::ThreadSpawningError(
                "{PROCFS_PIDS_PARSER_THREAD_NAME} failed with IO Error".to_string(),
            )
        });
    thread_handler
}
