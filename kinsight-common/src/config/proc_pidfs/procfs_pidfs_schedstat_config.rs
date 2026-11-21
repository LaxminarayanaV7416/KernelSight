use super::super::config::ProcFSConfig;
use super::super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

// its the schedstat config
// its from the /proc/<PID>/schedstat

#[derive(Debug, Deserialize)]
pub struct ProcPidfsSchedstatConfigFields {
    pub time_on_cpu: bool,
    pub time_waiting: bool,
    pub timeslices_run_count: bool,
}

impl Default for ProcPidfsSchedstatConfigFields {
    fn default() -> Self {
        Self {
            time_on_cpu: true,
            time_waiting: true,
            timeslices_run_count: true,
        }
    }
}

pub type ProcPidfsSchedstatConfig = ProcFSConfig<ProcPidfsSchedstatConfigFields>;

impl ProcFSConfigTrait for ProcPidfsSchedstatConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.time_on_cpu),
            (2, self.fields.time_waiting),
            (3, self.fields.timeslices_run_count),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow
            & self.fields.time_on_cpu
            & self.fields.time_waiting
            & self.fields.timeslices_run_count
    }
}
