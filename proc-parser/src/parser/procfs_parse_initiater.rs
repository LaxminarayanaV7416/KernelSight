use super::cgroups_v2_orchestrator::{CgroupsV2Actions, cgroup_initiater};
use super::pidsfs_utils::PIDActions;
use super::procfs_pid_parser::orchestrate_pids;
use crate::common::message_passing_utils::initiate_mpsc_channel;
use crate::common::parser_error::{ProcParserError, Result};
use crate::common::system_call_utils::monotonic;
use crate::configs::config::Config;
use crate::gpu::nvidia::NvidiaGPUMetrics;
use crate::parser::procfs_parser::record_procfs;
use crate::procfs::pidfs::cachable_reads::parse_procfs_pid_cgroup;
use crate::writer::writer::{start_cgroup_writer, start_gpu_writer, start_writer};
use crate::writer::writer_utils::WriteCgroupsDataMsg;
use crate::writer::writer_utils::WriteDataMsg;
use crate::writer::writer_utils::WriteGPUDataMsg;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Duration;

/*
 * Here is the pipeline overview of the what is happening
 * Sensors --PID's--> cgroupfileparser (recieves this and keeps the state of cgroup file buffers)
 * --Cgroup-->  cgroup_initiater (which sends the PIDS under cgroups.procs) --PID's-->
 * orchestrate_pids (here we again dont open the cgroups file as it sent along with that)
 * Rest GPU and Whole Kernel threads are running in there own space
 */

fn cgroup_file_parser(
    signaller: Arc<AtomicBool>,
    receiver: Receiver<PIDActions>,
    transmit_cgroup: Sender<CgroupsV2Actions>,
) -> Result<JoinHandle<()>> {
    // let define the hashmap that holds the pids and cgroup strings
    let mut pid_cgroup_map: HashMap<u64, String> = HashMap::new();

    let thread_builder = std::thread::Builder::new().name("CgroupFileParserThread".to_string());
    thread_builder
        .spawn(move || {
            while !signaller.load(Ordering::Relaxed) {
                match receiver.recv_timeout(Duration::from_secs(1)) {
                    Ok(PIDActions::Add(pid)) => {
                        // now we have to check if we have read the file or not
                        if !pid_cgroup_map.contains_key(&pid.pid) {
                            // now open the file and persist it in the map
                            let pid_cgroup = match parse_procfs_pid_cgroup(pid.pid as usize) {
                                Ok(cgroup_value) => cgroup_value,
                                Err(err) => {
                                    // the error is of type ProcParserError::FileReadError, so we continue so log and proceed
                                    eprintln!(
                                        "PROC_PID_CGROUP::Failed to read {} PID disappeared",
                                        err
                                    );
                                    // we dont need to add anything so just continue dont break the loop
                                    continue;
                                }
                            };
                            pid_cgroup_map.insert(pid.pid, pid_cgroup.clone());
                            // now send the cgroup to the cgroup reader such that it reads
                            transmit_cgroup.send(CgroupsV2Actions::Add(pid_cgroup));
                        }
                    }
                    Ok(PIDActions::Remove(pid)) => {
                        if pid_cgroup_map.contains_key(&pid) {
                            let pid_cgroup = match pid_cgroup_map.remove(&pid) {
                                Some(cgroup_value) => cgroup_value,
                                None => continue,
                            };
                            // now send the cgroup to the cgroup reader such that it reads
                            transmit_cgroup.send(CgroupsV2Actions::Remove(pid_cgroup));
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        break;
                    }
                }
            }
        })
        .map_err(|io_error| ProcParserError::ThreadSpawningError(io_error.to_string()))
}

pub fn initiate_procfs_parsing(
    config: &'static Config,
    signaller: Arc<AtomicBool>,
) -> Result<Vec<Result<JoinHandle<()>>>> {
    // record the starting time as constant
    let start_time: &'static f64 = Box::leak(Box::new(monotonic()));
    let mut threads: Vec<Result<JoinHandle<()>>> = Vec::new();

    // ===========================================================================
    // whole kernel proc scanner this is default and runs with 4 threads for now...
    let (transmitter_writer, receiver_writer): (Sender<WriteDataMsg>, Receiver<WriteDataMsg>) =
        initiate_mpsc_channel::<WriteDataMsg>();
    let mut threads_procs =
        match record_procfs(&config, Arc::clone(&signaller), transmitter_writer.clone()) {
            Ok(threads) => threads,
            Err(err) => panic!("Error occured starting the procfs {}", err),
        };
    let writer_thread = start_writer(&config, Arc::clone(&signaller), receiver_writer);
    threads.append(&mut threads_procs);
    threads.push(writer_thread);
    // ===========================================================================

    // this is the sensor triggering point, means sensors push this and cgroup
    // reciever recieves this and reads the cgroup file and decides based on the
    // read cgroup file.
    let (transmitter_sensor_pids_to_cgroup_parser, receiver_sensor_pids_to_cgroup_parser): (
        Sender<PIDActions>,
        Receiver<PIDActions>,
    ) = initiate_mpsc_channel::<PIDActions>();

    let (transmitter_cgroup_invoker, receiver_cgroup_invoker): (
        Sender<CgroupsV2Actions>,
        Receiver<CgroupsV2Actions>,
    ) = initiate_mpsc_channel::<CgroupsV2Actions>();

    let pid_to_cgroup_bridge_thread = cgroup_file_parser(
        Arc::clone(&signaller),
        receiver_sensor_pids_to_cgroup_parser,
        transmitter_cgroup_invoker,
    );
    threads.push(pid_to_cgroup_bridge_thread);

    let (transmitter_cgroup_writer, receiver_cgroup_writer): (
        Sender<WriteCgroupsDataMsg>,
        Receiver<WriteCgroupsDataMsg>,
    ) = initiate_mpsc_channel::<WriteCgroupsDataMsg>();

    let (transmitter_pids_from_cgroup, receiver_pids_from_cgroup): (
        Sender<PIDActions>,
        Receiver<PIDActions>,
    ) = initiate_mpsc_channel::<PIDActions>();

    let (cgroup_reciever_thread, cgroup_reader_thread) = cgroup_initiater(
        &config,
        Arc::clone(&signaller),
        receiver_cgroup_invoker,
        transmitter_pids_from_cgroup,
        transmitter_cgroup_writer,
    );

    let write_thread_cgroup =
        start_cgroup_writer(&config, Arc::clone(&signaller), receiver_cgroup_writer);

    let mut pids_threads = orchestrate_pids(
        &config,
        Arc::clone(&signaller),
        start_time,
        transmitter_sensor_pids_to_cgroup_parser.clone(),
        receiver_pids_from_cgroup,
        transmitter_writer.clone(),
    );
    threads.append(&mut pids_threads);
    threads.push(cgroup_reciever_thread);
    threads.push(cgroup_reader_thread);
    threads.push(write_thread_cgroup);

    // GPU Metrics Start from Here ================
    let (transmitter_gpu_writer, receiver_gpu_writer): (
        Sender<WriteGPUDataMsg>,
        Receiver<WriteGPUDataMsg>,
    ) = initiate_mpsc_channel::<WriteGPUDataMsg>();
    let mut nvidia = NvidiaGPUMetrics::new();
    let writer_thread_gpu = start_gpu_writer(&config, Arc::clone(&signaller), receiver_gpu_writer);
    let gpu_thread =
        nvidia.get_device_details(&config, &transmitter_gpu_writer, Arc::clone(&signaller));
    threads.push(writer_thread_gpu);
    threads.push(gpu_thread);
    // END of Gpu Metrics =========================

    Ok(threads)
}
