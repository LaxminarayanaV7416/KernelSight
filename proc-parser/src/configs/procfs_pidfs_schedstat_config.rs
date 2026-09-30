use serde::Deserialize;
use std::collections::HashMap;

// its the schedstat config
// its from the /proc/<PID>/schedstat

#[derive(Debug, Default, Deserialize)]
pub struct ProcPidfsSchedstatConfig {
    pub sum_exec_runtime: bool,
    pub run_delay: bool,
    pub pcount: bool,
}

impl ProcPidfsSchedstatConfig {
    pub fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.sum_exec_runtime),
            (2, self.run_delay),
            (3, self.pcount),
        ])
    }
}
