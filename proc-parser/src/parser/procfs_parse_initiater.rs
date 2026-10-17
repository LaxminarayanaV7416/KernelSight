use super::pidsfs_utils::PIDActions;
use super::procfs_pid_parser::orchestrate_pids;
use crate::common::message_passing_utils::initiate_mpsc_channel;
use crate::common::system_call_utils::monotonic;
use crate::configs::config::Config;
use crate::parser::procfs_parser::record_procfs;
use crate::writer::writer::start_writer;
use crate::writer::writer_utils::WriteDataMsg;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

pub fn initiate_procfs_parsing(
    config: &'static Config,
    signaller: Arc<AtomicBool>,
) -> Result<Vec<JoinHandle<()>>, Box<dyn Error>> {
    // record the starting time as constant
    let start_time: &'static i64 = Box::leak(Box::new(monotonic()));
    let (transmitter_pids, receiver_pids): (Sender<PIDActions>, Receiver<PIDActions>) =
        initiate_mpsc_channel::<PIDActions>();

    let (transmitter_writer, receiver_writer): (Sender<WriteDataMsg>, Receiver<WriteDataMsg>) =
        initiate_mpsc_channel::<WriteDataMsg>();

    let mut threads =
        match record_procfs(&config, Arc::clone(&signaller), transmitter_writer.clone()) {
            Ok(threads) => threads,
            Err(err) => panic!("Error occured starting the procfs {}", err),
        };

    let writer_thread = start_writer(&config, Arc::clone(&signaller), receiver_writer);

    let mut pids_threads = match orchestrate_pids(
        &config,
        Arc::clone(&signaller),
        start_time,
        transmitter_pids.clone(),
        receiver_pids,
        transmitter_writer.clone(),
    ) {
        Ok(threads) => threads,
        Err(err) => {
            panic!("Error occurred while starting procfs PIDs: {err}")
        }
    };

    threads.append(&mut pids_threads);
    threads.push(writer_thread);

    Ok(threads)
}
