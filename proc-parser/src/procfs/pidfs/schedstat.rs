use crate::common::file_reader::ProcFileReader;
use crate::common::parser_error::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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
    values: Arc<Mutex<ProcPIDSchedStatFields>>,
}

#[derive(Debug, Default)]
pub struct ProcPIDSchedStatFields {
    pub time_on_cpu: u64,
    pub time_waiting: u64,
    pub timeslices_run_count: u64,
}

impl ProcFSPIDSchedStatReader {
    pub fn new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            reader: ProcFileReader::new(path)?,
            values: Arc::new(Mutex::new(ProcPIDSchedStatFields::default())),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            if let Ok(mut values) = self.values.lock() {
                self.parse_buffer(&mut *values, &field_filter);
            }
        }
        read_status
    }

    pub fn values(&self) -> &Arc<Mutex<ProcPIDSchedStatFields>> {
        &self.values
    }

    fn set_field(&self, values: &mut ProcPIDSchedStatFields, field: usize, value: u64) {
        match field {
            1 => values.time_on_cpu = value,
            2 => values.time_waiting = value,
            3 => values.timeslices_run_count = value,
            _ => {}
        }
    }

    fn parse_buffer(
        &self,
        values: &mut ProcPIDSchedStatFields,
        field_filter: &HashMap<usize, bool>,
    ) {
        let mut field = 1usize;
        let mut selected = field_filter.get(&field).copied().unwrap_or(false);
        let mut token_started = false;
        let mut value = 0u64;
        let mut digit_count = 0usize;
        let mut valid = true;
        for index in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[index];
            if matches!(byte, b' ' | b'\t' | b'\n' | b'\0') {
                if token_started {
                    if selected {
                        let parsed_value = if valid && digit_count > 0 { value } else { 0 };
                        self.set_field(&mut *values, field, parsed_value);
                    }
                    field += 1;
                    // schedstat contains exactly three fields.
                    if field > 3 {
                        break;
                    }
                    selected = field_filter.get(&field).copied().unwrap_or(false);
                    token_started = false;
                    value = 0;
                    digit_count = 0;
                    valid = true;
                }
                continue;
            }
            token_started = true;
            // Continue detecting token boundaries, but avoid numeric work for
            // fields disabled by the configuration.
            if !selected || !valid {
                continue;
            }
            match byte {
                b'0'..=b'9' => {
                    digit_count += 1;

                    // Preserve parse_u64_swar's 20-digit input limit.
                    if digit_count > 20 {
                        valid = false;
                        continue;
                    }
                    let digit = u64::from(byte - b'0');
                    value = match value
                        .checked_mul(10)
                        .and_then(|current| current.checked_add(digit))
                    {
                        Some(value) => value,
                        None => {
                            valid = false;
                            continue;
                        }
                    };
                }
                _ => {
                    valid = false;
                }
            }
        }
        // Handle the third field when the file has no trailing newline or NUL.
        if token_started && field <= 3 && selected {
            let parsed_value = if valid && digit_count > 0 { value } else { 0 };

            self.set_field(&mut *values, field, parsed_value);
        }
    }
}
