use crate::common::file_reader::ProcFileReader;
use crate::common::parser_error::Result;
use crate::common::parser_utils::line_tracker;
use std::collections::HashMap;
use std::path::PathBuf;
/*
populated
    1 if the cgroup or its descendants contains any live processes; otherwise, 0.
frozen
    1 if the cgroup is frozen; otherwise, 0.
 */

pub struct CgroupFSV2MemoryEventReader {
    reader: ProcFileReader<256>,
    values: Box<CgroupFSV2MemoryEventFields>,
    lines_map: Option<HashMap<usize, usize>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CgroupFSV2MemoryEventFields {
    pub low: u64,
    pub high: u64,
    pub max: u64,
    pub oom: u64,
    pub oom_kill: u64,
    pub oom_group_kill: u64,
    pub sock_throttled: u64,
}

impl CgroupFSV2MemoryEventReader {
    pub fn new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            reader: ProcFileReader::new(path)?,
            values: Box::new(CgroupFSV2MemoryEventFields::default()),
            lines_map: None,
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<&str, (usize, bool)>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &CgroupFSV2MemoryEventFields {
        &self.values
    }

    #[inline]
    fn set_field(&mut self, field: usize, value: u64) {
        match field {
            1 => self.values.low = value,
            2 => self.values.high = value,
            3 => self.values.max = value,
            4 => self.values.oom = value,
            5 => self.values.oom_kill = value,
            6 => self.values.oom_group_kill = value,
            7 => self.values.sock_throttled = value,
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<&str, (usize, bool)>) {
        if field_filter.is_empty() {
            return;
        }
        // Build the line-to-field mapping once and reuse it for subsequent
        // samples. Available memory.events fields can vary by kernel.
        if self.lines_map.is_none() {
            self.lines_map = Some(line_tracker(
                &self.reader.buffer[..self.reader.buffer_len],
                field_filter,
                b' ',
            ));
        }
        let mut line_number = 1usize;
        let mut line_started = false;

        let mut field = self
            .lines_map
            .as_ref()
            .and_then(|lines| lines.get(&line_number))
            .copied()
            .unwrap_or(0);
        let mut key_finished = false;
        let mut value_started = false;
        let mut value_finished = false;
        let mut value = 0u64;
        let mut digit_count = 0usize;
        let mut valid = true;

        for index in 0..self.reader.buffer_len {
            // Copy the byte so no immutable borrow into the reader survives
            // when set_field() mutably borrows self.
            let byte = self.reader.buffer[index];
            if matches!(byte, b'\n' | b'\0') {
                if line_started {
                    if field > 0 && value_started {
                        let parsed_value = if valid && digit_count > 0 { value } else { 0 };

                        self.set_field(field, parsed_value);
                    }
                    line_number += 1;
                }
                line_started = false;
                field = self
                    .lines_map
                    .as_ref()
                    .and_then(|lines| lines.get(&line_number))
                    .copied()
                    .unwrap_or(0);
                key_finished = false;
                value_started = false;
                value_finished = false;
                value = 0;
                digit_count = 0;
                valid = true;
                continue;
            }
            line_started = true;
            if !key_finished {
                if matches!(byte, b' ' | b'\t') {
                    key_finished = true;
                }
                continue;
            }
            // line_tracker maps unknown and disabled lines to field zero.
            if field == 0 || value_finished {
                continue;
            }
            match byte {
                b'0'..=b'9' => {
                    value_started = true;
                    digit_count += 1;
                    // Preserve parse_u64_swar's maximum token length.
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
                byte if byte.is_ascii_whitespace() => {
                    if value_started {
                        value_finished = true;
                    }
                }
                _ if value_started => {
                    value_finished = true;
                }
                _ => {
                    valid = false;
                    value_finished = true;
                }
            }
        }
        // Handle a final line without trailing newline/NUL.
        if line_started && field > 0 && value_started {
            let parsed_value = if valid && digit_count > 0 { value } else { 0 };

            self.set_field(field, parsed_value);
        }
    }
}
