use super::master_config::ProcFSConfig;
use super::master_config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

// its the load avg config
// its from the /proc/loadavg

#[derive(Debug, Default, Deserialize)]
pub struct ProcStatConfigFields {
    pub cpu: bool,
    pub cpu_cores: bool,
    pub intr: bool,
    pub ctxt: bool,
    pub btime: bool,
    pub processes: bool,
    pub processes_running: bool,
    pub processes_blocked: bool,
    pub softirq: bool,
}

pub type ProcStatConfig = ProcFSConfig<ProcStatConfigFields>;

impl ProcFSConfigTrait for ProcStatConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.cpu),
            (2, self.fields.cpu_cores),
            (3, self.fields.intr),
            (4, self.fields.ctxt),
            (5, self.fields.btime),
            (6, self.fields.processes),
            (7, self.fields.processes_running),
            (8, self.fields.processes_blocked),
            (9, self.fields.softirq),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&str, (usize, bool)> {
        HashMap::new()
    }
}
