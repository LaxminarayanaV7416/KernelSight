use super::cgroups_v2_orchestrator::{CgroupsV2Actions, cgroup_initiater};
use super::pidsfs_utils::PIDActions;
use super::procfs_pid_parser::orchestrate_pids;
use crate::common::message_passing_utils::initiate_mpsc_channel;
use crate::common::system_call_utils::monotonic;
use crate::configs::config::Config;
use crate::gpu::nvidia::NvidiaGPUMetrics;
use crate::parser::procfs_parser::record_procfs;
use crate::writer::writer::{start_cgroup_writer, start_gpu_writer, start_writer};
use crate::writer::writer_utils::WriteCgroupsDataMsg;
use crate::writer::writer_utils::WriteDataMsg;
use crate::writer::writer_utils::WriteGPUDataMsg;
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
    let start_time: &'static f64 = Box::leak(Box::new(monotonic()));
    let (transmitter_pids, receiver_pids): (Sender<PIDActions>, Receiver<PIDActions>) =
        initiate_mpsc_channel::<PIDActions>();

    let (transmitter_writer, receiver_writer): (Sender<WriteDataMsg>, Receiver<WriteDataMsg>) =
        initiate_mpsc_channel::<WriteDataMsg>();

    let (transmitter_gpu_writer, receiver_gpu_writer): (
        Sender<WriteGPUDataMsg>,
        Receiver<WriteGPUDataMsg>,
    ) = initiate_mpsc_channel::<WriteGPUDataMsg>();

    let (transmitter_cgroup_invoker, receiver_cgroup_invoker): (
        Sender<CgroupsV2Actions>,
        Receiver<CgroupsV2Actions>,
    ) = initiate_mpsc_channel::<CgroupsV2Actions>();

    let (transmitter_cgroup_writer, receiver_cgroup_writer): (
        Sender<WriteCgroupsDataMsg>,
        Receiver<WriteCgroupsDataMsg>,
    ) = initiate_mpsc_channel::<WriteCgroupsDataMsg>();

    let mut nvidia = NvidiaGPUMetrics::new();

    let mut threads =
        match record_procfs(&config, Arc::clone(&signaller), transmitter_writer.clone()) {
            Ok(threads) => threads,
            Err(err) => panic!("Error occured starting the procfs {}", err),
        };
    let (cgroup_reciever_thread, cgroup_reader_thread) = match cgroup_initiater(
        &config,
        Arc::clone(&signaller),
        receiver_cgroup_invoker,
        transmitter_cgroup_writer,
    ) {
        Ok(thread) => thread,
        Err(err) => panic!("Error occured starting Cgroup Reciever {}", err),
    };

    let writer_thread = start_writer(&config, Arc::clone(&signaller), receiver_writer);
    let writer_thread_gpu = start_gpu_writer(&config, Arc::clone(&signaller), receiver_gpu_writer);
    let write_thread_cgroup =
        start_cgroup_writer(&config, Arc::clone(&signaller), receiver_cgroup_writer);

    let mut pids_threads = match orchestrate_pids(
        &config,
        Arc::clone(&signaller),
        start_time,
        transmitter_pids.clone(),
        receiver_pids,
        transmitter_writer.clone(),
        transmitter_cgroup_invoker.clone(),
    ) {
        Ok(threads) => threads,
        Err(err) => {
            panic!("Error occurred while starting procfs PIDs: {err}")
        }
    };
    let gpu_thread =
        match nvidia.get_device_details(&config, &transmitter_gpu_writer, Arc::clone(&signaller)) {
            Ok(thread) => thread,
            Err(err) => panic!("Error occured while calling the Gpu send message {err}"),
        };

    threads.append(&mut pids_threads);
    threads.push(cgroup_reciever_thread);
    threads.push(cgroup_reader_thread);
    threads.push(writer_thread);
    threads.push(writer_thread_gpu);
    threads.push(gpu_thread);

    Ok(threads)
}
