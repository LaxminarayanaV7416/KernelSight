use crate::common::parser_error::{ProcParserError, Result};
use crate::common::system_call_utils::monotonic;
use crate::procfs::loadavg::ProcFSLoadAvgReader;
use crate::procfs::meminfo::ProcFSMemInfoReader;
use crate::procfs::pressure_util::ProcProcessGeneralReader;
use crate::procfs::stat::ProcFSStatReader;
use crate::procfs::uptime::ProcFSUptimeReader;
use crate::procfs::vmstat::ProcFSVmStatReader;
use crate::sysfs::diskstats::DiskStatsReader;
use crate::writer::writer_utils::{WriteData, WriteDataMsg};
use std::sync::mpsc::Sender;
// ---- config import -----
use crate::common::procfs_constants::{
    PROC_FS_ROOT_PATH, PROC_LOADAVG_FILE_SLUG, PROC_MEMINFO_FILE_SLUG, PROC_PRESSURE_CPU_FILE_SLUG,
    PROC_PRESSURE_FOLDER_SLUG, PROC_PRESSURE_IO_FILE_SLUG, PROC_PRESSURE_IRQ_FILE_SLUG,
    PROC_PRESSURE_MEMORY_FILE_SLUG, PROC_STAT_FILE_SLUG, PROC_UPTIME_FILE_SLUG,
    PROC_VMSTAT_FILE_SLUG, SYS_FS_BLOCK_SLUG, SYS_FS_ROOT_PATH,
};
use crate::configs::config::{Config, ProcFSConfigTrait};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn initialize_diskstats() -> Result<Vec<(PathBuf, DiskStatsReader)>> {
    let sys_block_path = Path::new(SYS_FS_ROOT_PATH).join(SYS_FS_BLOCK_SLUG);
    let entries = fs::read_dir(&sys_block_path).map_err(|_| {
        ProcParserError::FileReadError(sys_block_path.to_string_lossy().into_owned())
    })?;
    let mut diskstats = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|err| ProcParserError::FileReadError(err.to_string()))?;
        let path = entry.path().join(PROC_STAT_FILE_SLUG);
        let reader = DiskStatsReader::new(&path)?;
        diskstats.push((path, reader));
    }
    Ok(diskstats)
}

