use crate::common::file_reader::ProcFileReader;
use crate::common::procfs_constants::PROC_FS_ROOT_PATH;
use std::path::Path;

pub struct ProcFSPIDCgroupReader {
    reader: ProcFileReader<256, ()>,
    values: ProcPIDCgroupFields,
}

#[derive(Debug, Default, Clone)]
pub struct ProcPIDCgroupFields {
    pub cgroup_name: String,
}

impl ProcFSPIDCgroupReader {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, true, false)?,
            values: ProcPIDCgroupFields::default(),
        })
    }

    pub fn read_and_parse(&mut self) -> bool {
        let read_status = self.reader.read();
        if read_status {
            let cgroup_name = self.reader.parse_bytes_to_string();
            let cleaned = cgroup_name.replace(&['\n', '\0'], "");
            self.values.cgroup_name = cleaned;
        }
        read_status
    }

    pub fn values(&self) -> &ProcPIDCgroupFields {
        &self.values
    }
}

pub fn parse_procfs_pid_cgroup(pid: usize) -> ProcPIDCgroupFields {
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH)
        .join(pid.to_string())
        .join("cgroup");
    let load_avg_temp_reader = ProcFSPIDCgroupReader::new(&load_avg_path.to_str().unwrap());
    let mut load_avg_reader = match load_avg_temp_reader {
        Ok(reader) => Box::new(reader),
        Err(e) => panic!("Failed to create loadavg reader: {}", e),
    };
    load_avg_reader.read_and_parse();
    load_avg_reader.values().clone()
}
