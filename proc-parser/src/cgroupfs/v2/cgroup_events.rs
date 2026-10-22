use crate::common::file_reader::ProcFileReader;
use std::collections::HashMap;
use std::path::PathBuf;
/*
populated
    1 if the cgroup or its descendants contains any live processes; otherwise, 0.
frozen
    1 if the cgroup is frozen; otherwise, 0.
 */

pub struct CgroupFSV2CgroupEventReader {
    reader: ProcFileReader<256>,
    values: Box<CgroupFSV2CgroupEventFields>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CgroupFSV2CgroupEventFields {
    pub populated: bool,
    pub frozen: bool,
}

impl CgroupFSV2CgroupEventReader {
    pub fn new(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path)?,
            values: Box::new(CgroupFSV2CgroupEventFields::default()),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &CgroupFSV2CgroupEventFields {
        &self.values
    }

    #[inline]
    fn set_field(&mut self, field: usize, value: bool) {
        match field {
            1 => self.values.populated = value,
            2 => self.values.frozen = value,
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        if field_filter.is_empty() {
            return;
        }

        let mut line_number = 1usize;
        let mut seen_separator = false;
        let mut value_parsed = false;

        for index in 0..self.reader.buffer_len {
            // Copy the byte so the borrow ends before set_field() takes &mut self.
            let byte = self.reader.buffer[index];

            if matches!(byte, b'\n' | b'\0') {
                line_number += 1;

                if line_number > 2 {
                    break;
                }

                seen_separator = false;
                value_parsed = false;
                continue;
            }

            if value_parsed {
                continue;
            }

            if !seen_separator {
                if matches!(byte, b' ' | b'\t') {
                    seen_separator = true;
                }

                continue;
            }

            // Skip additional whitespace between the key and its value.
            if matches!(byte, b' ' | b'\t') {
                continue;
            }

            let value = match byte {
                b'0' => false,
                b'1' => true,

                // The line is malformed. Ignore the remainder of it.
                _ => {
                    value_parsed = true;
                    continue;
                }
            };

            if field_filter.get(&line_number).copied().unwrap_or(false) {
                self.set_field(line_number, value);
            }

            value_parsed = true;
        }
    }
}
