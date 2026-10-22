use crate::common::file_reader::ProcFileReader;
use crate::common::parser_error::Result;
use std::collections::HashMap;
use std::path::PathBuf;
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
    values: Box<ProcLoadAvgFields>,
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
    pub fn new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            reader: ProcFileReader::new(path)?,
            values: Box::new(ProcLoadAvgFields::default()),
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

    fn set_float_field(&mut self, field: usize, value: f64) {
        match field {
            1 => self.values.load_avg_1m = value,
            2 => self.values.load_avg_5m = value,
            3 => self.values.load_avg_15m = value,
            _ => {}
        }
    }

    fn set_integer_field(&mut self, field: usize, value: u64) {
        match field {
            4 => self.values.num_runnable = value,
            5 => self.values.num_total = value,
            6 => self.values.last_pid = value,
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        let mut field = 1usize;
        let mut selected = field_filter.get(&field).copied().unwrap_or(false);
        let mut token_started = false;
        let mut valid = true;

        // State for fields 1-3.
        let mut integer_part = 0u64;
        let mut fractional_part = 0u64;
        let mut fractional_divisor = 1.0f64;
        let mut seen_dot = false;

        // State for fields 4-6.
        let mut integer_value = 0u64;
        let mut integer_digit_count = 0usize;

        for index in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[index];

            let is_delimiter = matches!(byte, b' ' | b'/' | b'\n' | b'\0');

            if is_delimiter {
                if token_started {
                    if selected {
                        if field <= 3 {
                            let value = if valid {
                                integer_part as f64 + fractional_part as f64 / fractional_divisor
                            } else {
                                0.0
                            };

                            self.set_float_field(field, value);
                        } else {
                            let value = if valid && integer_digit_count > 0 {
                                integer_value
                            } else {
                                0
                            };

                            self.set_integer_field(field, value);
                        }
                    }

                    field += 1;

                    // /proc/loadavg has exactly six fields.
                    if field > 6 {
                        break;
                    }

                    selected = field_filter.get(&field).copied().unwrap_or(false);
                    token_started = false;
                    valid = true;
                    integer_part = 0;
                    fractional_part = 0;
                    fractional_divisor = 1.0;
                    seen_dot = false;
                    integer_value = 0;
                    integer_digit_count = 0;
                }

                continue;
            }

            token_started = true;

            // An unselected field only needs delimiter detection.
            if !selected || !valid {
                continue;
            }

            if field <= 3 {
                match byte {
                    b'0'..=b'9' => {
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
            } else {
                match byte {
                    b'0'..=b'9' => {
                        integer_digit_count += 1;

                        // Preserve parse_u64_swar's 20-digit limit.
                        if integer_digit_count > 20 {
                            valid = false;
                            continue;
                        }
                        let digit = u64::from(byte - b'0');
                        integer_value = match integer_value
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
                    _ => {
                        valid = false;
                    }
                }
            }
        }

        // Handle input without a trailing newline or NUL byte.
        if token_started && field <= 6 && selected {
            if field <= 3 {
                let value = if valid {
                    integer_part as f64 + fractional_part as f64 / fractional_divisor
                } else {
                    0.0
                };
                self.set_float_field(field, value);
            } else {
                let value = if valid && integer_digit_count > 0 {
                    integer_value
                } else {
                    0
                };
                self.set_integer_field(field, value);
            }
        }
    }
}
