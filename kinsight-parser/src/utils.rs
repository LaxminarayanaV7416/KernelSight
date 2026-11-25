use kinsight_proc_parser::ProcFSStatCPUFields;

pub fn get_cpu_usage(current: &ProcFSStatCPUFields, previous: &ProcFSStatCPUFields) -> Option<f64> {
    let current_idle = current.idle + current.iowait;
    let previous_idle = previous.idle + previous.iowait;
    let current_non_idle = current.user
        + current.nice
        + current.system
        + current.irq
        + current.softirq
        + current.steal;

    let previous_non_idle = previous.user
        + previous.nice
        + previous.system
        + previous.irq
        + previous.softirq
        + previous.steal;

    let current_total = current_idle + current_non_idle;

    let previous_total = previous_idle + previous_non_idle;

    // checked_sub avoids panicking if counters unexpectedly
    // decrease, for example after a reset or malformed read.
    let idle_delta = current_idle.checked_sub(previous_idle);

    let total_delta = current_total.checked_sub(previous_total);
    match (idle_delta, total_delta) {
        (Some(idle_delta), Some(total_delta)) if total_delta > 0 => {
            let busy_delta = total_delta.saturating_sub(idle_delta);
            let cpu_usage = 100.0 * busy_delta as f64 / total_delta as f64;
            Some(cpu_usage)
        }
        _ => None,
    }
}

pub fn calcualte_diff_delta(current: u64, previous: u64, time_delta: f64) -> f64 {
    100.0 * current.saturating_sub(previous) as f64 / time_delta
}
