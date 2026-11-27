use super::kinsight_utils::PROCFS_ROOT_PARSER_THREAD_NAME;
use super::procfs_pid_parser::PidsProcFS;
use super::utils::calcualte_diff_delta;
use super::utils::get_cpu_usage;
use kinsight_common::KinsightDataParserTrait;
use kinsight_common::ProcParserError;
use kinsight_common::Result;
use kinsight_common::config::{Config, ProcFSConfigTrait};
use kinsight_common::monotonic;
use kinsight_common::procfs_constants::{
    PROC_LOADAVG_FILE_PATH, PROC_MEMINFO_FILE_PATH, PROC_NET_DEV_FILE_PATH,
    PROC_PRESSURE_CPU_FILE_PATH, PROC_PRESSURE_MEMORY_FILE_PATH, PROC_STAT_FILE_PATH,
    PROC_VMSTAT_FILE_PATH,
};
use kinsight_proc_parser::ProcFSLoadAvgReader;
use kinsight_proc_parser::ProcFSMemInfoReader;
use kinsight_proc_parser::ProcFSNetDevReader;
use kinsight_proc_parser::ProcFSStatReader;
use kinsight_proc_parser::ProcFSVmStatReader;
use kinsight_proc_parser::ProcProcessGeneralReader;
use kinsight_writer::MessagePool;
use kinsight_writer::NetworkMetrics;
use kinsight_writer::SystemWideMetrics;
use std::cell::RefCell;
use std::mem::swap;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

/*
 * Lets get some calculations laid down
 * System Wide CPU and Scheduler metrics
 * *************************************
 * CPU Utilization per core and total from /proc/stat (Done)
 * User / System CPU time
 * ***** User seconds - /proc/stat cpu.user + cpu.nice / cpu hz
 * ***** System seconds - /proc/stat cpu.system / cpu hz
 * IO Wait time /proc/stat - iowait/ cpu hz
 * CPU Steal
 * Runnable thread count - /proc/stat procs_running
 * I/O-blocked task count - /proc/stat procs_blocked
 */

