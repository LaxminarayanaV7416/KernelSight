use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::parse_char;
use crate::common::parser_utils::parse_i64_swar;
use crate::common::parser_utils::parse_string;
use crate::common::parser_utils::parse_u64_swar;
use crate::common::procfs_constants::PROC_FS_ROOT_PATH;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

pub struct ProcFSPIDStatReader {
    reader: ProcFileReader<4096, ()>,
    values: ProcPIDStatFields,
}

#[derive(Debug, Default, Clone)]
pub struct ProcPIDStatFields {
    pub pid: i64,
    pub comm: String,
    pub state: char,
    pub ppid: i64,
    pub pgrp: i64,
    pub session: i64,
    pub tty_nr: i64,
    pub tpgid: i64,
    pub flags: u64,
    pub minflt: u64,
    pub cminflt: u64,
    pub majflt: u64,
    pub cmajflt: u64,
    pub utime: u64,
    pub stime: u64,
    pub cutime: i64,
    pub cstime: i64,
    pub priority: i64,
    pub nice: i64,
    pub num_threads: i64,
    pub itrealvalue: i64,
    pub starttime: u64,
    pub vsize: u64,
    pub rss: i64,
    pub rsslim: u64,
    pub startcode: u64,
    pub endcode: u64,
    pub startstack: u64,
    pub kstkesp: u64,
    pub kstkeip: u64,
    pub signal: u64,
    pub blocked: u64,
    pub sigignore: u64,
    pub sigcatch: u64,
    pub wchan: u64,
    pub nswap: u64,
    pub cnswap: u64,
    pub exit_signal: i64,
    pub processor: i64,
    pub rt_priority: u64,
    pub policy: u64,
    pub delayacct_blkio_ticks: u64,
    pub guest_time: u64,
    pub cguest_time: i64,
    pub start_data: u64,
    pub end_data: u64,
    pub start_brk: u64,
    pub arg_start: u64,
    pub arg_end: u64,
    pub env_start: u64,
    pub env_end: u64,
    pub exit_code: i64,
}

impl ProcFSPIDStatReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: ProcPIDStatFields::default(),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &ProcPIDStatFields {
        &self.values
    }

    fn set_field(&mut self, field: usize, start: usize, end: usize) {
        let bytes = &self.reader.buffer[start..end];
        if field == 2 {
            let value = parse_string(bytes);
            match value {
                Some(v) => self.values.comm = v,
                _ => {}
            }
        } else if field == 3 {
            let value = parse_char(&self.reader.buffer[start]);
            match value {
                Some(v) => self.values.state = v,
                _ => {}
            }
        } else {
            match field {
                1 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.pid = value;
                }
                4 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.ppid = value;
                }
                5 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.pgrp = value;
                }
                6 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.session = value;
                }
                7 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.tty_nr = value;
                }
                8 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.tpgid = value;
                }
                9 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.flags = value;
                }
                10 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.minflt = value;
                }
                11 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.cminflt = value;
                }
                12 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.majflt = value;
                }
                13 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.cmajflt = value;
                }
                14 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.utime = value;
                }
                15 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.stime = value;
                }
                16 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.cutime = value;
                }
                17 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.cstime = value;
                }
                18 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.priority = value;
                }
                19 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.nice = value;
                }
                20 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.num_threads = value;
                }
                21 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.itrealvalue = value;
                }
                22 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.starttime = value;
                }
                23 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.vsize = value;
                }
                24 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.rss = value;
                }
                25 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.rsslim = value;
                }
                26 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.startcode = value;
                }
                27 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.endcode = value;
                }
                28 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.startstack = value;
                }
                29 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.kstkesp = value;
                }
                30 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.kstkeip = value;
                }
                31 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.signal = value;
                }
                32 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.blocked = value;
                }
                33 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.sigignore = value;
                }
                34 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.sigcatch = value;
                }
                35 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.wchan = value;
                }
                36 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.nswap = value;
                }
                37 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.cnswap = value;
                }
                38 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.exit_signal = value;
                }
                39 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.processor = value;
                }
                40 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.rt_priority = value;
                }
                41 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.policy = value;
                }
                42 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.delayacct_blkio_ticks = value;
                }
                43 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.guest_time = value;
                }
                44 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.cguest_time = value;
                }
                45 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.start_data = value;
                }
                46 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.end_data = value;
                }
                47 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.start_brk = value;
                }
                48 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.arg_start = value;
                }
                49 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.arg_end = value;
                }
                50 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.env_start = value;
                }
                51 => {
                    let value = parse_u64_swar(bytes).unwrap_or(0);
                    self.values.env_end = value;
                }
                52 => {
                    let value = parse_i64_swar(bytes).unwrap_or(0);
                    self.values.exit_code = value;
                }
                _ => {}
            }
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        let mut field = 0usize;
        let mut start: Option<usize> = None;
        let mut inside_comm = false;
        for i in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[i];
            if inside_comm {
                if byte == b')' {
                    if let Some(s) = start.take() {
                        field += 1;

                        if field_filter.get(&field).copied().unwrap_or(false) {
                            // Exclude '(' and ')' from comm.
                            self.set_field(field, s, i);
                        }
                    }
                    inside_comm = false;
                }
                continue;
            }
            if byte == b'(' {
                // End previous field if it was active.
                // For /proc/<pid>/stat this should be field 1 pid.
                if let Some(s) = start.take() {
                    field += 1;
                    if field_filter.get(&field).copied().unwrap_or(false) {
                        self.set_field(field, s, i);
                    }
                }
                // Start field 2 after '('.
                inside_comm = true;
                start = Some(i + 1);
                continue;
            }
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

pub fn parse_procfs_pid_stat(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
    pid: usize,
) -> JoinHandle<()> {
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH)
        .join(pid.to_string())
        .join("stat");
    let load_avg_temp_reader =
        ProcFSPIDStatReader::new(&load_avg_path.to_str().unwrap(), is_cachable, is_root);
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
        (8, true),
        (9, true),
        (10, true),
        (11, true),
        (12, true),
        (13, true),
        (14, true),
        (15, true),
        (16, true),
        (17, true),
        (18, true),
        (19, true),
        (20, true),
        (21, true),
        (22, true),
        (23, true),
        (24, true),
        (25, true),
        (26, true),
        (27, true),
        (28, true),
        (29, true),
        (30, true),
        (31, true),
        (32, true),
        (33, true),
        (34, true),
        (35, true),
        (36, true),
        (37, true),
        (38, true),
        (39, true),
        (40, true),
        (41, true),
        (42, true),
        (43, true),
        (44, true),
        (45, true),
        (46, true),
        (47, true),
        (48, true),
        (49, true),
        (50, true),
        (51, true),
        (52, true),
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
