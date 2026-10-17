use crate::gpu::nvidia::{GpuDeviceInfo, GpuDeviceMessage};
use crate::procfs::loadavg::ProcLoadAvgFields;
use crate::procfs::meminfo::ProcMemInfoFields;
use crate::procfs::pidfs::schedstat::ProcPIDSchedStatFields;
use crate::procfs::pidfs::stat::ProcPIDStatFields;
use crate::procfs::pidfs::statm::ProcPIDStatmFields;
use crate::procfs::pidfs::status::ProcPIDStatusFields;
use crate::procfs::pressure::pressure_util::ProcProcessGeneralFields;
use crate::procfs::stat::ProcStatFields;
use crate::procfs::uptime::ProcUptimeFields;
use crate::procfs::vmstat::ProcVmStatFields;
use crate::sysfs::diskstats::DiskStatsValues;

#[derive(Debug)]
pub enum WriteDataMsg {
    // ProcFS main files
    LoadAvg(WriteData<ProcLoadAvgFields>),
    MemInfo(WriteData<ProcMemInfoFields>),
    Stat(WriteData<ProcStatFields>),
    Uptime(WriteData<ProcUptimeFields>),
    VmStat(WriteData<ProcVmStatFields>),
    DiskStats(WriteData<DiskStatsValues>),
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

#[derive(Debug)]
pub struct WriteCgroupsData<T> {
    pub time: f64,
    pub data: T,
    pub pid: Option<u64>,
    pub variant: Option<&'static str>,
}

impl<T> WriteCgroupsData<T> {
    pub fn new(time: f64, data: T, pid: Option<u64>, variant: Option<&'static str>) -> Self {
        Self {
            time: time,
            data: data,
            pid: pid,
            variant: variant,
        }
    }
}

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
