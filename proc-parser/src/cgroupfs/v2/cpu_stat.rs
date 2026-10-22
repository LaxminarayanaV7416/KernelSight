use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::line_tracker;
use std::collections::HashMap;
use std::path::PathBuf;

/*
populated
    1 if the cgroup or its descendants contains any live processes; otherwise, 0.
frozen
    1 if the cgroup is frozen; otherwise, 0.
 */

pub struct CgroupFSV2CPUStatReader {
    reader: ProcFileReader<256>,
    values: CgroupFSV2CPUStatFields,
    lines_map: Option<HashMap<usize, usize>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CgroupFSV2CPUStatFields {
    pub usage_usec: u64,
    pub user_usec: u64,
    pub system_usec: u64,
    pub nice_usec: u64,
    pub core_sched_force_idle_usec: u64,
    pub nr_periods: u64,
    pub nr_throttled: u64,
    pub throttled_usec: u64,
    pub nr_bursts: u64,
    pub burst_usec: u64,
}

impl CgroupFSV2CPUStatReader {
    pub fn new(
        path: &PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path)?,
            values: CgroupFSV2CPUStatFields::default(),
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

    pub fn values(&self) -> &CgroupFSV2CPUStatFields {
        &self.values
    }

    #[inline]
    fn set_field(&mut self, field: usize, value: u64) {
        match field {
            1 => self.values.usage_usec = value,
            2 => self.values.user_usec = value,
            3 => self.values.system_usec = value,
            4 => self.values.nice_usec = value,
            5 => self.values.core_sched_force_idle_usec = value,
            6 => self.values.nr_periods = value,
            7 => self.values.nr_throttled = value,
            8 => self.values.throttled_usec = value,
            9 => self.values.nr_bursts = value,
            10 => self.values.burst_usec = value,
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<&str, (usize, bool)>) {
        if field_filter.is_empty() {
            return;
        }
        // Build the line-to-field mapping once. CPU stat fields may vary by
        // kernel and controller configuration, so name-based discovery is
        // preferable to assuming fixed line positions.
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
            // Copy the byte so no immutable buffer borrow survives when
            // set_field() mutably borrows self.
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
            // Such lines only need to be scanned until their newline.
            if field == 0 || value_finished {
                continue;
            }
            match byte {
                b'0'..=b'9' => {
                    value_started = true;
                    digit_count += 1;

                    // Preserve parse_u64_swar's maximum input length.
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
                    // Only the first contiguous decimal value is relevant.
                    value_finished = true;
                }
                _ => {
                    // A non-whitespace character before the number makes this
                    // selected line malformed.
                    valid = false;
                    value_finished = true;
                }
            }
        }
        // Handle a final line without newline/NUL termination.
        if line_started && field > 0 && value_started {
            let parsed_value = if valid && digit_count > 0 { value } else { 0 };

            self.set_field(field, parsed_value);
        }
    }
}
