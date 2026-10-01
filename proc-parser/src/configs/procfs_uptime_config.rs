use serde::Deserialize;
use std::collections::HashMap;

// its the Uptime config
// its from the /proc/uptime

#[derive(Debug, Default, Deserialize)]
pub struct ProcUptimeConfig {
    pub uptime: bool,
    pub idle_time: bool,
}

impl ProcUptimeConfig {
    pub fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([(1, self.uptime), (2, self.idle_time)])
    }
}
