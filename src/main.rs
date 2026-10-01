mod toml_parser;
mod utils;

use libc::{self};
// use proc_parser::procfs::loadavg::parse_procfs_loadavg;
// use proc_parser::procfs::meminfo::parse_procfs_meminfo;
// use proc_parser::sysfs::diskstats::parse_diskstats;
use proc_parser::common::file_reader::ProcFileReader;
use std::error::Error;
use std::sync::atomic::Ordering;
use std::time::Duration;
use utils::signal_handler;

fn main() -> Result<(), Box<dyn Error>> {
    let is_root = unsafe { libc::getuid() == 0 };
    println!("is_root: {}", is_root);

    let target_path = "/proc/1267200/stat";
    let mut file_reader = ProcFileReader::<256, ()>::new(target_path, false, is_root)?;
    let signal_hook = signal_handler();
    // let diskstats_thread = parse_procfs_meminfo(signal_hook.clone(), 1000, false, is_root);
    while !signal_hook.load(Ordering::Relaxed) {
        let read_status = file_reader.read();
        if !read_status {
            println!("Failed to read procfs file, {}, Its closed!", target_path);
            break;
        }
        let content = file_reader.parse_bytes();
        println!("content: {:?}", content);
        std::thread::sleep(Duration::from_millis(1));
    }
    // diskstats_thread.join().unwrap();
    Ok(())
}
