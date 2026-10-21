use crate::common::file_reader::ProcFileReader;
/*
Contains the pids as single line in the cgroup.procs file
 */

pub struct CgroupFSV2CgroupProcsReader {
    reader: ProcFileReader<4192>,
    values: CgroupFSV2CgroupProcsFields,
}

#[derive(Debug, Default, Clone)]
pub struct CgroupFSV2CgroupProcsFields {
    pub pids: Vec<u64>,
}

impl CgroupFSV2CgroupProcsReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: CgroupFSV2CgroupProcsFields::default(),
        })
    }

    pub fn read_and_parse(&mut self) -> bool {
        let read_status = self.reader.read();
        if read_status {
            // clear the pids vector and parse the buffer
            self.values.pids.clear();
            self.parse_buffer();
        }
        read_status
    }

    pub fn values(&self) -> &CgroupFSV2CgroupProcsFields {
        &self.values
    }

    fn parse_buffer(&mut self) {
        let mut pid_started = false;
        let mut pid = 0u64;
        let mut digit_count = 0usize;
        let mut valid = true;

        for index in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[index];
            if matches!(byte, b'\n' | b'\0') {
                if pid_started && valid && digit_count > 0 {
                    self.values.pids.push(pid);
                }
                pid_started = false;
                pid = 0;
                digit_count = 0;
                valid = true;
                continue;
            }
            if matches!(byte, b' ' | b'\t' | b'\r') {
                if pid_started {
                    valid = false;
                }
                continue;
            }
            pid_started = true;
            if !valid {
                continue;
            }
            match byte {
                b'0'..=b'9' => {
                    digit_count += 1;

                    if digit_count > 20 {
                        valid = false;
                        continue;
                    }
                    let digit = u64::from(byte - b'0');
                    pid = match pid
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
                _ => {
                    valid = false;
                }
            }
        }

        if pid_started && valid && digit_count > 0 {
            self.values.pids.push(pid);
        }
    }
}
