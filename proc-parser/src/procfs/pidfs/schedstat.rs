use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::parse_decimal_f64;
use crate::common::parser_utils::parse_u64_swar;
use crate::common::procfs_constants::PROC_FS_ROOT_PATH;
use crate::configs::procfs_loadavg_config::ProcLoadAvgConfig;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

/*
schedstats also adds a new /proc/<pid>/schedstat file to include some of the same information on a per-process level.
There are three fields in this file correlating for that process to:
- time spent on the cpu (in nanoseconds)
- time spent waiting on a runqueue (in nanoseconds)
- # of timeslices run on this cpu
 */

pub struct ProcFSPIDSchedStatReader {
    reader: ProcFileReader<64, ()>,
    values: ProcPIDSchedStatFields,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcPIDSchedStatFields {
    pub time_on_cpu: u64,
    pub time_waiting: u64,
    pub timeslices_run_count: u64,
}

impl ProcFSPIDSchedStatReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: ProcPIDSchedStatFields::default(),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &ProcPIDSchedStatFields {
        &self.values
    }

    fn set_field(&mut self, field: usize, start: usize, end: usize) {
        let bytes = &self.reader.buffer[start..end];
        let value = parse_u64_swar(bytes).unwrap_or(0);
        match field {
            1 => self.values.time_on_cpu = value,
            2 => self.values.time_waiting = value,
            3 => self.values.timeslices_run_count = value,
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        let mut field = 0usize;
        let mut start: Option<usize> = None;

        for i in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[i];

            if byte == b' ' || byte == b'\n' || byte == b'\0' {
                if let Some(s) = start.take() {
                    field += 1;
                    if field_filter.get(&field).copied().unwrap_or(false) {
                        self.set_field(field, s, i);
                    }
                }
            } else if start.is_none() {
                start = Some(i);
            }
        }

        if let Some(s) = start {
            field += 1;
            if field_filter.get(&field).copied().unwrap_or(false) {
                self.set_field(field, s, self.reader.buffer_len);
            }
        }
    }
}

pub fn parse_procfs_pid_schedstat(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
    pid: usize,
) -> JoinHandle<()> {
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH)
        .join(pid.to_string())
        .join("schedstat");
    let load_avg_temp_reader =
        ProcFSPIDSchedStatReader::new(&load_avg_path.to_str().unwrap(), is_cachable, is_root);
    let mut load_avg_reader = match load_avg_temp_reader {
        Ok(reader) => Box::new(reader),
        Err(e) => panic!("Failed to create loadavg reader: {}", e),
    };
    let filter_map = HashMap::from([(1, true), (2, true), (3, true)]);
    let thread_handle = thread::spawn(move || {
        while !signal_hook.load(Ordering::Relaxed) {
            let read_status = load_avg_reader.read_and_parse(&filter_map);
            if !read_status {
                println!("Failed to read loadavg, ProcFS File closed!");
                break;
            }
            println!("Parsed: {:?}", load_avg_reader.values());
            println!("=====================================");
            thread::sleep(std::time::Duration::from_millis(monitoring_heartbeat));
        }
        println!("Killed LoadAvg thread!!");
    });
    thread_handle
}
