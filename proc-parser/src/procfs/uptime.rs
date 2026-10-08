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
Field Number
Field 1 - Float
    1-Minute Load Average
    The average number of scheduling entities (processes/threads) that were either executable (State R)
    or blocked waiting for disk I/O (State D) over the last minute.
Field 2 - Float
    5-Minute Load Average
    The same workload metric averaged over the last 5 minutes.
Field 3 - Float
    15-Minute Load Average
    The same workload metric averaged over the last 15 minutes.
Field 4 - int32/int32
    Runnable / Total Entities
    Consists of two numbers separated by a slash (/):
    Numerator: The number of kernel scheduling entities currently executable and waiting to run.
    Denominator: The total number of kernel scheduling entities that currently exist on the system.
Field 5 - int32
    Last Created PID
    The Process ID (PID) of the most recently allocated process or thread on the system.
 */

pub struct ProcFSUptimeReader {
    reader: ProcFileReader<128, ()>,
    values: ProcUptimeFields,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcUptimeFields {
    pub suspended_time: f64,
    pub idle_time: f64,
}

impl ProcFSUptimeReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: ProcUptimeFields::default(),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &ProcUptimeFields {
        &self.values
    }

    fn set_field(&mut self, field: usize, start: usize, end: usize) {
        let bytes = &self.reader.buffer[start..end];
        let value = parse_decimal_f64(bytes).unwrap_or(0.0);
        match field {
            1 => self.values.suspended_time = value,
            2 => self.values.idle_time = value,
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

pub fn parse_procfs_uptime(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
) -> JoinHandle<()> {
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH).join("uptime");
    let load_avg_temp_reader =
        ProcFSUptimeReader::new(&load_avg_path.to_str().unwrap(), is_cachable, is_root);
    let mut load_avg_reader = match load_avg_temp_reader {
        Ok(reader) => Box::new(reader),
        Err(e) => panic!("Failed to create loadavg reader: {}", e),
    };
    let filter_map = HashMap::from([(1, true), (2, true)]);
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
