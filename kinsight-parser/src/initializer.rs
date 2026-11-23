use super::kinsight_utils::PROCFS_ROOT_PARSER_THREAD_NAME;
use kinsight_common::KinsightDataParserTrait;
use kinsight_common::ProcParserError;
use kinsight_common::Result;
use kinsight_common::config::{Config, ProcFSConfigTrait};
use kinsight_common::monotonic;
use kinsight_common::procfs_constants::{
    PROC_LOADAVG_FILE_PATH, PROC_MEMINFO_FILE_PATH, PROC_STAT_FILE_PATH,
};
use kinsight_proc_parser::ProcFSLoadAvgReader;
use kinsight_proc_parser::ProcFSMemInfoReader;
use kinsight_proc_parser::ProcFSStatReader;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

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

    let proc_stat_filter_map = Box::new(config.procfs.stat.get_hashmap());
    let proc_meminfo_filter_map = Box::new(config.procfs.meminfo.get_field_string_to_struct_ids());
    let proc_loadavg_filter_map = Box::new(config.procfs.loadavg.get_hashmap());
    // let prev_time = monotonic();
    let thread_handler = thread_builder
        .spawn(move || {
            let mut has_previous_sample = false;

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

                if stat_read_status {
                    if has_previous_sample {
                        let reader = proc_stat_reader.borrow();

                        let current = reader.values();
                        let previous = reader.prev_values();

                        let current_idle = current.cpu.idle + current.cpu.iowait;

                        let previous_idle = previous.cpu.idle + previous.cpu.iowait;

                        let current_non_idle = current.cpu.user
                            + current.cpu.nice
                            + current.cpu.system
                            + current.cpu.irq
                            + current.cpu.softirq
                            + current.cpu.steal;

                        let previous_non_idle = previous.cpu.user
                            + previous.cpu.nice
                            + previous.cpu.system
                            + previous.cpu.irq
                            + previous.cpu.softirq
                            + previous.cpu.steal;

                        let current_total = current_idle + current_non_idle;

                        let previous_total = previous_idle + previous_non_idle;

                        // checked_sub avoids panicking if counters unexpectedly
                        // decrease, for example after a reset or malformed read.
                        let idle_delta = current_idle.checked_sub(previous_idle);

                        let total_delta = current_total.checked_sub(previous_total);

                        match (idle_delta, total_delta) {
                            (Some(idle_delta), Some(total_delta)) if total_delta > 0 => {
                                let busy_delta = total_delta.saturating_sub(idle_delta);
                                let cpu_usage = 100.0 * busy_delta as f64 / total_delta as f64;
                                println!("CPU USAGE: {:.2}% -- {}", cpu_usage, current_time,);
                            }

                            _ => {
                                eprintln!(
                                    "Unable to calculate CPU usage: \
                                     invalid CPU counter delta"
                                );
                            }
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
