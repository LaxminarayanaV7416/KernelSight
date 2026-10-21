use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::parse_u64_swar;

/*
Contains the pids as single line in the cgroup.procs file
 */

pub struct CgroupFSV2PidsCurrentReader {
    reader: ProcFileReader<256>,
    values: CgroupFSV2PidsCurrentFields,
}

#[derive(Debug, Default, Clone)]
pub struct CgroupFSV2PidsCurrentFields {
    pub pids: u64,
}

impl CgroupFSV2PidsCurrentReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: CgroupFSV2PidsCurrentFields::default(),
        })
    }

    pub fn read_and_parse(&mut self) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer();
        }
        read_status
    }

    pub fn values(&self) -> &CgroupFSV2PidsCurrentFields {
        &self.values
    }

    fn parse_buffer(&mut self) {
        let mut start: Option<usize> = None;

        for i in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[i];

            if byte == b'\n' || byte == b'\0' {
                if let Some(s) = start.take() {
                    self.values.pids = parse_u64_swar(&self.reader.buffer[s..i]).unwrap_or(0);
                }
            } else if start.is_none() {
                start = Some(i);
            }
        }
    }
}
