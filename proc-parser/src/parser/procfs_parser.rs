use crate::procfs::loadavg::ProcFSLoadAvgReader;
use crate::procfs::meminfo::ProcFSMemInfoReader;
use crate::procfs::pressure::pressure_util::ProcProcessGeneralReader;
use crate::procfs::stat::ProcFSStatReader;
use crate::procfs::uptime::ProcFSUptimeReader;
use crate::procfs::vmstat::ProcFSVmStatReader;
use crate::sysfs::diskstats::DiskStatsReader;
// ---- config import -----
use crate::common::procfs_constants::{
    PROC_FS_ROOT_PATH, PROC_LOADAVG_FILE_SLUG, PROC_MEMINFO_FILE_SLUG, PROC_PRESSURE_CPU_FILE_SLUG,
    PROC_PRESSURE_FOLDER_SLUG, PROC_PRESSURE_IO_FILE_SLUG, PROC_PRESSURE_IRQ_FILE_SLUG,
    PROC_PRESSURE_MEMORY_FILE_SLUG, PROC_STAT_FILE_SLUG, PROC_UPTIME_FILE_SLUG,
    PROC_VMSTAT_FILE_SLUG, SYS_FS_BLOCK_SLUG, SYS_FS_ROOT_PATH,
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
    let debug = Box::new(config.debug.clone()); //Box::new(false);
    let general_signaller = Arc::clone(&signaller);
    // /proc/loadavg
    let is_loadavg_allowed: Box<bool> = Box::new(config.procfs.loadavg.is_enabled());
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH).join(PROC_LOADAVG_FILE_SLUG);
    let mut load_avg_temp_reader = Box::new(ProcFSLoadAvgReader::new(
        &load_avg_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let loadavg_filter_map = Box::new(config.procfs.loadavg.get_hashmap());

    // /proc/diskstats or /sys/block/<device>/stat
    let is_diskstat_allowed: Box<bool> = Box::new(config.procfs.diskstats.is_enabled());
    let mut diskstats_readers = initialize_diskstats(is_cachable, is_root)?;
    let diskstats_filter_map = Box::new(config.procfs.loadavg.get_hashmap());

    // /proc/pressure/cpu
    let pressure_cpu_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_CPU_FILE_SLUG);
    let is_pressure_cpu_allowed: Box<bool> = Box::new(config.procfs.pressure.cpu.is_enabled());
    let mut pressure_cpu_reader = Box::new(ProcProcessGeneralReader::new(
        &pressure_cpu_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let pressure_cpu_filter_map = Box::new(config.procfs.pressure.cpu.get_hashmap());

    // /proc/pressure/io
    let pressure_io_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_IO_FILE_SLUG);
    let is_pressure_io_allowed: Box<bool> = Box::new(config.procfs.pressure.io.is_enabled());
    let mut pressure_io_reader = Box::new(ProcProcessGeneralReader::new(
        &pressure_io_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let pressure_io_filter_map = Box::new(config.procfs.pressure.io.get_hashmap());

    // /proc/pressure/memory
    let pressure_memory_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_MEMORY_FILE_SLUG);
    let is_pressure_memory_allowed: Box<bool> =
        Box::new(config.procfs.pressure.memory.is_enabled());
    let mut pressure_memory_reader = Box::new(ProcProcessGeneralReader::new(
        &pressure_memory_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let pressure_memory_filter_map = Box::new(config.procfs.pressure.memory.get_hashmap());

    // /proc/pressure/irq
    let pressure_irq_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_IRQ_FILE_SLUG);
    let is_pressure_irq_allowed: Box<bool> = Box::new(config.procfs.pressure.irq.is_enabled());
    let mut pressure_irq_reader = Box::new(ProcProcessGeneralReader::new(
        &pressure_irq_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let pressure_irq_filter_map = Box::new(config.procfs.pressure.irq.get_hashmap());

    // /proc/uptime
    let proc_uptime_path = Path::new(PROC_FS_ROOT_PATH).join(PROC_UPTIME_FILE_SLUG);
    let is_proc_uptime_allowed: Box<bool> = Box::new(config.procfs.uptime.is_enabled());
    let mut proc_uptime_reader = Box::new(ProcFSUptimeReader::new(
        &proc_uptime_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let proc_uptime_filter_map = Box::new(config.procfs.uptime.get_hashmap());

    if *is_loadavg_allowed
        & *is_diskstat_allowed
        & *is_pressure_cpu_allowed
        & *is_pressure_io_allowed
        & *is_pressure_memory_allowed
        & *is_pressure_irq_allowed
        & *is_proc_uptime_allowed
    {
        let group_thread_handle = thread::spawn(move || {
            while !general_signaller.load(Ordering::Relaxed) {
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

                if *is_proc_uptime_allowed {
                    let read_status = proc_uptime_reader.read_and_parse(&proc_uptime_filter_map);
                    //TODO make sure we include the error handling properly
                    if !read_status {
                        panic!("Failed to read the /proc/uptime..");
                    }
                    if *debug {
                        println!("/proc/uptime : {:?}", proc_uptime_reader.values());
                    }
                }

                thread::sleep(Duration::from_millis(*global_heart_beat));
            }
        });
        threads.push(group_thread_handle);
    }

    // the Below requires new thread of there own
    // /proc/vmstat
    let global_heart_beat = Box::new(config.heart_beat.clone());
    let debug = Box::new(config.debug.clone());
    let vmstat_signaller = Arc::clone(&signaller);
    let proc_vmstat_path = Path::new(PROC_FS_ROOT_PATH).join(PROC_VMSTAT_FILE_SLUG);
    let is_proc_vmstat_allowed: Box<bool> = Box::new(config.procfs.vmstat.allow.clone());
    let mut proc_vmstat_reader = Box::new(ProcFSVmStatReader::new(
        &proc_vmstat_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let proc_vmstat_filter_map = Box::new(config.procfs.vmstat.get_hashmap());

    if *is_proc_vmstat_allowed {
        let vmstat_thread_handle = thread::spawn(move || {
            while !vmstat_signaller.load(Ordering::Relaxed) {
                let read_status = proc_vmstat_reader.read_and_parse(&proc_vmstat_filter_map);
                //TODO make sure we include the error handling properly
                if !read_status {
                    panic!("Failed to read the /proc/vmstat..");
                }
                if *debug {
                    println!("/proc/vmstat : {:?}", proc_vmstat_reader.values());
                }
                thread::sleep(Duration::from_millis(*global_heart_beat));
            }
        });
        threads.push(vmstat_thread_handle);
    }

    // /proc/stat
    let global_heart_beat = Box::new(config.heart_beat.clone());
    let debug = Box::new(config.debug.clone());
    let stat_signaller = Arc::clone(&signaller);
    let proc_stat_path = Path::new(PROC_FS_ROOT_PATH).join(PROC_STAT_FILE_SLUG);
    let is_proc_stat_allowed: Box<bool> = Box::new(config.procfs.stat.allow.clone());
    let mut proc_stat_reader = Box::new(ProcFSStatReader::new(
        &proc_stat_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let proc_stat_filter_map = Box::new(config.procfs.stat.get_hashmap());

    if *is_proc_stat_allowed {
        let stat_thread_handle = thread::spawn(move || {
            while !stat_signaller.load(Ordering::Relaxed) {
                let read_status = proc_stat_reader.read_and_parse(&proc_stat_filter_map);
                //TODO make sure we include the error handling properly
                if !read_status {
                    panic!("Failed to read the /proc/stat..");
                }
                if *debug {
                    println!("/proc/stat : {:?}", proc_stat_reader.values());
                }
                thread::sleep(Duration::from_millis(*global_heart_beat));
            }
        });
        threads.push(stat_thread_handle);
    }

    // /proc/meminfo
    let global_heart_beat = Box::new(config.heart_beat.clone());
    let debug = Box::new(config.debug.clone());
    let meminfo_signaller = Arc::clone(&signaller);
    let proc_meminfo_path = Path::new(PROC_FS_ROOT_PATH).join(PROC_MEMINFO_FILE_SLUG);
    let is_proc_meminfo_allowed: Box<bool> = Box::new(config.procfs.meminfo.allow.clone());
    let mut proc_meminfo_reader = Box::new(ProcFSMemInfoReader::new(
        &proc_meminfo_path.to_str().unwrap(),
        is_cachable,
        is_root,
    )?);
    let proc_meminfo_filter_map = Box::new(config.procfs.meminfo.get_field_string_to_struct_ids());

    if *is_proc_meminfo_allowed {
        let meminfo_thread_handle = thread::spawn(move || {
            while !meminfo_signaller.load(Ordering::Relaxed) {
                let read_meminfo = proc_meminfo_reader.read_and_parse(&proc_meminfo_filter_map);
                //TODO make sure we include the error handling properly
                if !read_meminfo {
                    panic!("Failed to read the /proc/meminfo..");
                }
                if *debug {
                    println!("/proc/meminfo : {:?}", proc_meminfo_reader.values());
                }
                thread::sleep(Duration::from_millis(*global_heart_beat));
            }
        });
        threads.push(meminfo_thread_handle);
    }

    Ok(threads)
}
