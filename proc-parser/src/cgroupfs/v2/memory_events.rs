use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::line_tracker;
use crate::common::parser_utils::parse_u64_swar;
use std::collections::HashMap;
/*
populated
    1 if the cgroup or its descendants contains any live processes; otherwise, 0.
frozen
    1 if the cgroup is frozen; otherwise, 0.
 */

pub struct CgroupFSV2MemoryEventReader {
    reader: ProcFileReader<256>,
    values: CgroupFSV2MemoryEventFields,
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
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: CgroupFSV2MemoryEventFields::default(),
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

    fn set_field(&mut self, field: usize, start: usize, end: usize) {
        let bytes = &self.reader.buffer[start..end];
        let mut seen_colon = false;
        let mut digit_start: Option<usize> = None;
        let mut digit_end: usize = 0;
        for (i, &byte) in bytes.iter().enumerate() {
            if !seen_colon {
                if byte == b' ' {
                    seen_colon = true;
                }
                continue;
            }
            match byte {
                b'0'..=b'9' => {
                    if digit_start.is_none() {
                        digit_start = Some(i);
                    }

                    digit_end = i + 1;
                }
                _ => {
                    if digit_start.is_some() {
                        break;
                    }
                }
            }
        }
        let Some(digit_start) = digit_start else {
            return;
        };
        let value_bytes = &bytes[digit_start..digit_end];
        let value = parse_u64_swar(value_bytes).unwrap_or_default();
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
        // first step is to track line numbers for the fields we care about
        // this is used to map field numbers to line numbers in the buffer
        // once this is done we only parse the fields we care about
        // the complicated logic is explained in the parser_utils line_tracker function
        if self.lines_map.is_none() {
            self.lines_map = Some(line_tracker(
                &self.reader.buffer[..self.reader.buffer_len],
                field_filter,
                b' ',
            ));
        }
        let mut parser_line_number = 0usize;
        let mut start: Option<usize> = None;
        for i in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[i];

            if byte == b'\n' || byte == b'\0' {
                if let Some(s) = start.take() {
                    parser_line_number += 1;
                    // now get the field number from the line number
                    let field_number = self
                        .lines_map
                        .as_ref()
                        .unwrap()
                        .get(&parser_line_number)
                        .copied()
                        .unwrap_or(0);

                    if field_number > 0 {
                        self.set_field(field_number, s, i);
                    }
                }
            } else if start.is_none() {
                start = Some(i);
            }
        }
        if let Some(s) = start {
            parser_line_number += 1;
            let field_number = self
                .lines_map
                .as_ref()
                .unwrap()
                .get(&parser_line_number)
                .copied()
                .unwrap_or(0);

            if field_number > 0 {
                self.set_field(field_number, s, self.reader.buffer_len);
            }
        }
    }
}
