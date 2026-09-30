use serde::Deserialize;
use std::collections::HashMap;

// its the load avg config
// its from the /proc/loadavg

#[derive(Debug, Default, Deserialize)]
pub struct ProcPidfsStatmConfig {
    pub size: bool,
    pub resident: bool,
    pub shared: bool,
    pub text: bool,
    pub lib: bool,
    pub data: bool,
    pub dirty: bool,
}

impl ProcPidfsStatmConfig {
    pub fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.size),
            (2, self.resident),
            (3, self.shared),
            (4, self.text),
            (5, self.lib),
            (6, self.data),
            (7, self.dirty),
        ])
    }
}
