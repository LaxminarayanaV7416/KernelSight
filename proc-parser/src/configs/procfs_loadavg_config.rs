use super::config::ProcFSConfig;
use super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

// its the load avg config
// its from the /proc/loadavg

#[derive(Debug, Default, Deserialize)]
pub struct ProcLoadAvgConfigFields {
    pub load_avg_1m: bool,
    pub load_avg_5m: bool,
    pub load_avg_15m: bool,
    pub num_runnable: bool,
    pub num_total: bool,
    pub last_pid: bool,
}

pub type ProcLoadAvgConfig = ProcFSConfig<ProcLoadAvgConfigFields>;

impl ProcFSConfigTrait for ProcLoadAvgConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.load_avg_1m),
            (2, self.fields.load_avg_5m),
            (3, self.fields.load_avg_15m),
            (4, self.fields.num_runnable),
            (5, self.fields.num_total),
            (6, self.fields.last_pid),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow
            & self.fields.load_avg_1m
            & self.fields.load_avg_5m
            & self.fields.load_avg_15m
            & self.fields.num_runnable
            & self.fields.num_total
            & self.fields.last_pid
    }
}
