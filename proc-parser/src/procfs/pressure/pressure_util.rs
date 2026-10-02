use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::CharacterType;
use crate::common::parser_utils::parse_decimal_f64;
use crate::common::parser_utils::parse_u64_swar;
use std::collections::HashMap;

pub struct ProcProcessGeneralReader {
    reader: ProcFileReader<512, ()>,
    values: ProcProcessGeneralFields,
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
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: ProcProcessGeneralFields::default(),
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

    fn set_field(&mut self, field: usize, start: usize, end: usize) {
        let bytes = &self.reader.buffer[start..end];

        match field {
            1 => self.values.some_avg_10 = parse_decimal_f64(bytes).unwrap_or(0.0),
            2 => self.values.some_avg_60 = parse_decimal_f64(bytes).unwrap_or(0.0),
            3 => self.values.some_avg_300 = parse_decimal_f64(bytes).unwrap_or(0.0),
            4 => self.values.some_total = parse_u64_swar(bytes).unwrap_or(0),
            5 => self.values.full_avg_10 = parse_decimal_f64(bytes).unwrap_or(0.0),
            6 => self.values.full_avg_60 = parse_decimal_f64(bytes).unwrap_or(0.0),
            7 => self.values.full_avg_300 = parse_decimal_f64(bytes).unwrap_or(0.0),
            8 => self.values.full_total = parse_u64_swar(bytes).unwrap_or(0),

            _ => {}
        }
    }

    fn pressure_field_id(&self, base: usize, key_start: usize, key_end: usize) -> usize {
        match &self.reader.buffer[key_start..key_end] {
            b"avg10" => base + 1,
            b"avg60" => base + 2,
            b"avg300" => base + 3,
            b"total" => base + 4,
            _ => 0,
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        self.values = ProcProcessGeneralFields::default();
        let mut line_start = 0usize;
        let mut field_base: Option<usize> = None;
        let mut token_start: Option<usize> = None;
        let mut key_start: Option<usize> = None;
        let mut key_end: Option<usize> = None;
        let mut value_start: Option<usize> = None;

        for i in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[i];
            // At the start of each line, decide whether this line is "some" or "full".
            if i == line_start {
                let remaining = &self.reader.buffer[i..self.reader.buffer_len];
                if remaining.starts_with(b"some ") {
                    field_base = Some(0);
                } else if remaining.starts_with(b"full ") {
                    field_base = Some(4);
                } else {
                    field_base = None;
                }
            }
            match byte {
                b' ' => {
                    // Space ends either:
                    // - the prefix token: "some" / "full"
                    // - or a key=value token
                    if let Some(vs) = value_start.take() {
                        if let Some(base) = field_base {
                            if let (Some(ks), Some(ke)) = (key_start, key_end) {
                                let field = self.pressure_field_id(base, ks, ke);
                                if field > 0 && field_filter.get(&field).copied().unwrap_or(false) {
                                    self.set_field(field, vs, i);
                                }
                            }
                        }
                        token_start = None;
                        key_start = None;
                        key_end = None;
                    } else {
                        // This was probably the space after "some" or "full".
                        token_start = None;
                        key_start = None;
                        key_end = None;
                    }
                }
                b'=' => {
                    // We just finished reading the key part.
                    key_end = Some(i);
                    value_start = Some(i + 1);
                }
                b'\n' | b'\0' => {
                    // Newline may end a key=value token, usually total=...
                    if let Some(vs) = value_start.take() {
                        if let Some(base) = field_base {
                            if let (Some(ks), Some(ke)) = (key_start, key_end) {
                                let field = self.pressure_field_id(base, ks, ke);
                                if field > 0 && field_filter.get(&field).copied().unwrap_or(false) {
                                    self.set_field(field, vs, i);
                                }
                            }
                        }
                    }
                    // Reset state for next line.
                    line_start = i + 1;
                    field_base = None;
                    token_start = None;
                    key_start = None;
                    key_end = None;
                    value_start = None;
                }
                _ => {
                    // Start of a token.
                    if token_start.is_none() {
                        token_start = Some(i);
                        key_start = Some(i);
                        key_end = None;
                        value_start = None;
                    }
                }
            }
        }
        // Handle final token if the file does not end with '\n'.
        if let Some(vs) = value_start {
            if let Some(base) = field_base {
                if let (Some(ks), Some(ke)) = (key_start, key_end) {
                    let field = self.pressure_field_id(base, ks, ke);
                    if field > 0 && field_filter.get(&field).copied().unwrap_or(false) {
                        self.set_field(field, vs, self.reader.buffer_len);
                    }
                }
            }
        }
    }
}
