use crate::common::file_reader::ProcFileReader;
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
pub struct ProcFSUptimeReader {
    reader: ProcFileReader<128>,
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

    fn set_field(&mut self, field: usize, value: f64) {
        match field {
            1 => self.values.suspended_time = value,
            2 => self.values.idle_time = value,
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        let mut field = 1usize;
        let mut selected = field_filter.get(&field).copied().unwrap_or(false);
        let mut token_started = false;
        let mut valid = true;

        let mut integer_part = 0u64;
        let mut fractional_part = 0u64;
        let mut fractional_divisor = 1.0f64;
        let mut seen_dot = false;
        let mut digit_count = 0usize;

        for index in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[index];

            if matches!(byte, b' ' | b'\t' | b'\n' | b'\0') {
                if token_started {
                    if selected {
                        let value = if valid && digit_count > 0 {
                            integer_part as f64 + fractional_part as f64 / fractional_divisor
                        } else {
                            0.0
                        };

                        self.set_field(field, value);
                    }

                    field += 1;

                    // /proc/uptime contains exactly two fields.
                    if field > 2 {
                        break;
                    }

                    selected = field_filter.get(&field).copied().unwrap_or(false);

                    token_started = false;
                    valid = true;

                    integer_part = 0;
                    fractional_part = 0;
                    fractional_divisor = 1.0;
                    seen_dot = false;
                    digit_count = 0;
                }

                continue;
            }

            token_started = true;

            // The field boundary still needs to be detected, but no numeric
            // work is needed for a field disabled by the configuration.
            if !selected || !valid {
                continue;
            }

            match byte {
                b'0'..=b'9' => {
                    digit_count += 1;
                    let digit = u64::from(byte - b'0');

                    if seen_dot {
                        fractional_part = match fractional_part
                            .checked_mul(10)
                            .and_then(|value| value.checked_add(digit))
                        {
                            Some(value) => value,
                            None => {
                                valid = false;
                                continue;
                            }
                        };

                        fractional_divisor *= 10.0;
                    } else {
                        integer_part = match integer_part
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
                }

                b'.' if !seen_dot => {
                    seen_dot = true;
                }

                _ => {
                    valid = false;
                }
            }
        }

        // Handle a second field without a trailing newline or NUL.
        if token_started && field <= 2 && selected {
            let value = if valid && digit_count > 0 {
                integer_part as f64 + fractional_part as f64 / fractional_divisor
            } else {
                0.0
            };

            self.set_field(field, value);
        }
    }
}
