use crate::common::file_reader::ProcFileReader;
use crate::common::parser_error::Result;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/*
Contains the pids as single line in the cgroup.procs file
 */

pub struct CgroupFSV2MemoryPeakReader {
    reader: ProcFileReader<256>,
    values: Arc<Mutex<CgroupFSV2MemoryPeakFields>>,
}

#[derive(Debug, Default, Clone)]
pub struct CgroupFSV2MemoryPeakFields {
    pub memory: u64,
}

impl CgroupFSV2MemoryPeakReader {
    pub fn new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            reader: ProcFileReader::new(path)?,
            values: Arc::new(Mutex::new(CgroupFSV2MemoryPeakFields::default())),
        })
    }

    pub fn read_and_parse(&mut self) -> bool {
        let read_status = self.reader.read();
        if read_status {
            if let Ok(mut values) = self.values.lock() {
                self.parse_buffer(&mut *values);
            }
        }
        read_status
    }

    pub fn values(&self) -> &Arc<Mutex<CgroupFSV2MemoryPeakFields>> {
        &self.values
    }

    fn parse_buffer(&self, values: &mut CgroupFSV2MemoryPeakFields) {
        let mut value = 0u64;
        let mut digit_count = 0usize;
        let mut started = false;
        let mut valid = true;
        for index in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[index];
            match byte {
                b'0'..=b'9' => {
                    started = true;
                    digit_count += 1;
                    // Preserve parse_u64_swar's maximum input length.
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
                b' ' | b'\t' | b'\r' if !started => {
                    // Tolerate leading whitespace.
                }
                b'\n' | b'\0' | b' ' | b'\t' | b'\r' if started => {
                    // memory.peak contains one value.
                    break;
                }
                _ => {
                    valid = false;
                    break;
                }
            }
        }
        values.memory = if valid && digit_count > 0 { value } else { 0 };
    }
}
