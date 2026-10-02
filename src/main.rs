mod toml_parser;
mod utils;

use libc::{self};
// use proc_parser::procfs::loadavg::parse_procfs_loadavg;
// use proc_parser::procfs::meminfo::parse_procfs_meminfo;
// use proc_parser::sysfs::diskstats::parse_diskstats;
// use proc_parser::common::file_reader::ProcFileReader;
// use proc_parser::procfs::pidfs::stat::parse_procfs_pid_stat;
use proc_parser::procfs::pidfs::statm::parse_procfs_pid_statm;
// use proc_parser::procfs::pressure::cpu::parse_procfs_pressure_cpu;
// use proc_parser::procfs::pressure::io::parse_procfs_pressure_io;
// use proc_parser::procfs::pressure::irq::parse_procfs_pressure_irq;
// use proc_parser::procfs::pressure::memory::parse_procfs_pressure_memory;
// use proc_parser::common::procfs_constants::PROC_FS_ROOT_PATH;
use std::error::Error;
// use std::fs;
// use std::path::Path;
use std::sync::atomic::Ordering;
use std::time::Duration;
use utils::signal_handler;

fn main() -> Result<(), Box<dyn Error>> {
    let is_root = unsafe { libc::getuid() == 0 };
    println!("is_root: {}", is_root);

    // let target_path = "/proc/1267200/stat";
    // let mut file_reader = ProcFileReader::<256, ()>::new(target_path, false, is_root)?;
    let signal_hook = signal_handler();
    let thread_handle = parse_procfs_pid_statm(signal_hook.clone(), 1000, false, is_root, 1122847);
    // let pressure_cpu_thread = parse_procfs_pressure_cpu(signal_hook.clone(), 1000, false, is_root);
    // let pressure_io_thread = parse_procfs_pressure_io(signal_hook.clone(), 1000, false, is_root);
    // let pressure_irq_thread = parse_procfs_pressure_irq(signal_hook.clone(), 1000, false, is_root);
    // let pressure_memory_thread =
    //     parse_procfs_pressure_memory(signal_hook.clone(), 1000, false, is_root);

    // let proc_path = Path::new(PROC_FS_ROOT_PATH);
    // while !signal_hook.load(Ordering::Relaxed) {
    //     for entry in fs::read_dir(proc_path).into_iter().flatten().flatten() {
    //         let path = entry.path();
    //         if !path.is_dir() {
    //             continue;
    //         }
    //         let pid: Option<usize> = path
    //             .file_name()
    //             .and_then(|n| n.to_str())
    //             .and_then(|s| s.parse::<usize>().ok());
    //         let pid_val = match pid {
    //             Some(v) => v,
    //             None => continue,
    //         };
    //         let result: String = parse_procfs_pid_cgroup(pid_val);
    //         println!("result: {:?}", result);
    //     }
    //     std::thread::sleep(Duration::from_secs(1));
    // }

    // while !signal_hook.load(Ordering::Relaxed) {
    //     let read_status = file_reader.read();
    //     if !read_status {
    //         println!("Failed to read procfs file, {}, Its closed!", target_path);
    //         break;
    //     }
    //     let content = file_reader.parse_bytes();
    //     println!("content: {:?}", content);
    //     std::thread::sleep(Duration::from_secs(1));
    // }
    // pressure_cpu_thread.join().unwrap();
    // pressure_io_thread.join().unwrap();
    // pressure_irq_thread.join().unwrap();
    // pressure_memory_thread.join().unwrap();
    //

    while !signal_hook.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_secs(1));
    }
    thread_handle.join().unwrap();
    Ok(())
}
