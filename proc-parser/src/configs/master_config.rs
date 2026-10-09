use super::procfs_diskstats_config::ProcDiskStatsConfig;
use super::procfs_loadavg_config::ProcLoadAvgConfig;
use super::procfs_meminfo_config::ProcMemInfoConfig;
use super::procfs_pressure_config::ProcPressureConfig;
use super::procfs_stat_config::ProcStatConfig;
use super::procfs_uptime_config::ProcUptimeConfig;
use super::procfs_vmstat_config::ProcVmstatConfig;
use serde::Deserialize;
use std::collections::HashMap;

pub trait ProcFSConfigTrait {
    fn get_hashmap(&self) -> HashMap<usize, bool>;
    fn get_field_string_to_struct_ids(&self) -> HashMap<&str, (usize, bool)>;
}

#[derive(Debug, Default, Deserialize)]
pub struct ProcFSConfig<T> {
    pub allow: bool,
    pub heart_beat: u64,
    pub fields: T,
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub thread_pool_count: u64,
    pub procfs: ProcFS,
}

#[derive(Debug, Default, Deserialize)]
pub struct ProcFS {
    pub uptime: ProcUptimeConfig,
    pub vmstat: ProcVmstatConfig,
    pub stat: ProcStatConfig,
    pub loadavg: ProcLoadAvgConfig,
    pub meminfo: ProcMemInfoConfig,
    pub diskstats: ProcDiskStatsConfig,
    pub pressure: ProcFSPressure,
    // pub pid: ProcFSPID,
}

#[derive(Debug, Default, Deserialize)]
pub struct ProcFSPressure {
    pub io: ProcPressureConfig,
    pub memory: ProcPressureConfig,
    pub cpu: ProcPressureConfig,
    pub irq: ProcPressureConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct ProcFSPID {
    // pub schedstat: ProcFSConfig<T>,
    // pub stat: ProcFSConfig<T>,
    // pub uptime: ProcFSConfig<T>,
}
