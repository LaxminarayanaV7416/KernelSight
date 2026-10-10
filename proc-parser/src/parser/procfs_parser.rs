use crate::procfs::loadavg::ProcFSLoadAvgReader;
use crate::procfs::meminfo::ProcFSMemInfoReader;
use crate::procfs::pressure::pressure_util::ProcProcessGeneralReader;
use crate::procfs::stat::ProcFSStatReader;
use crate::procfs::uptime::ProcFSUptimeReader;
use crate::procfs::vmstat::ProcFSVmStatReader;
use crate::sysfs::diskstats::DiskStatsReader;
// ---- config import -----
use crate::common::procfs_constants::{
    PROC_FS_ROOT_PATH, PROC_LOADAVG_FILE_SLUG, PROC_PRESSURE_CPU_FILE_SLUG,
    PROC_PRESSURE_FOLDER_SLUG, PROC_PRESSURE_IO_FILE_SLUG, PROC_PRESSURE_IRQ_FILE_SLUG,
    PROC_PRESSURE_MEMORY_FILE_SLUG, PROC_STAT_FILE_SLUG, SYS_FS_BLOCK_SLUG, SYS_FS_ROOT_PATH,
};
use crate::configs::config::{Config, ProcFSConfigTrait};
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn initialize_diskstats(
    is_cachable: bool,
    is_root: bool,
) -> io::Result<Vec<(String, DiskStatsReader)>> {
    let sys_block_path = Path::new(SYS_FS_ROOT_PATH).join(SYS_FS_BLOCK_SLUG);
    let entries = fs::read_dir(&sys_block_path)?;
    let mut diskstats = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path().join(PROC_STAT_FILE_SLUG);
        let resolved_path = path
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Non-UTF8 device path"))?;
        match DiskStatsReader::new(resolved_path, is_cachable, is_root) {
            Ok(reader) => diskstats.push((resolved_path.to_string(), reader)),
            Err(err) => {
                eprintln!("Failed to initialize {}: {:?}", path.display(), err);
            }
        }
    }

    Ok(diskstats)
}

pub fn record_procfs(
    config: &Config,
    signaller: Arc<AtomicBool>,
) -> Result<Vec<JoinHandle<()>>, Box<dyn Error>> {
    let mut threads: Vec<JoinHandle<()>> = vec![];
    let (is_cachable, is_root) = (false, false);
    let global_heart_beat = Box::new(config.heart_beat.clone());
    let debug = Box::new(config.debug.clone());

    // /proc/loadavg
    let is_loadavg_allowed: Box<bool> = Box::new(config.procfs.loadavg.allow.clone());
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH).join(PROC_LOADAVG_FILE_SLUG);
    let mut load_avg_temp_reader = Box::new(ProcFSLoadAvgReader::new(
        &load_avg_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let loadavg_filter_map = Box::new(config.procfs.loadavg.get_hashmap());

    // /proc/diskstats or /sys/block/<device>/stat
    let is_diskstat_allowed: Box<bool> = Box::new(config.procfs.diskstats.allow.clone());
    let mut diskstats_readers = initialize_diskstats(is_cachable, is_root)?;
    let diskstats_filter_map = Box::new(config.procfs.loadavg.get_hashmap());

    // /proc/pressure/cpu
    let pressure_cpu_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_CPU_FILE_SLUG);
    let is_pressure_cpu_allowed: Box<bool> = Box::new(config.procfs.pressure.cpu.allow.clone());
    let mut pressure_cpu_reader = Box::new(ProcProcessGeneralReader::new(
        &pressure_cpu_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let pressure_cpu_filter_map = Box::new(config.procfs.loadavg.get_hashmap());

    // /proc/pressure/io
    let pressure_cpu_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_IO_FILE_SLUG);
    let is_pressure_io_allowed: Box<bool> = Box::new(config.procfs.pressure.cpu.allow.clone());
    let mut pressure_io_reader = Box::new(ProcProcessGeneralReader::new(
        &pressure_cpu_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let pressure_io_filter_map = Box::new(config.procfs.loadavg.get_hashmap());

    // /proc/pressure/memory
    let pressure_cpu_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_MEMORY_FILE_SLUG);
    let is_pressure_memory_allowed: Box<bool> = Box::new(config.procfs.pressure.cpu.allow.clone());
    let mut pressure_memory_reader = Box::new(ProcProcessGeneralReader::new(
        &pressure_cpu_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let pressure_memory_filter_map = Box::new(config.procfs.loadavg.get_hashmap());

    // /proc/pressure/irq
    let pressure_cpu_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_IRQ_FILE_SLUG);
    let is_pressure_irq_allowed: Box<bool> = Box::new(config.procfs.pressure.cpu.allow.clone());
    let mut pressure_irq_reader = Box::new(ProcProcessGeneralReader::new(
        &pressure_cpu_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let pressure_irq_filter_map = Box::new(config.procfs.loadavg.get_hashmap());

    let thread_handle = thread::spawn(move || {
        while !signaller.load(Ordering::Relaxed) {
            if *is_loadavg_allowed {
                let read_status = load_avg_temp_reader.read_and_parse(&loadavg_filter_map);
                //TODO make sure we include the error handling properly
                if !read_status {
                    panic!("Failed to read the /proc/loadavg..");
                }
                if *debug {
                    println!("/proc/loadavg : {:?}", load_avg_temp_reader.values());
                }
            }
            if *is_diskstat_allowed {
                for (path, disk_reader) in diskstats_readers.iter_mut() {
                    let read_status = disk_reader.read_and_parse(&diskstats_filter_map);
                    if !read_status {
                        panic!("Failed to read diskstats, ProcFS File closed!");
                    }
                    if *debug {
                        println!("{} : {:?}", path, disk_reader.values());
                    }
                }
            }
            if *is_pressure_cpu_allowed {
                let read_status = pressure_cpu_reader.read_and_parse(&pressure_cpu_filter_map);
                //TODO make sure we include the error handling properly
                if !read_status {
                    panic!("Failed to read the /proc/pressure/cpu..");
                }
                if *debug {
                    println!("/proc/pressure/cpu : {:?}", pressure_cpu_reader.values());
                }
            }

            if *is_pressure_io_allowed {
                let read_status = pressure_io_reader.read_and_parse(&pressure_io_filter_map);
                //TODO make sure we include the error handling properly
                if !read_status {
                    panic!("Failed to read the /proc/pressure/io..");
                }
                if *debug {
                    println!("/proc/pressure/io : {:?}", pressure_io_reader.values());
                }
            }

            if *is_pressure_memory_allowed {
                let read_status =
                    pressure_memory_reader.read_and_parse(&pressure_memory_filter_map);
                //TODO make sure we include the error handling properly
                if !read_status {
                    panic!("Failed to read the /proc/pressure/memory..");
                }
                if *debug {
                    println!(
                        "/proc/pressure/memory : {:?}",
                        pressure_memory_reader.values()
                    );
                }
            }

            if *is_pressure_irq_allowed {
                let read_status = pressure_irq_reader.read_and_parse(&pressure_irq_filter_map);
                //TODO make sure we include the error handling properly
                if !read_status {
                    panic!("Failed to read the /proc/pressure/irq..");
                }
                if *debug {
                    println!("/proc/pressure/irq : {:?}", pressure_irq_reader.values());
                }
            }

            thread::sleep(Duration::from_millis(*global_heart_beat));
        }
    });

    threads.push(thread_handle);

    Ok(threads)
}
