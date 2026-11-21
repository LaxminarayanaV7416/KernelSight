use super::cgroupfs::CgroupV2CgroupEventsConfig;
use super::cgroupfs::CgroupV2CpuPressureConfig;
use super::cgroupfs::CgroupV2CpuStatConfig;
use super::cgroupfs::CgroupV2IoPressureConfig;
use super::cgroupfs::CgroupV2MemoryCurrentConfig;
use super::cgroupfs::CgroupV2MemoryEventConfig;
use super::cgroupfs::CgroupV2MemoryPeakConfig;
use super::cgroupfs::CgroupV2MemoryPressureConfig;
use super::cgroupfs::CgroupV2MemoryStatConfig;
use super::cgroupfs::CgroupV2PidsCurrentConfig;
use super::cgroupfs::CgroupV2PidsEventsConfig;
use super::cgroupfs::CgroupV2PidsPeakConfig;
use super::proc_pidfs::ProcPIDStatusConfig;
use super::proc_pidfs::ProcPidfsSchedstatConfig;
use super::proc_pidfs::ProcPidfsStatConfig;
use super::proc_pidfs::ProcPidfsStatmConfig;
use super::procfs::ProcDiskStatsConfig;
use super::procfs::ProcLoadAvgConfig;
use super::procfs::ProcMemInfoConfig;
use super::procfs::ProcPressureConfig;
use super::procfs::ProcStatConfig;
use super::procfs::ProcUptimeConfig;
use super::procfs::ProcVmstatConfig;
use serde::Deserialize;
use std::collections::HashMap;

pub trait ProcFSConfigTrait {
    fn get_hashmap(&self) -> HashMap<usize, bool>;
    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)>;
    fn is_enabled(&self) -> bool;
}

#[derive(Debug, Deserialize)]
pub struct ProcFSConfig<T> {
    pub allow: bool,
    pub heart_beat: u64,
    pub fields: T,
}

impl<T: Default> Default for ProcFSConfig<T> {
    fn default() -> Self {
        Self {
            allow: true,
            heart_beat: 1000,
            fields: T::default(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub thread_pool_count: u64,
    pub heart_beat: u64,
    pub pids_scan_heart_beat: u64,
    pub gpu_scan_heart_beat: u64,
    pub debug: bool,
    pub procfs: ProcFS,
    pub cgroupfs: CgroupV2,
    pub cgroup_patterns: Vec<String>,
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
