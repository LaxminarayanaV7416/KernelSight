use crate::common::file_reader::ProcFileReader;
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
/proc/pid/statm
Provides information about memory usage, measured in pages.
The columns are:
size       (1) total program size
            (same as VmSize in /proc/pid/status)
resident   (2) resident set size
            (inaccurate; same as VmRSS in /proc/pid/status)
shared     (3) number of resident shared pages
            (i.e., backed by a file)
            (inaccurate; same as RssFile+RssShmem in
            /proc/pid/status)
text       (4) text (code)
lib        (5) library (unused since Linux 2.6; always 0)
data       (6) data + stack
dt         (7) dirty pages (unused since Linux 2.6; always 0)

Some of these values are inaccurate because of a kernel-
internal scalability optimization.  If accurate values are
required, use /proc/pid/smaps or /proc/pid/smaps_rollup
instead, which are much slower but provide accurate,
detailed information.
 */

pub struct ProcFSPIDStatmReader {
    reader: ProcFileReader<512>,
    values: ProcPIDStatmFields,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcPIDStatmFields {
    pub size: u64,
    pub resident: u64,
    pub shared: u64,
    pub text: u64,
    pub lib: u64,
    pub data: u64,
    pub dirty: u64,
}

impl ProcFSPIDStatmReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: ProcPIDStatmFields::default(),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &ProcPIDStatmFields {
        &self.values
    }

    fn set_field(&mut self, field: usize, start: usize, end: usize) {
        let bytes = &self.reader.buffer[start..end];
        let value = parse_u64_swar(bytes).unwrap_or(0);
        match field {
            1 => self.values.size = value,
            2 => self.values.resident = value,
            3 => self.values.shared = value,
            4 => self.values.text = value,
            5 => self.values.lib = value,
            6 => self.values.data = value,
            7 => self.values.dirty = value,
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        if field_filter.is_empty() {
            return;
        }
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

pub fn parse_procfs_pid_statm(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
    pid: usize,
) -> JoinHandle<()> {
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH)
        .join(pid.to_string())
        .join("statm");
    let load_avg_temp_reader =
        ProcFSPIDStatmReader::new(&load_avg_path.to_str().unwrap(), is_cachable, is_root);
    let mut load_avg_reader = match load_avg_temp_reader {
        Ok(reader) => Box::new(reader),
        Err(e) => panic!("Failed to create loadavg reader: {}", e),
    };
    let filter_map = HashMap::from([
        (1, true),
        (2, true),
        (3, true),
        (4, true),
        (5, true),
        (6, true),
        (7, true),
    ]);
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
