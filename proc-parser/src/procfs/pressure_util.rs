use crate::common::file_reader::ProcFileReader;
use crate::common::parser_error::Result;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ProcProcessGeneralReader {
    reader: ProcFileReader<512>,
    values: Box<ProcProcessGeneralFields>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcProcessGeneralFields {
    pub some_avg_10: f64,
    pub some_avg_60: f64,
    pub some_avg_300: f64,
    pub some_total: u64,
    pub full_avg_10: f64,
    pub full_avg_60: f64,
    pub full_avg_300: f64,
    pub full_total: u64,
}

impl ProcProcessGeneralReader {
    pub fn new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            reader: ProcFileReader::new(path)?,
            values: Box::new(ProcProcessGeneralFields::default()),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &ProcProcessGeneralFields {
        &self.values
    }

    fn set_float_field(&mut self, field: usize, value: f64) {
        match field {
            1 => self.values.some_avg_10 = value,
            2 => self.values.some_avg_60 = value,
            3 => self.values.some_avg_300 = value,
            5 => self.values.full_avg_10 = value,
            6 => self.values.full_avg_60 = value,
            7 => self.values.full_avg_300 = value,
            _ => {}
        }
    }

    fn set_integer_field(&mut self, field: usize, value: u64) {
        match field {
            4 => self.values.some_total = value,
            8 => self.values.full_total = value,
            _ => {}
        }
    }

    fn field_from_key(base: usize, key: u64) -> usize {
        const AVG10: u64 = 0x61_76_67_31_30; // this is the hex representation of the
        const AVG60: u64 = 0x61_76_67_36_30; // fields so we can store them easily
        const AVG300: u64 = 0x61_76_67_33_30_30; // learned about this by looking at
        const TOTAL: u64 = 0x74_6f_74_61_6c; // other repos, better for systems programming

        match key {
            AVG10 => base + 1,
            AVG60 => base + 2,
            AVG300 => base + 3,
            TOTAL => base + 4,
            _ => 0,
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        const SOME: u64 = 0x73_6f_6d_65;
        const FULL: u64 = 0x66_75_6c_6c;
        // self.values = Box::new(ProcProcessGeneralFields::default());
        let mut field_base: Option<usize> = None;
        let mut token_code = 0u64;
        let mut token_length = 0usize;

        let mut field = 0usize;
        let mut selected = false;
        let mut parsing_value = false;
        let mut valid = true;

        // Decimal parsing state.
        let mut integer_part = 0u64;
        let mut fractional_part = 0u64;
        let mut fractional_divisor = 1.0f64;
        let mut seen_dot = false;
        let mut digit_count = 0usize;

        for index in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[index];

            match byte {
                b'=' if !parsing_value => {
                    field = field_base
                        .map(|base| Self::field_from_key(base, token_code))
                        .unwrap_or(0);

                    selected = field > 0 && field_filter.get(&field).copied().unwrap_or(false);

                    parsing_value = true;
                    valid = true;

                    integer_part = 0;
                    fractional_part = 0;
                    fractional_divisor = 1.0;
                    seen_dot = false;
                    digit_count = 0;
                }

                b' ' | b'\n' | b'\0' => {
                    if parsing_value {
                        if selected {
                            if matches!(field, 4 | 8) {
                                let value = if valid && digit_count > 0 {
                                    integer_part
                                } else {
                                    0
                                };

                                self.set_integer_field(field, value);
                            } else {
                                let value = if valid && digit_count > 0 {
                                    integer_part as f64
                                        + fractional_part as f64 / fractional_divisor
                                } else {
                                    0.0
                                };

                                self.set_float_field(field, value);
                            }
                        }
                    } else if field_base.is_none() {
                        // The first token on each line is either "some" or "full".
                        field_base = match token_code {
                            SOME => Some(0),
                            FULL => Some(4),
                            _ => None,
                        };
                    }

                    token_code = 0;
                    token_length = 0;
                    field = 0;
                    selected = false;
                    parsing_value = false;
                    valid = true;

                    integer_part = 0;
                    fractional_part = 0;
                    fractional_divisor = 1.0;
                    seen_dot = false;
                    digit_count = 0;

                    if matches!(byte, b'\n' | b'\0') {
                        field_base = None;
                    }
                }

                _ if parsing_value => {
                    // For unselected fields, only scan until the next delimiter.
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
                                // Preserve parse_u64_swar's limit for total fields.
                                if matches!(field, 4 | 8) && digit_count > 20 {
                                    valid = false;
                                    continue;
                                }

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

                        b'.' if !seen_dot && !matches!(field, 4 | 8) => {
                            seen_dot = true;
                        }

                        _ => {
                            valid = false;
                        }
                    }
                }

                _ => {
                    // Build a compact key identifier while scanning the key.
                    // PSI keys are at most six bytes long.
                    if token_length < 8 {
                        token_code = (token_code << 8) | u64::from(byte);
                        token_length += 1;
                    } else {
                        // Ensure an unexpectedly long key cannot match.
                        token_code = u64::MAX;
                    }
                }
            }
        }

        // Handle a final value when the file has no trailing newline.
        if parsing_value && selected {
            if matches!(field, 4 | 8) {
                let value = if valid && digit_count > 0 {
                    integer_part
                } else {
                    0
                };

                self.set_integer_field(field, value);
            } else {
                let value = if valid && digit_count > 0 {
                    integer_part as f64 + fractional_part as f64 / fractional_divisor
                } else {
                    0.0
                };

                self.set_float_field(field, value);
            }
        }
    }
}
