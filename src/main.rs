mod toml_parser;
mod utils;
use libc::printf;
use libc::{self};
use proc_parser::common::parser_utils::CgroupType;
use proc_parser::common::parser_utils::cgroup_classifier;
use proc_parser::common::procfs_constants::AVAILABLE_LOGICAL_CPUS;
use proc_parser::configs::master_config::Config;
use proc_parser::gpu::nvidia::metrics::NvidiaGPUMetrics;
use std::error::Error;
use std::fs;
use std::sync::atomic::Ordering;
use std::time::Duration;
use utils::signal_handler;

const PATH: &str = "/home/lax/Documents/perf_insight_lite/proc-parser/src/config.toml";

fn main() -> Result<(), Box<dyn Error>> {
    let is_root = unsafe { libc::getuid() == 0 };
    println!(
        "is_root: {} and Available Logical CPUs: {}",
        is_root, *AVAILABLE_LOGICAL_CPUS
    );

    let content = fs::read_to_string(PATH)?;
    let config: Config = basic_toml::from_str(&content)?;

    println!("CONFIG : {:?}", config);
    
    let cgroup_type: CgroupType = cgroup_classifier();
    println!("cgroup_type: {:?}", cgroup_type);

    let signal_hook = signal_handler();
    // let thread_handle = parse_procfs_vmstat(signal_hook.clone(), 1000, false, is_root);
    // let mut nvml = NvidiaGPUMetrics::new();
    while !signal_hook.load(Ordering::Relaxed) {
        // let data = nvml.get_device_details();
        // println!("{:?}", data);
        std::thread::sleep(Duration::from_secs(1));
    }
    // thread_handle.join().unwrap();
    // nvml.close_nvml()?;
    Ok(())
}
