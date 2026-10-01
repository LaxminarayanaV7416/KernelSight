use serde::Deserialize;
use std::collections::HashMap;

// its the schedstat config
// its from the /proc/<PID>/schedstat

#[derive(Debug, Default, Deserialize)]
pub struct ProcPidfsSchedstatConfig {
    pub time_on_cpu: bool,
    pub time_waiting: bool,
    pub timeslices_run_count: bool,
}

impl ProcPidfsSchedstatConfig {
    pub fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.time_on_cpu),
            (2, self.time_waiting),
            (3, self.timeslices_run_count),
        ])
    }
}
