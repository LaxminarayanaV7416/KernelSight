use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::parse_bool;
use std::collections::HashMap;
/*
populated
    1 if the cgroup or its descendants contains any live processes; otherwise, 0.
frozen
    1 if the cgroup is frozen; otherwise, 0.
 */

pub struct CgroupFSV2CgroupEventReader {
    reader: ProcFileReader<256>,
    values: CgroupFSV2CgroupEventFields,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CgroupFSV2CgroupEventFields {
    pub populated: bool,
    pub frozen: bool,
}

impl CgroupFSV2CgroupEventReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: CgroupFSV2CgroupEventFields::default(),
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

    fn set_field(&mut self, field: usize, start: usize) {
        let bytes = &self.reader.buffer[start];
        let value = parse_bool(bytes).unwrap_or(false);
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
        let mut value_already_sent_for_line = false;

        for i in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[i];

            if byte == b' ' && !value_already_sent_for_line {
                let value_start = i + 1;

                if value_start < self.reader.buffer_len
                    && field_filter.get(&line_number).copied().unwrap_or(false)
                {
                    // Value is only one byte: b'0' or b'1'.
                    self.set_field(line_number, value_start);
                }

                value_already_sent_for_line = true;
            } else if byte == b'\n' || byte == b'\0' {
                line_number += 1;
                value_already_sent_for_line = false;

                if line_number > 2 {
                    break;
                }
            }
        }
    }
}
