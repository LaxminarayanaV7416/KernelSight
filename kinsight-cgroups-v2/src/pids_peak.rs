use kinsight_common::{KinsightDataParserTrait, KinsightReader, Result};
use std::mem::swap;
use std::path::PathBuf;

/*
Contains the pids as single line in the cgroup.procs file
*/

#[derive(Debug, Default, Clone, Copy)]
pub struct CgroupFSV2PidsPeakFields {
    pub pids: u64,
}

pub struct CgroupFSV2PidsPeakReader {
    reader: KinsightReader<256>,
    values: Box<CgroupFSV2PidsPeakFields>,
    previous_values: Box<CgroupFSV2PidsPeakFields>,
}

impl CgroupFSV2PidsPeakReader {
    pub fn new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            reader: KinsightReader::new(path)?,
            values: Box::new(CgroupFSV2PidsPeakFields::default()),
            previous_values: Box::new(CgroupFSV2PidsPeakFields::default()),
        })
    }
}

impl KinsightDataParserTrait<CgroupFSV2PidsPeakFields, usize> for CgroupFSV2PidsPeakReader {
    fn read_and_parse(&mut self, nothing: &usize) -> bool {
        let read_status = self.reader.read();
        if read_status {
            swap(&mut self.values, &mut self.previous_values);
            self.parse_buffer(&nothing);
        }
        read_status
    }

    fn values(&self) -> &CgroupFSV2PidsPeakFields {
        &self.values
    }

    fn prev_values(&self) -> &CgroupFSV2PidsPeakFields {
        &self.previous_values
    }

    fn parse_buffer(&mut self, _: &usize) {
        let mut value = 0u64;
        let mut digit_count = 0usize;
        let mut value_started = false;
        let mut valid = true;
        for index in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[index];
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
                b' ' | b'\t' | b'\r' if !value_started => {
                    // Tolerate leading whitespace.
                }
                b'\n' | b'\0' | b' ' | b'\t' | b'\r' if value_started => {
                    // pids.peak contains one value.
                    break;
                }
                _ => {
                    valid = false;
                    break;
                }
            }
        }

        self.values.pids = if valid && value_started && digit_count > 0 {
            value
        } else {
            0
        };
    }
}