pub fn procfs_metrics_collector(
    global_heart_rate: &'static u64,
    signaller: Arc<AtomicBool>, // dont clone we already cloned this sent it over here
    config: &LazyLock<Box<Config>>,
) -> Result<JoinHandle<()>> {
    let thread_builder = thread::Builder::new().name(PROCFS_ROOT_PARSER_THREAD_NAME.to_string());
    // this doesnt contain any hashmap for the pid tracking

    // we are wrapping the readers in refcell because of the
    // interior mutability features which is most useful
    // since we are actually get mutable borrow when we are reading
    // file path we are passing here.
    let proc_stat_reader: RefCell<ProcFSStatReader> =
        RefCell::new(ProcFSStatReader::new(&PROC_STAT_FILE_PATH)?);
    let proc_meminfo_reader: RefCell<ProcFSMemInfoReader> =
        RefCell::new(ProcFSMemInfoReader::new(&PROC_MEMINFO_FILE_PATH)?);
    let proc_loadavg_reader: RefCell<ProcFSLoadAvgReader> =
        RefCell::new(ProcFSLoadAvgReader::new(&PROC_LOADAVG_FILE_PATH)?);
    let proc_vmstat_reader: RefCell<ProcFSVmStatReader> =
        RefCell::new(ProcFSVmStatReader::new(&PROC_VMSTAT_FILE_PATH)?);
    let proc_pressure_cpu_reader: RefCell<ProcProcessGeneralReader> =
        RefCell::new(ProcProcessGeneralReader::new(&PROC_PRESSURE_CPU_FILE_PATH)?);
    let proc_pressure_memory_reader: RefCell<ProcProcessGeneralReader> = RefCell::new(
        ProcProcessGeneralReader::new(&PROC_PRESSURE_MEMORY_FILE_PATH)?,
    );
    let proc_net_dev_reader: RefCell<ProcFSNetDevReader> =
        RefCell::new(ProcFSNetDevReader::new(&PROC_NET_DEV_FILE_PATH)?);

    let proc_stat_filter_map = config.procfs.stat.get_hashmap();
    let proc_meminfo_filter_map = Box::new(config.procfs.meminfo.get_field_string_to_struct_ids());
    let proc_loadavg_filter_map = config.procfs.loadavg.get_hashmap();
    let proc_vmstat_filter_map = config.procfs.vmstat.get_hashmap();
    let proc_pressure_cpu_filter_map = config.procfs.pressure.cpu.get_hashmap();
    let proc_pressure_memory_filter_map = config.procfs.pressure.memory.get_hashmap();
    let proc_net_dev_filter_map = config.procfs.net_dev.get_hashmap();

    let mut system_wide_metrics = Box::new(SystemWideMetrics::default());
    // TODO: for now capacity is hardcoded
    let system_metrics_msg_pool: MessagePool<SystemWideMetrics> = MessagePool::new(10);

    let mut pids = PidsProcFS::new();

    let mut prev_time = monotonic();
    let thread_handler = thread_builder
        .spawn(move || {
            let mut has_previous_sample = false;
            while !signaller.load(Ordering::Relaxed) {
                let mut current_time = monotonic();
                system_wide_metrics.time = current_time;
                system_wide_metrics.time_delta = current_time - prev_time;

                // replace the current time with prev time
                swap(&mut prev_time, &mut current_time);

                let stat_read_status = {
                    let mut reader = proc_stat_reader.borrow_mut();
                    reader.read_and_parse(&proc_stat_filter_map)
                };

                let meminfo_read_status = {
                    let mut reader = proc_meminfo_reader.borrow_mut();
                    reader.read_and_parse(&proc_meminfo_filter_map)
                };

                let loadavg_read_status = {
                    let mut reader = proc_loadavg_reader.borrow_mut();
                    reader.read_and_parse(&proc_loadavg_filter_map)
                };

                let vmstat_read_status = {
                    let mut reader = proc_vmstat_reader.borrow_mut();
                    reader.read_and_parse(&proc_vmstat_filter_map)
                };

                let pressure_cpu_read_status = {
                    let mut reader = proc_pressure_cpu_reader.borrow_mut();
                    reader.read_and_parse(&proc_pressure_cpu_filter_map)
                };

                let pressure_memory_read_status = {
                    let mut reader = proc_pressure_memory_reader.borrow_mut();
                    reader.read_and_parse(&proc_pressure_memory_filter_map)
                };

                let proc_net_dev_read_status = {
                    let mut reader = proc_net_dev_reader.borrow_mut();
                    reader.read_and_parse(&proc_net_dev_filter_map)
                };

                if has_previous_sample {
                    if stat_read_status {
                        let reader = proc_stat_reader.borrow();
                        let pressure_reader = proc_pressure_cpu_reader.borrow();

                        let current = reader.values();
                        let previous = reader.prev_values();

                        if pressure_cpu_read_status {
                            let current = pressure_reader.values();
                            let previous = pressure_reader.prev_values();
                            let elapsed = system_wide_metrics.time_delta;
                            if elapsed > 0.0 {
                                let stalled_microseconds =
                                    current.some_total.saturating_sub(previous.some_total);
                                system_wide_metrics.cpu_metrics.cpu_pressure =
                                    100.0 * stalled_microseconds as f64 / (elapsed * 1_000_000.0);
                            }
                        }

                        // some direct assignments
                        system_wide_metrics.cpu_metrics.ctxt_per_time_delta = calcualte_diff_delta(
                            current.ctxt,
                            previous.ctxt,
                            system_wide_metrics.time_delta,
                        );
                        system_wide_metrics.cpu_metrics.total_ctxt = current.ctxt;
                        system_wide_metrics.cpu_metrics.processes_running =
                            current.processes_running;
                        system_wide_metrics.cpu_metrics.io_blocked_task_count =
                            current.processes_blocked;
                        system_wide_metrics.cpu_metrics.softirq_time_delta = calcualte_diff_delta(
                            current.softirq,
                            previous.softirq,
                            system_wide_metrics.time_delta,
                        );

                        match get_cpu_usage(&current.cpu, &previous.cpu) {
                            Some(cpu_usage) => {
                                system_wide_metrics.cpu_metrics.cpu_utilization_percentage =
                                    cpu_usage;
                            }
                            None => println!("Error its an empty value"),
                        }

                        system_wide_metrics
                            .cpu_metrics
                            .cpu_core_utilization_per_core
                            .clear();
                        for (curr_cpu, prev_cpu) in
                            current.cpu_cores.iter().zip(&previous.cpu_cores)
                        {
                            match get_cpu_usage(&curr_cpu, &prev_cpu) {
                                Some(cpu_usage) => {
                                    system_wide_metrics
                                        .cpu_metrics
                                        .cpu_core_utilization_per_core
                                        .push(cpu_usage);
                                }
                                None => println!("Error its an empty value"),
                            }
                        }
                    }

                    // /proc/loadavg -- for thread info
                    if loadavg_read_status {
                        let reader = proc_loadavg_reader.borrow();
                        let values = reader.values();
                        system_wide_metrics.cpu_metrics.total_thread_count = values.num_total;
                        system_wide_metrics.cpu_metrics.total_thread_running = values.num_runnable;
                    }

                    // /proc/meminfo
                    if meminfo_read_status {
                        let reader = proc_meminfo_reader.borrow();
                        let values = reader.values();

                        system_wide_metrics.memory_metrics.total_ram = values.mem_total;
                        system_wide_metrics.memory_metrics.available_ram = values.mem_available;
                        system_wide_metrics.memory_metrics.ram_used =
                            values.mem_total.saturating_sub(values.mem_available);
                        system_wide_metrics.memory_metrics.ram_usage_percentage =
                            if values.mem_total > 0 {
                                100.0 * system_wide_metrics.memory_metrics.ram_used as f64
                                    / values.mem_total as f64
                            } else {
                                0.0
                            };
                        system_wide_metrics.memory_metrics.free_ram = values.mem_free;
                        system_wide_metrics.memory_metrics.page_cache = values.mem_cached;
                        system_wide_metrics.memory_metrics.buffer = values.mem_buffers;
                        system_wide_metrics.memory_metrics.anonymous_memory = values.mem_anon_pages;
                        system_wide_metrics.memory_metrics.shared_memory = values.mem_shmem;
                        system_wide_metrics.memory_metrics.slab_memory = values.mem_slab;
                        system_wide_metrics.memory_metrics.reclaimable_slab =
                            values.mem_s_reclaimable;
                        system_wide_metrics.memory_metrics.unreclaimable_slab =
                            values.mem_s_unreclaim;
                        system_wide_metrics.memory_metrics.kernel_stack = values.mem_kernel_stack;
                        system_wide_metrics.memory_metrics.page_tables = values.mem_page_tables;
                        system_wide_metrics.memory_metrics.swap_used =
                            values.mem_swap_total.saturating_sub(values.mem_swap_free);
                    }

                    if vmstat_read_status {
                        let reader = proc_vmstat_reader.borrow();

                        let current = reader.values();
                        let previous = reader.prev_values();

                        let elapsed = system_wide_metrics.time_delta;
                        if elapsed > 0.0 {
                            system_wide_metrics.memory_metrics.page_fault_time_delta =
                                current.pgfault.saturating_sub(previous.pgfault) as f64 / elapsed;
                            system_wide_metrics
                                .memory_metrics
                                .page_major_fault_time_delta =
                                current.pgmajfault.saturating_sub(previous.pgmajfault) as f64
                                    / elapsed;
                            system_wide_metrics.memory_metrics.oom_kill_time_delta =
                                current.oom_kill.saturating_sub(previous.oom_kill) as f64 / elapsed;
                        }
                    }

                    if pressure_memory_read_status {
                        let reader = proc_pressure_memory_reader.borrow();

                        let current = reader.values();
                        let previous = reader.prev_values();

                        let elapsed = system_wide_metrics.time_delta;

                        if elapsed > 0.0 {
                            let stalled_microseconds =
                                current.some_total.saturating_sub(previous.some_total);

                            system_wide_metrics.memory_metrics.memory_pressure =
                                100.0 * stalled_microseconds as f64 / (elapsed * 1_000_000.0);
                        }
                    }

                    if proc_net_dev_read_status {
                        let reader = proc_net_dev_reader.borrow();

                        let current = reader.values();
                        let previous = reader.prev_values();

                        for (name, current) in &current.interfaces {
                            let previous = previous.interfaces.get(name);

                            if !system_wide_metrics
                                .network_metrics
                                .contains_key(name.as_str())
                            {
                                system_wide_metrics
                                    .network_metrics
                                    .insert(name.clone(), NetworkMetrics::default());
                            }

                            let metrics = system_wide_metrics
                                .network_metrics
                                .get_mut(name.as_str())
                                .expect("interface exists");

                            metrics.total_download_bytes = current.total_download_bytes;
                            metrics.total_donwload_packets = current.total_donwload_packets;
                            metrics.bad_packets_downloaded = current.bad_packets_downloaded;
                            metrics.recieved_packets_dropped = current.recieved_packets_dropped;
                            metrics.total_upload_bytes = current.total_upload_bytes;
                            metrics.total_upload_packets = current.total_upload_packets;
                            metrics.bad_packets_uploaded = current.bad_packets_uploaded;
                            metrics.sent_packets_dropped = current.sent_packets_dropped;

                            if let Some(previous) = previous {
                                let elapsed = system_wide_metrics.time_delta;

                                if elapsed > 0.0 {
                                    metrics.rate_download_bytes = current
                                        .total_download_bytes
                                        .saturating_sub(previous.total_download_bytes)
                                        as f64
                                        / elapsed;

                                    metrics.rate_upload_bytes = current
                                        .total_upload_bytes
                                        .saturating_sub(previous.total_upload_bytes)
                                        as f64
                                        / elapsed;

                                    metrics.rate_donwload_packets = current
                                        .total_donwload_packets
                                        .saturating_sub(previous.total_donwload_packets)
                                        as f64
                                        / elapsed;

                                    metrics.rate_upload_packets = current
                                        .total_upload_packets
                                        .saturating_sub(previous.total_upload_packets)
                                        as f64
                                        / elapsed;
                                }
                            }
                        }
                    }

                    println!("{:?}", system_wide_metrics);
                } else {
                    // CPU usage is a delta between two samples. The first
                    // successful read only initializes the baseline.
                    has_previous_sample = true;
                    prev_time = monotonic();
                }

                // Sleep after both successful and failed reads.
                thread::sleep(Duration::from_millis(*global_heart_rate));
            }
        })
        .map_err(|error| {
            ProcParserError::ThreadSpawningError(format!(
                "{PROCFS_ROOT_PARSER_THREAD_NAME} failed: {error}"
            ))
        });

    thread_handler
}
