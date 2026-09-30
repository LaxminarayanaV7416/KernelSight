use serde::Deserialize;
use std::collections::HashMap;

// its the cgroup config
// its from the /proc/<PID>/cgroup

#[derive(Debug, Default, Deserialize)]
pub struct ProcPidfsCgroupConfig {
    pub cgroup_name: bool,
}

impl ProcPidfsCgroupConfig {
    pub fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([(1, self.cgroup_name)])
    }
}
