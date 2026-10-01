use serde::Deserialize;
use std::collections::HashMap;

// its the load avg config
// its from the /proc/pressure/cpu

#[derive(Debug, Default, Deserialize)]
pub struct ProcPressureCPUConfig {
    pub some_avg_10: bool,
    pub some_avg_60: bool,
    pub some_avg_300: bool,
    pub some_total: bool,
    pub full_avg_10: bool,
    pub full_avg_60: bool,
    pub full_avg_300: bool,
    pub full_total: bool,
}

impl ProcPressureCPUConfig {
    pub fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.some_avg_10),
            (2, self.some_avg_60),
            (3, self.some_avg_300),
            (4, self.some_total),
            (5, self.full_avg_10),
            (6, self.full_avg_60),
            (7, self.full_avg_300),
            (8, self.full_total),
        ])
    }
}
