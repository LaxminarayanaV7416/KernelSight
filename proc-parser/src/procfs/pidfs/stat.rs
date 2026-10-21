use crate::common::file_reader::ProcFileReader;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ProcFSPIDStatReader {
    reader: ProcFileReader<4096>,
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

    fn set_unsigned_field(&mut self, field: usize, value: u64) {
        match field {
            9 => self.values.flags = value,
            10 => self.values.minflt = value,
            11 => self.values.cminflt = value,
            12 => self.values.majflt = value,
            13 => self.values.cmajflt = value,
            14 => self.values.utime = value,
            15 => self.values.stime = value,
            22 => self.values.starttime = value,
            23 => self.values.vsize = value,
            25 => self.values.rsslim = value,
            26 => self.values.startcode = value,
            27 => self.values.endcode = value,
            28 => self.values.startstack = value,
            29 => self.values.kstkesp = value,
            30 => self.values.kstkeip = value,
            31 => self.values.signal = value,
            32 => self.values.blocked = value,
            33 => self.values.sigignore = value,
            34 => self.values.sigcatch = value,
            35 => self.values.wchan = value,
            36 => self.values.nswap = value,
            37 => self.values.cnswap = value,
            40 => self.values.rt_priority = value,
            41 => self.values.policy = value,
            42 => self.values.delayacct_blkio_ticks = value,
            43 => self.values.guest_time = value,
            45 => self.values.start_data = value,
            46 => self.values.end_data = value,
            47 => self.values.start_brk = value,
            48 => self.values.arg_start = value,
            49 => self.values.arg_end = value,
            50 => self.values.env_start = value,
            51 => self.values.env_end = value,
            _ => {}
        }
    }

    fn set_signed_field(&mut self, field: usize, value: i64) {
        match field {
            1 => self.values.pid = value,
            4 => self.values.ppid = value,
            5 => self.values.pgrp = value,
            6 => self.values.session = value,
            7 => self.values.tty_nr = value,
            8 => self.values.tpgid = value,
            16 => self.values.cutime = value,
            17 => self.values.cstime = value,
            18 => self.values.priority = value,
            19 => self.values.nice = value,
            20 => self.values.num_threads = value,
            21 => self.values.itrealvalue = value,
            24 => self.values.rss = value,
            38 => self.values.exit_signal = value,
            39 => self.values.processor = value,
            44 => self.values.cguest_time = value,
            52 => self.values.exit_code = value,
            _ => {}
        }
    }

    #[inline]
    fn is_signed_field(field: usize) -> bool {
        matches!(
            field,
            1 | 4 | 5 | 6 | 7 | 8 | 16 | 17 | 18 | 19 | 20 | 21 | 24 | 38 | 39 | 44 | 52
        )
    }

    fn finish_numeric_field(
        &mut self,
        field: usize,
        magnitude: u64,
        negative: bool,
        valid: bool,
        digit_count: usize,
    ) {
        if digit_count == 0 {
            return;
        }
        if Self::is_signed_field(field) {
            let value = if !valid {
                0
            } else if negative {
                if magnitude == (i64::MAX as u64) + 1 {
                    i64::MIN
                } else if magnitude <= i64::MAX as u64 {
                    -(magnitude as i64)
                } else {
                    0
                }
            } else if magnitude <= i64::MAX as u64 {
                magnitude as i64
            } else {
                0
            };
            self.set_signed_field(field, value);
        } else {
            self.set_unsigned_field(field, if valid && !negative { magnitude } else { 0 });
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        let buffer_len = self.reader.buffer_len;

        if buffer_len == 0 || field_filter.is_empty() {
            return;
        }

        // Keep borrows of self.reader.buffer inside this scope. Only copy the
        // resulting indexes and process name out of it.
        let (comm_open, comm_close, comm) = {
            let buffer = &self.reader.buffer[..buffer_len];

            let Some(comm_open) = buffer.iter().position(|byte| *byte == b'(') else {
                return;
            };

            // Use the last ')' because comm itself may contain parentheses.
            let Some(comm_close) = buffer.iter().rposition(|byte| *byte == b')') else {
                return;
            };

            if comm_open >= comm_close {
                return;
            }

            let comm = if field_filter.get(&2).copied().unwrap_or(false) {
                std::str::from_utf8(&buffer[comm_open + 1..comm_close])
                    .ok()
                    .map(str::to_owned)
            } else {
                None
            };

            (comm_open, comm_close, comm)
        };

        // ---------------------------------------------------------------------
        // Field 1: PID
        // ---------------------------------------------------------------------

        if field_filter.get(&1).copied().unwrap_or(false) {
            let mut magnitude = 0u64;
            let mut digit_count = 0usize;
            let mut negative = false;
            let mut valid = true;
            let mut started = false;

            for index in 0..comm_open {
                // Copy the byte so no borrow into self survives this statement.
                let byte = self.reader.buffer[index];

                match byte {
                    b' ' | b'\t' if !started => {}

                    b'-' if !started => {
                        started = true;
                        negative = true;
                    }

                    b'+' if !started => {
                        started = true;
                    }

                    b'0'..=b'9' => {
                        started = true;
                        digit_count += 1;

                        if digit_count > 20 {
                            valid = false;
                            continue;
                        }

                        let digit = u64::from(byte - b'0');

                        magnitude = match magnitude
                            .checked_mul(10)
                            .and_then(|value| value.checked_add(digit))
                        {
                            Some(value) => value,
                            None => {
                                valid = false;
                                continue;
                            }
                        };
                    }

                    b' ' | b'\t' if started => break,

                    _ => {
                        valid = false;
                    }
                }
            }

            self.finish_numeric_field(1, magnitude, negative, valid, digit_count);
        }

        // ---------------------------------------------------------------------
        // Field 2: comm
        // ---------------------------------------------------------------------

        if let Some(comm) = comm {
            // Reuse the String's existing allocation where possible.
            self.values.comm.clear();
            self.values.comm.push_str(&comm);
        }

        // ---------------------------------------------------------------------
        // Field 3: state
        // ---------------------------------------------------------------------

        let mut index = comm_close + 1;

        while index < buffer_len && matches!(self.reader.buffer[index], b' ' | b'\t') {
            index += 1;
        }

        if index >= buffer_len {
            return;
        }

        if field_filter.get(&3).copied().unwrap_or(false) {
            self.values.state = char::from(self.reader.buffer[index]);
        }

        index += 1;

        // --------------------------------------------------------------------
        // Fields 4-52: numeric values
        // ---------------------------------------------------------------------
        let mut field = 4usize;
        let mut selected = field_filter.get(&field).copied().unwrap_or(false);

        let mut token_started = false;
        let mut negative = false;
        let mut sign_seen = false;
        let mut magnitude = 0u64;
        let mut digit_count = 0usize;
        let mut valid = true;
        while index < buffer_len {
            let byte = self.reader.buffer[index];
            if matches!(byte, b' ' | b'\t' | b'\n' | b'\0') {
                if token_started {
                    if selected {
                        self.finish_numeric_field(field, magnitude, negative, valid, digit_count);
                    }
                    field += 1;
                    if field > 52 {
                        break;
                    }
                    selected = field_filter.get(&field).copied().unwrap_or(false);
                    token_started = false;
                    negative = false;
                    sign_seen = false;
                    magnitude = 0;
                    digit_count = 0;
                    valid = true;
                }
                index += 1;
                continue;
            }
            token_started = true;
            // Disabled fields only require delimiter detection.
            if !selected || !valid {
                index += 1;
                continue;
            }
            match byte {
                b'-' if digit_count == 0 && !sign_seen => {
                    negative = true;
                    sign_seen = true;
                }
                b'+' if digit_count == 0 && !sign_seen => {
                    sign_seen = true;
                }
                b'0'..=b'9' => {
                    digit_count += 1;
                    // Preserve the numeric helpers' 20-digit magnitude limit.
                    if digit_count > 20 {
                        valid = false;
                        index += 1;
                        continue;
                    }
                    let digit = u64::from(byte - b'0');
                    magnitude = match magnitude
                        .checked_mul(10)
                        .and_then(|value| value.checked_add(digit))
                    {
                        Some(value) => value,
                        None => {
                            valid = false;
                            index += 1;
                            continue;
                        }
                    };
                }
                _ => {
                    valid = false;
                }
            }
            index += 1;
        }
        // Handle field 52 when the file lacks a trailing newline or NUL.
        if token_started && field <= 52 && selected {
            self.finish_numeric_field(field, magnitude, negative, valid, digit_count);
        }
    }
}
