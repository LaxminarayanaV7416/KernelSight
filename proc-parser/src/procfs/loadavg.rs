use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::parse_decimal_f64;
use crate::common::parser_utils::parse_u64_swar;
use std::collections::HashMap;
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

#[derive(Debug)]
pub struct ProcFSLoadAvgReader {
    reader: ProcFileReader<512>,
    values: ProcLoadAvgFields,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcLoadAvgFields {
    pub load_avg_1m: f64,
    pub load_avg_5m: f64,
    pub load_avg_15m: f64,
    pub num_runnable: u64,
    pub num_total: u64,
    pub last_pid: u64,
}

impl ProcFSLoadAvgReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: ProcLoadAvgFields::default(),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &ProcLoadAvgFields {
        &self.values
    }

    fn set_field(&mut self, field: usize, start: usize, end: usize) {
        let bytes = &self.reader.buffer[start..end];
        if field >= 4 {
            let value = parse_u64_swar(bytes).unwrap_or(0);
            match field {
                4 => self.values.num_runnable = value,
                5 => self.values.num_total = value,
                6 => self.values.last_pid = value,
                _ => {}
            }
        } else {
            let value = parse_decimal_f64(bytes).unwrap_or(0.0);
            match field {
                1 => self.values.load_avg_1m = value,
                2 => self.values.load_avg_5m = value,
                3 => self.values.load_avg_15m = value,
                _ => {}
            }
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        let mut field = 0usize;
        let mut start: Option<usize> = None;

        for i in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[i];

            if byte == b' ' || byte == b'\n' || byte == b'\0' || byte == b'/' {
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
