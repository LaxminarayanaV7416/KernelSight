use kinsight_common::{KinsightDataParserTrait, KinsightReader, Result};
use std::mem::swap;
use std::path::PathBuf;

/*
Contains the pids as single line in the cgroup.procs file
 */

#[derive(Debug, Default, Clone, Copy)]
pub struct CgroupFSV2MemoryCurrentFields {
    pub memory: u64,
}

pub struct CgroupFSV2MemoryCurrentReader {
    reader: KinsightReader<256>,
    values: Box<CgroupFSV2MemoryCurrentFields>,
    previous_values: Box<CgroupFSV2MemoryCurrentFields>,
}

impl CgroupFSV2MemoryCurrentReader {
    pub fn new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            reader: KinsightReader::new(path)?,
            values: Box::new(CgroupFSV2MemoryCurrentFields::default()),
            previous_values: Box::new(CgroupFSV2MemoryCurrentFields::default()),
        })
    }
}

impl KinsightDataParserTrait<CgroupFSV2MemoryCurrentFields, usize>
    for CgroupFSV2MemoryCurrentReader
{
    fn read_and_parse(&mut self, nothing: &usize) -> bool {
        let read_status = self.reader.read();
        if read_status {
            swap(&mut self.values, &mut self.previous_values);
            self.parse_buffer(&nothing);
        }
        read_status
    }

    fn values(&self) -> &CgroupFSV2MemoryCurrentFields {
        &self.values
    }

    fn prev_values(&self) -> &CgroupFSV2MemoryCurrentFields {
        &self.previous_values
    }

    fn parse_buffer(&mut self, _: &usize) {
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
                    // memory.current has exactly one value.
                    break;
                }
                _ => {
                    valid = false;
                    break;
                }
            }
        }
        self.values.memory = if valid && digit_count > 0 { value } else { 0 };
    }
}
