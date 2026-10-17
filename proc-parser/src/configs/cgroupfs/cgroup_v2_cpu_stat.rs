use super::super::config::ProcFSConfig;
use super::super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

// its the load avg config
// its from the /proc/pressure/cpu

#[derive(Debug, Default, Deserialize)]
pub struct CgroupV2CpuStatConfigFields {
    pub usage_usec: bool,
    pub user_usec: bool,
    pub system_usec: bool,
    pub nice_usec: bool,
    pub core_sched_force_idle_usec: bool,
    pub nr_periods: bool,
    pub nr_throttled: bool,
    pub throttled_usec: bool,
    pub nr_bursts: bool,
    pub burst_usec: bool,
}

pub type CgroupV2CpuStatConfig = ProcFSConfig<CgroupV2CpuStatConfigFields>;

impl ProcFSConfigTrait for CgroupV2CpuStatConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.usage_usec),
            (2, self.fields.user_usec),
            (3, self.fields.system_usec),
            (4, self.fields.nice_usec),
            (5, self.fields.core_sched_force_idle_usec),
            (6, self.fields.nr_periods),
            (7, self.fields.nr_throttled),
            (8, self.fields.throttled_usec),
            (9, self.fields.nr_bursts),
            (10, self.fields.burst_usec),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow
            & self.fields.usage_usec
            & self.fields.user_usec
            & self.fields.system_usec
            & self.fields.nice_usec
            & self.fields.core_sched_force_idle_usec
            & self.fields.nr_periods
            & self.fields.nr_throttled
            & self.fields.throttled_usec
            & self.fields.nr_bursts
            & self.fields.burst_usec
    }
}
