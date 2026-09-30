use serde::Deserialize;
use std::collections::HashMap;

// its the comm config
// its from the /proc/<PID>/comm

#[derive(Debug, Default, Deserialize)]
pub struct ProcPidfsCommConfig {
    pub comm: bool,
}

impl ProcPidfsCommConfig {
    pub fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([(1, self.comm)])
    }
}
