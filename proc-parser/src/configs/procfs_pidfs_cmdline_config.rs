use serde::Deserialize;
use std::collections::HashMap;

// its the cmdline of the process with PID
// its from the /proc/<PID>/cmdline

#[derive(Debug, Default, Deserialize)]
pub struct ProcPidfsCmdlineConfig {
    pub cmdline: bool,
}

impl ProcPidfsCmdlineConfig {
    pub fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.cmdline),
        ])
    }
}
