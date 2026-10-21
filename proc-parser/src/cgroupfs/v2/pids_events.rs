use crate::common::file_reader::ProcFileReader;
use std::collections::HashMap;

/*
populated
    1 if the cgroup or its descendants contains any live processes; otherwise, 0.
frozen
    1 if the cgroup is frozen; otherwise, 0.
 */

pub struct CgroupFSV2PidsEventReader {
    reader: ProcFileReader<256>,
    values: CgroupFSV2PidsEventFields,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CgroupFSV2PidsEventFields {
    pub max: u64,
}

impl CgroupFSV2PidsEventReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: CgroupFSV2PidsEventFields::default(),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<&str, (usize, bool)>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &CgroupFSV2PidsEventFields {
        &self.values
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<&str, (usize, bool)>) {
        let max_enabled = field_filter
            .get("max")
            .map(|(_, enabled)| *enabled)
            .unwrap_or(false);

        if !max_enabled {
            return;
        }
        const MAX_KEY: &[u8] = b"max";
        let mut key_index = 0usize;
        let mut key_matches = true;
        let mut key_finished = false;
        let mut is_max_line = false;
        let mut value_started = false;
        let mut value_finished = false;
        let mut value = 0u64;
        let mut digit_count = 0usize;
        let mut valid = true;
        for index in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[index];
            if matches!(byte, b'\n' | b'\0') {
                if is_max_line {
                    self.values.max = if value_started && valid && digit_count > 0 {
                        value
                    } else {
                        0
                    };
                    // The desired field has been found; no further scan is
                    // necessary.
                    return;
                }
                key_index = 0;
                key_matches = true;
                key_finished = false;
                is_max_line = false;
                value_started = false;
                value_finished = false;
                value = 0;
                digit_count = 0;
                valid = true;
                continue;
            }
            if !key_finished {
                if matches!(byte, b' ' | b'\t') {
                    key_finished = true;
                    is_max_line = key_matches && key_index == MAX_KEY.len();
                    continue;
                }
                if key_index >= MAX_KEY.len() || byte != MAX_KEY[key_index] {
                    key_matches = false;
                }
                key_index += 1;
                continue;
            }
            if !is_max_line || value_finished {
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
        // Handle a max line without trailing newline/NUL.
        if is_max_line {
            self.values.max = if value_started && valid && digit_count > 0 {
                value
            } else {
                0
            };
        }
    }
}
