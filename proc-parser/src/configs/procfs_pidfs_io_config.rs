use serde::Deserialize;
use std::collections::HashMap;

// its the io config of the /proc/<PID>/io
// its from the /proc/<PID>/io

#[derive(Debug, Default, Deserialize)]
pub struct ProcPidfsIoConfig {
    pub rchar: bool,
    pub wchar: bool,
    pub syscr: bool,
    pub syscw: bool,
    pub read_bytes: bool,
    pub write_bytes: bool,
    pub cancelled_write_bytes: bool,
}

impl ProcPidfsIoConfig {
    pub fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.rchar),
            (2, self.wchar),
            (3, self.syscr),
            (4, self.syscw),
            (5, self.read_bytes),
            (6, self.write_bytes),
            (7, self.cancelled_write_bytes),
        ])
    }
}
