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
