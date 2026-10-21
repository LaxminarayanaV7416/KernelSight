use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::parse_u64_swar;
use std::collections::HashMap;
/*
schedstats also adds a new /proc/<pid>/schedstat file to include some of the same information on a per-process level.
There are three fields in this file correlating for that process to:
- time spent on the cpu (in nanoseconds)
- time spent waiting on a runqueue (in nanoseconds)
- # of timeslices run on this cpu
 */

#[derive(Debug)]
pub struct ProcFSPIDSchedStatReader {
    reader: ProcFileReader<64>,
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
