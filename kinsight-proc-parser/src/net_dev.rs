use kinsight_common::{KinsightDataParserTrait, KinsightReader, Result};
use std::collections::HashMap;
use std::mem::swap;
use std::path::PathBuf;

/*
The dev pseudo-file contains network device status
information.  This gives the number of received and sent
packets, the number of errors and collisions and other
basic statistics.  These are used by the ifconfig(8)
program to report device status.  The format is:

Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    lo: 2776770   11307    0    0    0     0          0         0  2776770   11307    0    0    0     0       0          0
  eth0: 1215645    2751    0    0    0     0          0         0  1782404    4324    0    0    0   427       0          0
  ppp0: 1622270    5552    1    0    0     0          0         0   354130    5669    0    0    0     0       0          0
  tap0:    7714      81    0    0    0     0          0         0     7714      81    0    0    0     0       0          0
 */

#[derive(Debug)]
pub struct ProcFSNetDevReader {
    reader: KinsightReader<16384>,
    values: Box<ProcNetDevFields>,
    previous_values: Box<ProcNetDevFields>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcNetInterfaceFields {
    pub total_download_bytes: u64,
    pub total_donwload_packets: u64,
    pub bad_packets_downloaded: u64,
    pub recieved_packets_dropped: u64,
    pub total_upload_bytes: u64,
    pub total_upload_packets: u64,
    pub bad_packets_uploaded: u64,
    pub sent_packets_dropped: u64,
}

#[derive(Debug, Default, Clone)]
pub struct ProcNetDevFields {
    pub interfaces: HashMap<String, ProcNetInterfaceFields>,
}

impl ProcFSNetDevReader {
    pub fn new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            reader: KinsightReader::new(path)?,
            values: Box::new(ProcNetDevFields::default()),
            previous_values: Box::new(ProcNetDevFields::default()),
        })
    }

    #[inline]
    fn set_field(
        values: &mut ProcNetInterfaceFields,
        raw_field: usize,
        value: u64,
        field_filter: &HashMap<usize, bool>,
    ) {
        let selected_field = match raw_field {
            1 => 1,  // receive bytes
            2 => 2,  // receive packets
            3 => 3,  // receive errors
            4 => 4,  // receive dropped
            9 => 5,  // transmit bytes
            10 => 6, // transmit packets
            11 => 7, // transmit errors
            12 => 8, // transmit dropped
            _ => return,
        };

        if !field_filter.get(&selected_field).copied().unwrap_or(false) {
            return;
        }

        match selected_field {
            1 => values.total_download_bytes = value,
            2 => values.total_donwload_packets = value,
            3 => values.bad_packets_downloaded = value,
            4 => values.recieved_packets_dropped = value,
            5 => values.total_upload_bytes = value,
            6 => values.total_upload_packets = value,
            7 => values.bad_packets_uploaded = value,
            8 => values.sent_packets_dropped = value,
            _ => {}
        }
    }
}

impl KinsightDataParserTrait<ProcNetDevFields, HashMap<usize, bool>> for ProcFSNetDevReader {
    fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            swap(&mut self.values, &mut self.previous_values);
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    fn values(&self) -> &ProcNetDevFields {
        &self.values
    }

    fn prev_values(&self) -> &ProcNetDevFields {
        &self.previous_values
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        if field_filter.is_empty() {
            return;
        }

        let buffer_len = self.reader.buffer_len;

        if buffer_len == 0 {
            return;
        }

        let mut line_number = 1usize;

        // Interface-name parsing state.
        let mut name_start: Option<usize> = None;
        let mut name_end: Option<usize> = None;
        let mut seen_colon = false;

        // Numeric parsing state.
        let mut raw_field = 0usize;
        let mut token_started = false;
        let mut value = 0u64;
        let mut digit_count = 0usize;
        let mut valid = true;

        // Current interface values. This is initialized from the existing map
        // entry after the interface name is known, allowing disabled fields to
        // preserve their prior values.
        let mut interface_values = ProcNetInterfaceFields::default();

        for index in 0..buffer_len {
            let byte = self.reader.buffer[index];
            let is_line_end = matches!(byte, b'\n' | b'\0');

            // The first two lines are headers.
            if line_number <= 2 {
                if is_line_end {
                    line_number += 1;
                }

                continue;
            }

            if !seen_colon {
                match byte {
                    b':' => {
                        let Some(start) = name_start else {
                            // Malformed line without an interface name.
                            seen_colon = true;
                            valid = false;
                            continue;
                        };

                        name_end = Some(index);
                        seen_colon = true;

                        if let Ok(name) = std::str::from_utf8(&self.reader.buffer[start..index]) {
                            let name = name.trim();

                            interface_values = self
                                .values
                                .interfaces
                                .get(name)
                                .copied()
                                .unwrap_or_default();
                        }
                    }

                    b'\n' | b'\0' => {
                        // Empty or malformed line.
                        line_number += 1;
                        name_start = None;
                    }

                    byte if byte.is_ascii_whitespace() => {
                        // Ignore indentation before the interface name.
                    }

                    _ => {
                        if name_start.is_none() {
                            name_start = Some(index);
                        }
                    }
                }

                continue;
            }

            let is_token_delimiter = byte.is_ascii_whitespace();

            if is_token_delimiter {
                if token_started {
                    raw_field += 1;

                    if valid && digit_count > 0 {
                        Self::set_field(&mut interface_values, raw_field, value, field_filter);
                    } else {
                        // Preserve the old parser convention of converting a
                        // malformed selected value to zero.
                        Self::set_field(&mut interface_values, raw_field, 0, field_filter);
                    }

                    token_started = false;
                    value = 0;
                    digit_count = 0;
                    valid = true;
                }

                if is_line_end {
                    if let (Some(start), Some(end)) = (name_start, name_end) {
                        // Build an owned key only when inserting. Existing keys
                        // are updated without allocating a new String.
                        if let Ok(interface_name) =
                            std::str::from_utf8(&self.reader.buffer[start..end])
                        {
                            let interface_name = interface_name.trim();

                            if !interface_name.is_empty() {
                                if let Some(existing) =
                                    self.values.interfaces.get_mut(interface_name)
                                {
                                    *existing = interface_values;
                                } else {
                                    self.values
                                        .interfaces
                                        .insert(interface_name.to_owned(), interface_values);
                                }
                            }
                        }
                    }

                    line_number += 1;

                    name_start = None;
                    name_end = None;
                    seen_colon = false;

                    raw_field = 0;
                    token_started = false;
                    value = 0;
                    digit_count = 0;
                    valid = true;

                    interface_values = ProcNetInterfaceFields::default();
                }

                continue;
            }

            token_started = true;

            match byte {
                b'0'..=b'9' => {
                    digit_count += 1;

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

        // Handle the final interface line when the input has no trailing
        // newline or NUL.
        if seen_colon {
            if token_started {
                raw_field += 1;

                Self::set_field(
                    &mut interface_values,
                    raw_field,
                    if valid && digit_count > 0 { value } else { 0 },
                    field_filter,
                );
            }

            if let (Some(start), Some(end)) = (name_start, name_end) {
                if let Ok(interface_name) = std::str::from_utf8(&self.reader.buffer[start..end]) {
                    let interface_name = interface_name.trim();

                    if !interface_name.is_empty() {
                        if let Some(existing) = self.values.interfaces.get_mut(interface_name) {
                            *existing = interface_values;
                        } else {
                            self.values
                                .interfaces
                                .insert(interface_name.to_owned(), interface_values);
                        }
                    }
                }
            }
        }
    }
}
