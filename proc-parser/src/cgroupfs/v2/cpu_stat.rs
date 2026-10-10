use crate::common::cgroup_constants::CGROUP_V2_MOUNT_PATH;
use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::line_tracker;
use crate::common::parser_utils::parse_u64_swar;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

/*
populated
    1 if the cgroup or its descendants contains any live processes; otherwise, 0.
frozen
    1 if the cgroup is frozen; otherwise, 0.
 */

pub struct CgroupFSV2CPUStatReader {
    reader: ProcFileReader<256>,
    values: CgroupFSV2CPUStatFields,
    lines_map: Option<HashMap<usize, usize>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CgroupFSV2CPUStatFields {
    pub usage_usec: u64,
    pub user_usec: u64,
    pub system_usec: u64,
    pub nice_usec: u64,
    pub core_sched_force_idle_usec: u64,
    pub nr_periods: u64,
    pub nr_throttled: u64,
    pub throttled_usec: u64,
    pub nr_bursts: u64,
    pub burst_usec: u64,
}

impl CgroupFSV2CPUStatReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: CgroupFSV2CPUStatFields::default(),
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

    pub fn values(&self) -> &CgroupFSV2CPUStatFields {
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
            1 => self.values.usage_usec = value,
            2 => self.values.user_usec = value,
            3 => self.values.system_usec = value,
            4 => self.values.nice_usec = value,
            5 => self.values.core_sched_force_idle_usec = value,
            6 => self.values.nr_periods = value,
            7 => self.values.nr_throttled = value,
            8 => self.values.throttled_usec = value,
            9 => self.values.nr_bursts = value,
            10 => self.values.burst_usec = value,
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

pub fn parse_cgroup_v2_cgroup_cpu_stat(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
    cgroup_name: &str,
) -> JoinHandle<()> {
    let cgroup_event_path = Path::new(CGROUP_V2_MOUNT_PATH)
        .join("system.slice")
        .join(cgroup_name)
        .join("cpu.stat");
    let cgroup_event_temp_reader =
        CgroupFSV2CPUStatReader::new(&cgroup_event_path.to_str().unwrap(), is_cachable, is_root);
    let mut cgroup_event_reader = match cgroup_event_temp_reader {
        Ok(reader) => Box::new(reader),
        Err(e) => panic!("Failed to create loadavg reader: {}", e),
    };
    // TODO: from here get the config which have HashMap<&str, (usize, bool)>
    // Just use the method get_field_string_to_struct_ids
    // let filter_map = ProcMemInfoConfig.get_field_string_to_struct_ids()
    let filter_map = HashMap::from([
        ("usage_usec", (1, true)),
        ("user_usec", (2, true)),
        ("system_usec", (3, true)),
        ("nice_usec", (4, true)),
        ("core_sched.force_idle_usec", (5, true)),
        ("nr_periods", (6, true)),
        ("nr_throttled", (7, true)),
        ("throttled_usec", (8, true)),
        ("nr_bursts", (9, true)),
        ("burst_usec", (10, true)),
    ]);
    let thread_handle = thread::spawn(move || {
        while !signal_hook.load(Ordering::Relaxed) {
            let read_status = cgroup_event_reader.read_and_parse(&filter_map);
            if read_status {
                println!("Parsed: {:?}", cgroup_event_reader.values());
                println!("=====================================");
                thread::sleep(std::time::Duration::from_millis(monitoring_heartbeat));
            } else {
                println!("Failed to read meminfo, ProcFS File closed!");
                break;
            }
        }
    });
    println!("Killed LoadAvg thread!!");
    thread_handle
}
