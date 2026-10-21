use crate::common::file_reader::ProcFileReader;
use std::collections::HashMap;

/*
/proc/pid/statm
Provides information about memory usage, measured in pages.
The columns are:
size       (1) total program size
            (same as VmSize in /proc/pid/status)
resident   (2) resident set size
            (inaccurate; same as VmRSS in /proc/pid/status)
shared     (3) number of resident shared pages
            (i.e., backed by a file)
            (inaccurate; same as RssFile+RssShmem in
            /proc/pid/status)
text       (4) text (code)
lib        (5) library (unused since Linux 2.6; always 0)
data       (6) data + stack
dt         (7) dirty pages (unused since Linux 2.6; always 0)

Some of these values are inaccurate because of a kernel-
internal scalability optimization.  If accurate values are
required, use /proc/pid/smaps or /proc/pid/smaps_rollup
instead, which are much slower but provide accurate,
detailed information.
 */

#[derive(Debug)]
pub struct ProcFSPIDStatmReader {
    reader: ProcFileReader<512>,
    values: ProcPIDStatmFields,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcPIDStatmFields {
    pub size: u64,
    pub resident: u64,
    pub shared: u64,
    pub text: u64,
    pub lib: u64,
    pub data: u64,
    pub dirty: u64,
}

impl ProcFSPIDStatmReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: ProcPIDStatmFields::default(),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &ProcPIDStatmFields {
        &self.values
    }

    #[inline]
    fn set_field(&mut self, field: usize, value: u64) {
        match field {
            1 => self.values.size = value,
            2 => self.values.resident = value,
            3 => self.values.shared = value,
            4 => self.values.text = value,
            5 => self.values.lib = value,
            6 => self.values.data = value,
            7 => self.values.dirty = value,
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        if field_filter.is_empty() {
            return;
        }
        let mut field = 1usize;
        let mut selected = field_filter.get(&field).copied().unwrap_or(false);
        let mut token_started = false;
        let mut value = 0u64;
        let mut digit_count = 0usize;
        let mut valid = true;

        for index in 0..self.reader.buffer_len {
            // Copy the byte so no immutable borrow into self survives when
            // set_field() takes &mut self.
            let byte = self.reader.buffer[index];
            if matches!(byte, b' ' | b'\t' | b'\n' | b'\0') {
                if token_started {
                    if selected {
                        let parsed_value = if valid && digit_count > 0 { value } else { 0 };
                        self.set_field(field, parsed_value);
                    }
                    field += 1;
                    // /proc/<pid>/statm contains exactly seven fields.
                    if field > 7 {
                        break;
                    }
                    selected = field_filter.get(&field).copied().unwrap_or(false);
                    token_started = false;
                    value = 0;
                    digit_count = 0;
                    valid = true;
                }
                continue;
            }
            token_started = true;
            // Continue scanning for the delimiter, but avoid numeric work when
            // the current field is disabled.
            if !selected || !valid {
                continue;
            }
            match byte {
                b'0'..=b'9' => {
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
                _ => {
                    valid = false;
                }
            }
        }
        // Handle field 7 if the file does not end with newline/NUL.
        if token_started && field <= 7 && selected {
            let parsed_value = if valid && digit_count > 0 { value } else { 0 };

            self.set_field(field, parsed_value);
        }
    }
}
