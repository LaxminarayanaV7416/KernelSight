use super::kinsight_utils::PROCFS_ROOT_PARSER_THREAD_NAME;
use super::utils::get_cpu_usage;
use kinsight_common::CPU_CORES;
use kinsight_common::KinsightDataParserTrait;
use kinsight_common::ProcParserError;
use kinsight_common::Result;
use kinsight_common::config::{Config, ProcFSConfigTrait};
use kinsight_common::monotonic;
use kinsight_common::procfs_constants::{
    PROC_LOADAVG_FILE_PATH, PROC_MEMINFO_FILE_PATH, PROC_PRESSURE_CPU_FILE_PATH,
    PROC_STAT_FILE_PATH, PROC_VMSTAT_FILE_PATH,
};
use kinsight_proc_parser::ProcFSLoadAvgReader;
use kinsight_proc_parser::ProcFSMemInfoReader;
use kinsight_proc_parser::ProcFSStatReader;
use kinsight_proc_parser::ProcFSVmStatReader;
use kinsight_proc_parser::ProcProcessGeneralReader;
use std::cell::RefCell;
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

pub fn procfs_root_scanner(
    global_heart_rate: &'static u64,
    signaller: Arc<AtomicBool>, // dont clone we already cloned this sent it over here
    config: &LazyLock<Box<Config>>,
) -> Result<JoinHandle<()>> {
    let thread_builder = thread::Builder::new().name(PROCFS_ROOT_PARSER_THREAD_NAME.to_string());
    // this doesnt contain any hashmap for the pid tracking
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

    let proc_stat_filter_map = Box::new(config.procfs.stat.get_hashmap());
    let proc_meminfo_filter_map = Box::new(config.procfs.meminfo.get_field_string_to_struct_ids());
    let proc_loadavg_filter_map = Box::new(config.procfs.loadavg.get_hashmap());
    let proc_vmstat_filter_map = Box::new(config.procfs.vmstat.get_hashmap());
    let proc_pressure_cpu_filter_map = Box::new(config.procfs.pressure.cpu.get_hashmap());

    // let prev_time = monotonic();
    let thread_handler = thread_builder
        .spawn(move || {
            let mut has_previous_sample = false;
            let mut cpu_core_usage: Vec<f64> = Vec::with_capacity(*CPU_CORES);
            while !signaller.load(Ordering::Relaxed) {
                let current_time = monotonic();

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

                if stat_read_status {
                    if has_previous_sample {
                        let reader = proc_stat_reader.borrow();

                        let current = reader.values();
                        let previous = reader.prev_values();

                        match get_cpu_usage(&current.cpu, &previous.cpu) {
                            Some(cpu_usage) => {
                                println!("CPU Usage: {}, {}", cpu_usage, current_time)
                            }
                            None => println!("Error its an empty value"),
                        }

                        cpu_core_usage.clear();
                        for (curr_cpu, prev_cpu) in
                            current.cpu_cores.iter().zip(&previous.cpu_cores)
                        {
                            match get_cpu_usage(&curr_cpu, &prev_cpu) {
                                Some(cpu_usage) => {
                                    cpu_core_usage.push(cpu_usage);
                                }
                                None => println!("Error its an empty value"),
                            }
                        }
                        for (idx, core_usage) in cpu_core_usage.iter().enumerate() {
                            println!("CPU CORE-{}: {}", idx, core_usage);
                        }
                    } else {
                        // CPU usage is a delta between two samples. The first
                        // successful read only initializes the baseline.
                        has_previous_sample = true;
                    }
                }

                // /proc/meminfo
                if meminfo_read_status {
                    let reader = proc_meminfo_reader.borrow();
                    let values = reader.values();
                    let used_ram = values.mem_total - values.mem_available;
                    let used_swap = values.mem_swap_total - values.mem_swap_free;
                    println!(
                        "TOTAL RAM: {} KB and SWAP TOTAL: {} KB -- {}",
                        used_ram, used_swap, current_time
                    );
                }

                // /proc/loadavg -- for thread info
                if loadavg_read_status {
                    let reader = proc_loadavg_reader.borrow();
                    let values = reader.values();
                    let total_thread_count = values.num_total;
                    let total_thread_running = values.num_runnable;
                    println!(
                        "TOTAL Threads: {}, Running; {}",
                        total_thread_count, total_thread_running
                    );
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