pub fn record_procfs(
    config: &Config,
    signaller: Arc<AtomicBool>,
    transmit_writer: Sender<WriteDataMsg>,
) -> Result<Vec<Result<JoinHandle<()>>>> {
    let mut threads: Vec<Result<JoinHandle<()>>> = Vec::new();
    let global_heart_beat = Box::new(config.heart_beat.clone());
    let general_signaller = Arc::clone(&signaller);
    // /proc/loadavg
    let is_loadavg_allowed: Box<bool> = Box::new(config.procfs.loadavg.is_enabled());
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH).join(PROC_LOADAVG_FILE_SLUG);
    // below omits a result of our Custom Error so this is allowed with no map_err
    let mut load_avg_temp_reader = Box::new(ProcFSLoadAvgReader::new(&load_avg_path)?);
    let loadavg_filter_map = Box::new(config.procfs.loadavg.get_hashmap());

    // /proc/diskstats or /sys/block/<device>/stat
    let is_diskstat_allowed: Box<bool> = Box::new(config.procfs.diskstats.is_enabled());
    let mut diskstats_readers = initialize_diskstats()?; // this is already mapped so no need to worry about this
    let diskstats_filter_map = Box::new(config.procfs.loadavg.get_hashmap());

    // /proc/pressure/cpu
    let pressure_cpu_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_CPU_FILE_SLUG);
    let is_pressure_cpu_allowed: Box<bool> = Box::new(config.procfs.pressure.cpu.is_enabled());
    let mut pressure_cpu_reader = Box::new(ProcProcessGeneralReader::new(&pressure_cpu_path)?);
    let pressure_cpu_filter_map = Box::new(config.procfs.pressure.cpu.get_hashmap());

    // /proc/pressure/io
    let pressure_io_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_IO_FILE_SLUG);
    let is_pressure_io_allowed: Box<bool> = Box::new(config.procfs.pressure.io.is_enabled());
    let mut pressure_io_reader = Box::new(ProcProcessGeneralReader::new(&pressure_io_path)?);
    let pressure_io_filter_map = Box::new(config.procfs.pressure.io.get_hashmap());

    // /proc/pressure/memory
    let pressure_memory_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_MEMORY_FILE_SLUG);
    let is_pressure_memory_allowed: Box<bool> =
        Box::new(config.procfs.pressure.memory.is_enabled());
    let mut pressure_memory_reader =
        Box::new(ProcProcessGeneralReader::new(&pressure_memory_path)?);
    let pressure_memory_filter_map = Box::new(config.procfs.pressure.memory.get_hashmap());

    // /proc/pressure/irq
    let pressure_irq_path = Path::new(PROC_FS_ROOT_PATH)
        .join(PROC_PRESSURE_FOLDER_SLUG)
        .join(PROC_PRESSURE_IRQ_FILE_SLUG);
    let is_pressure_irq_allowed: Box<bool> = Box::new(config.procfs.pressure.irq.is_enabled());
    let mut pressure_irq_reader = Box::new(ProcProcessGeneralReader::new(&pressure_irq_path)?);
    let pressure_irq_filter_map = Box::new(config.procfs.pressure.irq.get_hashmap());

    // /proc/uptime
    let proc_uptime_path = Path::new(PROC_FS_ROOT_PATH).join(PROC_UPTIME_FILE_SLUG);
    let is_proc_uptime_allowed: Box<bool> = Box::new(config.procfs.uptime.is_enabled());
    let mut proc_uptime_reader = Box::new(ProcFSUptimeReader::new(&proc_uptime_path)?);
    let proc_uptime_filter_map = Box::new(config.procfs.uptime.get_hashmap());

    let transmit_writer_clone = transmit_writer.clone();

    let thread_builder = thread::Builder::new().name("ProcGeneralThread".to_string());
    let group_thread_handle = thread_builder.spawn(move || {
        while !general_signaller.load(Ordering::Relaxed) {
            // globally calculate  the time monotonically
            let time = monotonic();

            if *is_loadavg_allowed {
                let read_status = load_avg_temp_reader.read_and_parse(&loadavg_filter_map);
                match transmit_writer_clone.send(WriteDataMsg::LoadAvg(WriteData::new(
                    time,
                    *load_avg_temp_reader.values(),
                    None,
                    None,
                ))) {
                    Ok(_) => (),
                    Err(_) => {
                        eprintln!(
                            "PROC_PARSER::Sending Message from /proc/loadavg at {} failed!!",
                            time
                        );
                    }
                };
                if !read_status {
                    // dont panic just log that reading have failed!!
                    eprintln!(
                        "PROC_PARSER::Failed to read from /proc/loadavg at {}!!",
                        time
                    );
                }
            }
            if *is_diskstat_allowed {
                for (_, disk_reader) in diskstats_readers.iter_mut() {
                    let read_status = disk_reader.read_and_parse(&diskstats_filter_map);
                    match transmit_writer_clone.send(WriteDataMsg::DiskStats(WriteData::new(
                        time,
                        *disk_reader.values(),
                        None,
                        None,
                    ))) {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!(
                                "PROC_PARSER::Sending Message from /sys/block/<SLUG> at {} failed!!",
                                time
                            );
                        }
                    };
                    if !read_status {
                        // dont panic just log that reading have failed!!
                        eprintln!(
                            "PROC_PARSER::Failed to read from file /sys/block/<SLUG> at {}!!",
                            time
                        );
                    }
                }
            }
            if *is_pressure_cpu_allowed {
                let read_status = pressure_cpu_reader.read_and_parse(&pressure_cpu_filter_map);
                match transmit_writer_clone.send(WriteDataMsg::Pressure(WriteData::new(
                    time,
                    *pressure_cpu_reader.values(),
                    None,
                    Some(PROC_PRESSURE_CPU_FILE_SLUG),
                ))) {
                    Ok(_) => (),
                    Err(_) => {
                        eprintln!(
                            "PROC_PARSER::Sending Message from /proc/pressure/cpu at {} failed!!",
                            time
                        );
                    }
                };
                if !read_status {
                    // dont panic just log that reading have failed!!
                    eprintln!(
                        "PROC_PARSER::Failed to read from file /proc/pressure/cpu at {}!!",
                        time
                    );
                }
            }

            if *is_pressure_io_allowed {
                let read_status = pressure_io_reader.read_and_parse(&pressure_io_filter_map);
                match transmit_writer_clone.send(WriteDataMsg::Pressure(WriteData::new(
                    time,
                    *pressure_io_reader.values(),
                    None,
                    Some(PROC_PRESSURE_IO_FILE_SLUG),
                ))) {
                    Ok(_) => (),
                    Err(_) => {
                        eprintln!(
                            "PROC_PARSER::Sending Message from /proc/pressure/io at {} failed!!",
                            time
                        );
                    }
                };
                if !read_status {
                    // dont panic just log that reading have failed!!
                    eprintln!(
                        "PROC_PARSER::Failed to read from file /proc/pressure/io at {}!!",
                        time
                    );
                }
            }

            if *is_pressure_memory_allowed {
                let read_status =
                    pressure_memory_reader.read_and_parse(&pressure_memory_filter_map);
                match transmit_writer_clone.send(WriteDataMsg::Pressure(WriteData::new(
                    time,
                    *pressure_memory_reader.values(),
                    None,
                    Some(PROC_PRESSURE_MEMORY_FILE_SLUG),
                ))) {
                    Ok(_) => (),
                    Err(_) => {
                        eprintln!(
                            "PROC_PARSER::Sending Message from /proc/pressure/memory at {} failed!!",
                            time
                        );
                    }
                };
                if !read_status {
                    // dont panic just log that reading have failed!!
                    eprintln!(
                        "PROC_PARSER::Failed to read from file /proc/pressure/memory at {}!!",
                        time
                    );
                }
            }

            if *is_pressure_irq_allowed {
                let read_status = pressure_irq_reader.read_and_parse(&pressure_irq_filter_map);
                match transmit_writer_clone.send(WriteDataMsg::Pressure(WriteData::new(
                    time,
                    *pressure_irq_reader.values(),
                    None,
                    Some(PROC_PRESSURE_IRQ_FILE_SLUG),
                ))) {
                    Ok(_) => (),
                    Err(_) => {
                        eprintln!(
                            "PROC_PARSER::Sending Message from /proc/pressure/irq at {} failed!!",
                            time
                        );
                    }
                };
                if !read_status {
                    // dont panic just log that reading have failed!!
                    eprintln!(
                        "PROC_PARSER::Failed to read from file /proc/pressure/irq at {}!!",
                        time
                    );
                }
            }

            if *is_proc_uptime_allowed {
                let read_status = proc_uptime_reader.read_and_parse(&proc_uptime_filter_map);
                match transmit_writer_clone.send(WriteDataMsg::Uptime(WriteData::new(
                    time,
                    *proc_uptime_reader.values(),
                    None,
                    None,
                ))) {
                    Ok(_) => (),
                    Err(_) => {
                        eprintln!(
                            "PROC_PARSER::Sending Message from /proc/uptime at {} failed!!",
                            time
                        );
                    }
                };
                if !read_status {
                    // dont panic just log that reading have failed!!
                    eprintln!(
                        "PROC_PARSER::Failed to read from file /proc/uptime at {}!!",
                        time
                    );
                }
            }

            thread::sleep(Duration::from_millis(*global_heart_beat));
        }
    }).map_err(|io_error| ProcParserError::ThreadSpawningError(io_error.to_string()));
    threads.push(group_thread_handle);

    // the Below requires new thread of there own
    // /proc/vmstat
    let global_heart_beat = Box::new(config.heart_beat.clone());
    let vmstat_signaller = Arc::clone(&signaller);
    let proc_vmstat_path = Path::new(PROC_FS_ROOT_PATH).join(PROC_VMSTAT_FILE_SLUG);
    let is_proc_vmstat_allowed: Box<bool> = Box::new(config.procfs.vmstat.allow.clone());
    let mut proc_vmstat_reader = Box::new(ProcFSVmStatReader::new(&proc_vmstat_path)?);
    let proc_vmstat_filter_map = Box::new(config.procfs.vmstat.get_hashmap());
    let transmit_writer_clone = transmit_writer.clone();
    if *is_proc_vmstat_allowed {
        let thread_builder = thread::Builder::new().name("ProcVmStatThread".to_string());
        let vmstat_thread_handle = thread_builder
            .spawn(move || {
                while !vmstat_signaller.load(Ordering::Relaxed) {
                    let time = monotonic();
                    let read_status = proc_vmstat_reader.read_and_parse(&proc_vmstat_filter_map);
                    match transmit_writer_clone.send(WriteDataMsg::VmStat(WriteData::new(
                        time,
                        *proc_vmstat_reader.values(),
                        None,
                        None,
                    ))) {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!(
                                "PROC_PARSER::Sending Message from /proc/vsmtat at {} failed!!",
                                time
                            );
                        }
                    };
                    if !read_status {
                        // dont panic just log that reading have failed!!
                        eprintln!(
                            "PROC_PARSER::Failed to read from file /proc/vsmtat at {}!!",
                            time
                        );
                    }
                    thread::sleep(Duration::from_millis(*global_heart_beat));
                }
            })
            .map_err(|io_error| ProcParserError::ThreadSpawningError(io_error.to_string()));
        threads.push(vmstat_thread_handle);
    }

    // /proc/stat
    let global_heart_beat = Box::new(config.heart_beat.clone());
    let stat_signaller = Arc::clone(&signaller);
    let proc_stat_path = Path::new(PROC_FS_ROOT_PATH).join(PROC_STAT_FILE_SLUG);
    let is_proc_stat_allowed: Box<bool> = Box::new(config.procfs.stat.allow.clone());
    let mut proc_stat_reader = Box::new(ProcFSStatReader::new(&proc_stat_path)?);
    let proc_stat_filter_map = Box::new(config.procfs.stat.get_hashmap());
    let transmit_writer_clone = transmit_writer.clone();
    if *is_proc_stat_allowed {
        let thread_builder = thread::Builder::new().name("ProcStatThread".to_string());
        let stat_thread_handle = thread_builder
            .spawn(move || {
                while !stat_signaller.load(Ordering::Relaxed) {
                    let time = monotonic();
                    let read_status = proc_stat_reader.read_and_parse(&proc_stat_filter_map);
                    match transmit_writer_clone.send(WriteDataMsg::Stat(WriteData::new(
                        time,
                        proc_stat_reader.take_values(),
                        None,
                        None,
                    ))) {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!(
                                "PROC_PARSER::Sending Message from /proc/stat at {} failed!!",
                                time
                            );
                        }
                    };
                    if !read_status {
                        // dont panic just log that reading have failed!!
                        eprintln!(
                            "PROC_PARSER::Failed to read from file /proc/stat at {}!!",
                            time
                        );
                    }
                    thread::sleep(Duration::from_millis(*global_heart_beat));
                }
            })
            .map_err(|io_error| ProcParserError::ThreadSpawningError(io_error.to_string()));
        threads.push(stat_thread_handle);
    }

    // /proc/meminfo
    let global_heart_beat = Box::new(config.heart_beat.clone());
    let meminfo_signaller = Arc::clone(&signaller);
    let proc_meminfo_path = Path::new(PROC_FS_ROOT_PATH).join(PROC_MEMINFO_FILE_SLUG);
    let is_proc_meminfo_allowed: Box<bool> = Box::new(config.procfs.meminfo.allow.clone());
    let mut proc_meminfo_reader = Box::new(ProcFSMemInfoReader::new(&proc_meminfo_path)?);
    let proc_meminfo_filter_map = Box::new(config.procfs.meminfo.get_field_string_to_struct_ids());
    if *is_proc_meminfo_allowed {
        let thread_builder = thread::Builder::new().name("ProcMemInfoThread".to_string());
        let meminfo_thread_handle = thread_builder
            .spawn(move || {
                while !meminfo_signaller.load(Ordering::Relaxed) {
                    let time = monotonic();
                    let read_status = proc_meminfo_reader.read_and_parse(&proc_meminfo_filter_map);
                    match transmit_writer.send(WriteDataMsg::MemInfo(WriteData::new(
                        time,
                        *proc_meminfo_reader.values(),
                        None,
                        None,
                    ))) {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!(
                                "PROC_PARSER::Sending Message from /proc/meminfo at {} failed!!",
                                time
                            );
                        }
                    };
                    if !read_status {
                        // dont panic just log that reading have failed!!
                        eprintln!(
                            "PROC_PARSER::Failed to read from file /proc/meminfo at {}!!",
                            time
                        );
                    }
                    thread::sleep(Duration::from_millis(*global_heart_beat));
                }
            })
            .map_err(|io_error| ProcParserError::ThreadSpawningError(io_error.to_string()));
        threads.push(meminfo_thread_handle);
    }

    Ok(threads)
}
