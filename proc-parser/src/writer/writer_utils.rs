use crate::procfs::loadavg::ProcLoadAvgFields;
use crate::procfs::meminfo::ProcMemInfoFields;
use crate::procfs::pressure::pressure_util::ProcProcessGeneralFields;
use crate::procfs::stat::ProcStatFields;
use crate::procfs::uptime::ProcUptimeFields;
use crate::procfs::vmstat::ProcVmStatFields;
use crate::sysfs::diskstats::DiskStatsValues;

#[derive(Debug)]
pub enum WriteDataMsg {
    LoadAvg(WriteData<ProcLoadAvgFields>),
    MemInfo(WriteData<ProcMemInfoFields>),
    Stat(WriteData<ProcStatFields>),
    Uptime(WriteData<ProcUptimeFields>),
    VmStat(WriteData<ProcVmStatFields>),
    DiskStats(WriteData<DiskStatsValues>),
    Pressure(WriteData<ProcProcessGeneralFields>),
}

#[derive(Debug)]
pub struct WriteData<T> {
    pub time: i64,
    pub data: T,
    pub pid: Option<u64>,
    pub variant: Option<&'static str>,
}

impl<T> WriteData<T> {
    pub fn new(time: i64, data: T, pid: Option<u64>, variant: Option<&'static str>) -> Self {
        Self {
            time: time,
            data: data,
            pid: pid,
            variant: variant,
        }
    }
}
