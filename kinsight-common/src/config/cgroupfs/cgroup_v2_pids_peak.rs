use super::super::config::ProcFSConfig;
use super::super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CgroupV2PidsPeakConfigFields {
    pub pids: bool,
}

impl Default for CgroupV2PidsPeakConfigFields {
    fn default() -> Self {
        Self { pids: true }
    }
}

pub type CgroupV2PidsPeakConfig = ProcFSConfig<CgroupV2PidsPeakConfigFields>;

impl ProcFSConfigTrait for CgroupV2PidsPeakConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([(1, self.fields.pids)])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow
    }
}
