use super::cgroupfs::cgroup_v2_cgroup_events::CgroupV2CgroupEventsConfig;
use super::cgroupfs::cgroup_v2_cpu_pressure::CgroupV2CpuPressureConfig;
use super::cgroupfs::cgroup_v2_cpu_stat::CgroupV2CpuStatConfig;
use super::cgroupfs::cgroup_v2_io_pressure::CgroupV2IoPressureConfig;
use super::cgroupfs::cgroup_v2_memory_current::CgroupV2MemoryCurrentConfig;
use super::cgroupfs::cgroup_v2_memory_events::CgroupV2MemoryEventConfig;
use super::cgroupfs::cgroup_v2_memory_peak::CgroupV2MemoryPeakConfig;
use super::cgroupfs::cgroup_v2_memory_pressure::CgroupV2MemoryPressureConfig;
use super::cgroupfs::cgroup_v2_memory_stat::CgroupV2MemoryStatConfig;
use super::cgroupfs::cgroup_v2_pids_current::CgroupV2PidsCurrentConfig;
use super::cgroupfs::cgroup_v2_pids_events::CgroupV2PidsEventsConfig;
use super::cgroupfs::cgroup_v2_pids_peak::CgroupV2PidsPeakConfig;
use super::procfs_diskstats_config::ProcDiskStatsConfig;
use super::procfs_loadavg_config::ProcLoadAvgConfig;
use super::procfs_meminfo_config::ProcMemInfoConfig;
use super::procfs_pidfs_schedstat_config::ProcPidfsSchedstatConfig;
use super::procfs_pidfs_stat_config::ProcPidfsStatConfig;
use super::procfs_pidfs_statm_config::ProcPidfsStatmConfig;
use super::procfs_pidfs_status_config::ProcPIDStatusConfig;
use super::procfs_pressure_config::ProcPressureConfig;
use super::procfs_stat_config::ProcStatConfig;
use super::procfs_uptime_config::ProcUptimeConfig;
use super::procfs_vmstat_config::ProcVmstatConfig;
use serde::Deserialize;
use std::collections::HashMap;

pub trait ProcFSConfigTrait {
    fn get_hashmap(&self) -> HashMap<usize, bool>;
    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)>;
    fn is_enabled(&self) -> bool;
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
    pub heart_beat: u64,
    pub pids_scan_heart_beat: u64,
    pub debug: bool,
    pub procfs: ProcFS,
    pub cgroupfs: CgroupV2,
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
    pub pid: ProcFSPID,
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
    pub schedstat: ProcPidfsSchedstatConfig,
    pub stat: ProcPidfsStatConfig,
    pub statm: ProcPidfsStatmConfig,
    pub status: ProcPIDStatusConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct CgroupV2 {
    pub cgroup_events: CgroupV2CgroupEventsConfig,
    pub cpu_pressure: CgroupV2CpuPressureConfig,
    pub cpu_stat: CgroupV2CpuStatConfig,
    pub io_pressure: CgroupV2IoPressureConfig,
    pub memory_current: CgroupV2MemoryCurrentConfig,
    pub memory_events: CgroupV2MemoryEventConfig,
    pub memory_peak: CgroupV2MemoryPeakConfig,
    pub memory_pressure: CgroupV2MemoryPressureConfig,
    pub memory_stat: CgroupV2MemoryStatConfig,
    pub pids_current: CgroupV2PidsCurrentConfig,
    pub pids_events: CgroupV2PidsEventsConfig,
    pub pids_peak: CgroupV2PidsPeakConfig,
}
