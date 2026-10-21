use crate::common::file_reader::ProcFileReader;

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
                    // pids.current contains exactly one value.
                    break;
                }
                _ => {
                    valid = false;
                    break;
                }
            }
        }
        self.values.pids = if valid && digit_count > 0 { value } else { 0 };
    }
}
