use crate::cgroupfs::v2::cgroup_events::CgroupFSV2CgroupEventFields;
use crate::cgroupfs::v2::cgroup_procs::CgroupFSV2CgroupProcsFields;
use crate::cgroupfs::v2::cpu_stat::CgroupFSV2CPUStatFields;
use crate::cgroupfs::v2::memory_current::CgroupFSV2MemoryCurrentFields;
use crate::cgroupfs::v2::memory_events::CgroupFSV2MemoryEventFields;
use crate::cgroupfs::v2::memory_peak::CgroupFSV2MemoryPeakFields;
use crate::cgroupfs::v2::memory_stat::CgroupFSV2MemoryStatFields;
use crate::cgroupfs::v2::pids_current::CgroupFSV2PidsCurrentFields;
use crate::cgroupfs::v2::pids_events::CgroupFSV2PidsEventFields;
use crate::cgroupfs::v2::pids_peak::CgroupFSV2PidsPeakFields;
use crate::gpu::nvidia::{GpuDeviceInfo, GpuDeviceMessage};
use crate::procfs::loadavg::ProcLoadAvgFields;
use crate::procfs::meminfo::ProcMemInfoFields;
use crate::procfs::pidfs::schedstat::ProcPIDSchedStatFields;
use crate::procfs::pidfs::stat::ProcPIDStatFields;
use crate::procfs::pidfs::statm::ProcPIDStatmFields;
use crate::procfs::pidfs::status::ProcPIDStatusFields;
use crate::procfs::pressure_util::ProcProcessGeneralFields;
use crate::procfs::stat::ProcStatFields;
use crate::procfs::uptime::ProcUptimeFields;
use crate::procfs::vmstat::ProcVmStatFields;
use crate::sysfs::diskstats::DiskStatsFields;

#[derive(Debug)]
pub enum WriteDataMsg {
    // ProcFS main files
    LoadAvg(WriteData<ProcLoadAvgFields>),
    MemInfo(WriteData<ProcMemInfoFields>),
    Stat(WriteData<ProcStatFields>),
    Uptime(WriteData<ProcUptimeFields>),
    VmStat(WriteData<ProcVmStatFields>),
    DiskStats(WriteData<DiskStatsFields>),
    Pressure(WriteData<ProcProcessGeneralFields>),
    // ProcFS PID files
    PIDCgroup(WriteData<String>),
    PIDComm(WriteData<String>),
    PIDCmdline(WriteData<String>),
    PIDSchedStat(WriteData<ProcPIDSchedStatFields>),
    PIDStat(WriteData<ProcPIDStatFields>),
    PIDStatm(WriteData<ProcPIDStatmFields>),
    PIDStatus(WriteData<ProcPIDStatusFields>),
}

#[derive(Debug)]
pub struct WriteData<T> {
    pub time: f64,
    pub data: T,
    pub pid: Option<u64>,
    pub variant: Option<&'static str>, // this is used in case of pressure to identify what type it is
                                       // for pressure we are usign generic type so instead of duplicating everythign we are using same
                                       // as per the SOLID principle and passing variant as identifier
}

impl<T> WriteData<T> {
    pub fn new(time: f64, data: T, pid: Option<u64>, variant: Option<&'static str>) -> Self {
        Self {
            time: time,
            data: data,
            pid: pid,
            variant: variant,
        }
    }
}

// -------------------------------------------------

#[derive(Debug)]
pub struct WriteGPUData<T> {
    pub data: T,
}

impl<T> WriteGPUData<T> {
    pub fn new(data: T) -> Self {
        Self { data: data }
    }
}

#[derive(Debug)]
pub enum WriteGPUDataMsg {
    NvidiaDeviceInfo(WriteGPUData<GpuDeviceInfo>),
    NvidiaProcs(WriteGPUData<GpuDeviceMessage>),
}

// -------------------------------------------------

#[derive(Debug)]
pub struct WriteCgroupsData<T> {
    pub time: f64,
    pub data: T,
    pub path: String,
}

impl<T> WriteCgroupsData<T> {
    pub fn new(time: f64, data: T, path: String) -> Self {
        Self {
            time: time,
            data: data,
            path: path,
        }
    }
}

pub enum WriteCgroupsDataMsg {
    CgroupEvent(WriteCgroupsData<CgroupFSV2CgroupEventFields>),
    CgroupProcs(WriteCgroupsData<CgroupFSV2CgroupProcsFields>),
    CpuPressure(WriteCgroupsData<ProcProcessGeneralFields>),
    CpuStat(WriteCgroupsData<CgroupFSV2CPUStatFields>),
    IoPressure(WriteCgroupsData<ProcProcessGeneralFields>),
    MemoryCurrent(WriteCgroupsData<CgroupFSV2MemoryCurrentFields>),
    MemoryEvent(WriteCgroupsData<CgroupFSV2MemoryEventFields>),
    MemoryPeak(WriteCgroupsData<CgroupFSV2MemoryPeakFields>),
    MemoryPressure(WriteCgroupsData<ProcProcessGeneralFields>),
    MemoryStat(WriteCgroupsData<CgroupFSV2MemoryStatFields>),
    PidsCurrent(WriteCgroupsData<CgroupFSV2PidsCurrentFields>),
    PidsEvent(WriteCgroupsData<CgroupFSV2PidsEventFields>),
    PidsPeak(WriteCgroupsData<CgroupFSV2PidsPeakFields>),
}
