use crate::common::cgroup_constants::CGROUP_V2_MOUNT_PATH;
use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::parse_u64_swar;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

/*
Contains the pids as single line in the cgroup.procs file
 */

pub struct CgroupFSV2PidsCurrentReader {
    reader: ProcFileReader<256, ()>,
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

pub fn parse_cgroup_v2_pids_current(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
    cgroup_name: &str,
) -> JoinHandle<()> {
    let cgroup_event_path = Path::new(CGROUP_V2_MOUNT_PATH)
        .join("system.slice")
        .join(cgroup_name)
        .join("pids.current");
    let cgroup_event_temp_reader = CgroupFSV2PidsCurrentReader::new(
        &cgroup_event_path.to_str().unwrap(),
        is_cachable,
        is_root,
    );
    let mut cgroup_event_reader = match cgroup_event_temp_reader {
        Ok(reader) => Box::new(reader),
        Err(e) => panic!("Failed to create loadavg reader: {}", e),
    };
    let thread_handle = thread::spawn(move || {
        while !signal_hook.load(Ordering::Relaxed) {
            let read_status = cgroup_event_reader.read_and_parse();
            if !read_status {
                println!("Failed to read loadavg, ProcFS File closed!");
                break;
            }
            println!("Parsed: {:?}", cgroup_event_reader.values());
            println!("=====================================");
            thread::sleep(std::time::Duration::from_millis(monitoring_heartbeat));
        }
        println!("Killed LoadAvg thread!!");
    });
    thread_handle
}
