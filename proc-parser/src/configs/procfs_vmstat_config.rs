use serde::Deserialize;
use std::collections::HashMap;

// its the load avg config
// its from the /proc/loadavg

#[derive(Debug, Default, Deserialize)]
pub struct ProcLoadAvgConfig {
    pub load_avg_1m: bool,
    pub load_avg_5m: bool,
    pub load_avg_15m: bool,
    pub num_runnable: bool,
    pub num_total: bool,
    pub last_pid: bool,
}

impl ProcLoadAvgConfig {
    pub fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.load_avg_1m),
            (2, self.load_avg_5m),
            (3, self.load_avg_15m),
            (4, self.num_runnable),
            (5, self.num_total),
            (6, self.last_pid),
        ])
    }
}
