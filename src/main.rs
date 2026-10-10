mod toml_parser;
mod utils;
use libc::{self};
use proc_parser::common::parser_utils::CgroupType;
use proc_parser::common::parser_utils::cgroup_classifier;
use proc_parser::common::procfs_constants::AVAILABLE_LOGICAL_CPUS;
use proc_parser::configs::config::Config;
use proc_parser::parser::procfs_parser::record_procfs;
use std::error::Error;
use std::fs;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
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
    let cgroup_type: CgroupType = cgroup_classifier();
    println!("cgroup_type: {:?}", cgroup_type);

    let content = fs::read_to_string(PATH)?;
    let config: Config = basic_toml::from_str(&content)?;
    let signaller: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    signal_handler(&signaller);
    let threads = record_procfs(&config, Arc::clone(&signaller))?;

    
    for thread in threads {
        thread.join().unwrap();
    }
    Ok(())
}
