use super::super::config::ProcFSConfig;
use super::super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

// its the load avg config
// its from the /proc/pressure/cpu

#[derive(Debug, Deserialize)]
pub struct CgroupV2MemoryPressureConfigFields {
    pub some_avg_10: bool,
    pub some_avg_60: bool,
    pub some_avg_300: bool,
    pub some_total: bool,
    pub full_avg_10: bool,
    pub full_avg_60: bool,
    pub full_avg_300: bool,
    pub full_total: bool,
}

pub type CgroupV2MemoryPressureConfig = ProcFSConfig<CgroupV2MemoryPressureConfigFields>;

impl ProcFSConfigTrait for CgroupV2MemoryPressureConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.some_avg_10),
            (2, self.fields.some_avg_60),
            (3, self.fields.some_avg_300),
            (4, self.fields.some_total),
            (5, self.fields.full_avg_10),
            (6, self.fields.full_avg_60),
            (7, self.fields.full_avg_300),
            (8, self.fields.full_total),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow
            & self.fields.some_avg_10
            & self.fields.some_avg_60
            & self.fields.some_avg_300
            & self.fields.some_total
            & self.fields.full_avg_10
            & self.fields.full_avg_60
            & self.fields.full_avg_300
            & self.fields.full_total
    }
}

impl Default for CgroupV2MemoryPressureConfigFields {
    fn default() -> Self {
        Self {
            some_avg_10: true,
            some_avg_60: true,
            some_avg_300: true,
            some_total: true,
            full_avg_10: true,
            full_avg_60: true,
            full_avg_300: true,
            full_total: true,
        }
    }
}
