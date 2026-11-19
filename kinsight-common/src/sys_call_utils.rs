use super::errors::{ProcParserError, Result};
use rustix::thread::sched_getaffinity;
use rustix::time::{ClockId, clock_gettime};

pub fn get_available_core_count() -> Result<usize> {
    let cpu_set = sched_getaffinity(None).map_err(|err| {
        ProcParserError::SysCallError(format!("Sched Affinity Sys call Error: {err}"))
    })?;

    Ok(cpu_set.count() as usize)
}

pub fn monotonic() -> f64 {
    // this is the rustix version and I feel its way faster than libc wrapper
    let real_time = clock_gettime(ClockId::MonotonicCoarse);
    (real_time.tv_sec as f64) + (real_time.tv_nsec as f64 / 1_000_000_000.0)
}
