use super::kinsight_utils::PROCFS_ROOT_PARSER_THREAD_NAME;
use super::utils::calcualte_diff_delta;
use super::utils::get_cpu_usage;
use kinsight_common::KinsightDataParserTrait;
use kinsight_common::ProcParserError;
use kinsight_common::Result;
use kinsight_common::config::{Config, ProcFSConfigTrait};
use kinsight_common::monotonic;
use kinsight_common::procfs_constants::{
    PROC_LOADAVG_FILE_PATH, PROC_MEMINFO_FILE_PATH, PROC_PRESSURE_CPU_FILE_PATH,
    PROC_PRESSURE_MEMORY_FILE_PATH, PROC_STAT_FILE_PATH, PROC_VMSTAT_FILE_PATH,
};
use kinsight_proc_parser::ProcFSLoadAvgReader;
use kinsight_proc_parser::ProcFSMemInfoReader;
use kinsight_proc_parser::ProcFSStatReader;
use kinsight_proc_parser::ProcFSVmStatReader;
use kinsight_proc_parser::ProcProcessGeneralReader;
use kinsight_writer::CpuSystemMetrics;
use kinsight_writer::MemorySystemMetrics;
use kinsight_writer::MessagePool;
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

    let proc_stat_filter_map = Box::new(config.procfs.stat.get_hashmap());
    let proc_meminfo_filter_map = Box::new(config.procfs.meminfo.get_field_string_to_struct_ids());
    let proc_loadavg_filter_map = Box::new(config.procfs.loadavg.get_hashmap());
    let proc_vmstat_filter_map = Box::new(config.procfs.vmstat.get_hashmap());
    let proc_pressure_cpu_filter_map = Box::new(config.procfs.pressure.cpu.get_hashmap());
    let proc_pressure_memory_filter_map = Box::new(config.procfs.pressure.memory.get_hashmap());

    let mut cpu_system_metrics = Box::new(CpuSystemMetrics::default());
    let mut memory_system_metrics = Box::new(MemorySystemMetrics::default());
    // TODO: for now capacity is hardcoded
    let cpu_system_metrics_msg_pool: MessagePool<Box<CpuSystemMetrics>> = MessagePool::new(10);
    let memory_system_metrics_msg_pool: MessagePool<Box<MemorySystemMetrics>> =
        MessagePool::new(10);

    let mut prev_time = monotonic();
    let thread_handler = thread_builder
        .spawn(move || {
            let mut has_previous_sample = false;
            while !signaller.load(Ordering::Relaxed) {
                let mut current_time = monotonic();
                cpu_system_metrics.time = current_time;
                cpu_system_metrics.time_delta = current_time - prev_time;
                memory_system_metrics.time = current_time;
                memory_system_metrics.time_delta = cpu_system_metrics.time_delta;

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

                if has_previous_sample {
                    if stat_read_status {
                        let reader = proc_stat_reader.borrow();
                        let pressure_reader = proc_pressure_cpu_reader.borrow();

                        let current = reader.values();
                        let previous = reader.prev_values();

                        if pressure_cpu_read_status {
                            let current = pressure_reader.values();
                            let previous = pressure_reader.prev_values();
                            let elapsed = cpu_system_metrics.time_delta;
                            if elapsed > 0.0 {
                                let stalled_microseconds =
                                    current.some_total.saturating_sub(previous.some_total);
                                cpu_system_metrics.cpu_pressure =
                                    100.0 * stalled_microseconds as f64 / (elapsed * 1_000_000.0);
                            }
                        }

                        // some direct assignments
                        cpu_system_metrics.ctxt_per_time_delta = calcualte_diff_delta(
                            current.ctxt,
                            previous.ctxt,
                            cpu_system_metrics.time_delta,
                        );
                        cpu_system_metrics.total_ctxt = current.ctxt;
                        cpu_system_metrics.processes_running = current.processes_running;
                        cpu_system_metrics.io_blocked_task_count = current.processes_blocked;
                        cpu_system_metrics.softirq_time_delta = calcualte_diff_delta(
                            current.softirq,
                            previous.softirq,
                            cpu_system_metrics.time_delta,
                        );

                        match get_cpu_usage(&current.cpu, &previous.cpu) {
                            Some(cpu_usage) => {
                                cpu_system_metrics.cpu_utilization_percentage = cpu_usage;
                            }
                            None => println!("Error its an empty value"),
                        }

                        cpu_system_metrics.cpu_core_utilization_per_core.clear();
                        for (curr_cpu, prev_cpu) in
                            current.cpu_cores.iter().zip(&previous.cpu_cores)
                        {
                            match get_cpu_usage(&curr_cpu, &prev_cpu) {
                                Some(cpu_usage) => {
                                    cpu_system_metrics
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
                        cpu_system_metrics.total_thread_count = values.num_total;
                        cpu_system_metrics.total_thread_running = values.num_runnable;
                    }

                    // /proc/meminfo
                    if meminfo_read_status {
                        let reader = proc_meminfo_reader.borrow();
                        let values = reader.values();

                        memory_system_metrics.total_ram = values.mem_total;
                        memory_system_metrics.available_ram = values.mem_available;
                        memory_system_metrics.ram_used =
                            values.mem_total.saturating_sub(values.mem_available);
                        memory_system_metrics.ram_usage_percentage = if values.mem_total > 0 {
                            100.0 * memory_system_metrics.ram_used as f64 / values.mem_total as f64
                        } else {
                            0.0
                        };
                        memory_system_metrics.free_ram = values.mem_free;
                        memory_system_metrics.page_cache = values.mem_cached;
                        memory_system_metrics.buffer = values.mem_buffers;
                        memory_system_metrics.anonymous_memory = values.mem_anon_pages;
                        memory_system_metrics.shared_memory = values.mem_shmem;
                        memory_system_metrics.slab_memory = values.mem_slab;
                        memory_system_metrics.reclaimable_slab = values.mem_s_reclaimable;
                        memory_system_metrics.unreclaimable_slab = values.mem_s_unreclaim;
                        memory_system_metrics.kernel_stack = values.mem_kernel_stack;
                        memory_system_metrics.page_tables = values.mem_page_tables;
                        memory_system_metrics.swap_used =
                            values.mem_swap_total.saturating_sub(values.mem_swap_free);
                    }

                    if vmstat_read_status {
                        let reader = proc_vmstat_reader.borrow();

                        let current = reader.values();
                        let previous = reader.prev_values();

                        let elapsed = memory_system_metrics.time_delta;
                        if elapsed > 0.0 {
                            memory_system_metrics.page_fault_time_delta =
                                current.pgfault.saturating_sub(previous.pgfault) as f64 / elapsed;
                            memory_system_metrics.page_major_fault_time_delta =
                                current.pgmajfault.saturating_sub(previous.pgmajfault) as f64
                                    / elapsed;
                            memory_system_metrics.oom_kill_time_delta =
                                current.oom_kill.saturating_sub(previous.oom_kill) as f64 / elapsed;
                        }
                    }

                    if pressure_memory_read_status {
                        let reader = proc_pressure_memory_reader.borrow();

                        let current = reader.values();
                        let previous = reader.prev_values();

                        let elapsed = memory_system_metrics.time_delta;

                        if elapsed > 0.0 {
                            let stalled_microseconds =
                                current.some_total.saturating_sub(previous.some_total);

                            memory_system_metrics.memory_pressure =
                                100.0 * stalled_microseconds as f64 / (elapsed * 1_000_000.0);
                        }
                    }

                    println!("{:?}", cpu_system_metrics);
                    println!("{:?}", memory_system_metrics);
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
