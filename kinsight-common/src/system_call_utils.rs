use libc::{CPU_COUNT, CPU_ZERO};
use libc::{cpu_set_t, sched_getaffinity};
use rustix::time::{ClockId, clock_gettime};
// use libc::{CLOCK_MONOTONIC_COARSE, clock_gettime, timespec};
use std::mem;

pub fn available_logical_cpus() -> Result<usize, Box<dyn std::error::Error>> {
    // we use sched_getaffinity to get the number of logical CPUs
    // we are passing PID as 0 that represents the root of all processes
    unsafe {
        let mut set: cpu_set_t = mem::zeroed();
        CPU_ZERO(&mut set);
        let ret = sched_getaffinity(
            0, // PID 0 represents the root of all processes
            mem::size_of::<cpu_set_t>(),
            &mut set,
        );
        if ret != 0 {
            return Err(Box::new(std::io::Error::last_os_error()));
        }
        Ok(CPU_COUNT(&set) as usize)
    }
}

// pub fn monotonic() -> f64 {
//     let mut time = timespec {
//         tv_sec: 0,
//         tv_nsec: 0,
//     };
//     unsafe { clock_gettime(CLOCK_MONOTONIC_COARSE, &mut time) };
//     (time.tv_sec as f64) + (time.tv_nsec as f64 / 1_000_000_000.0)
// }

pub fn monotonic() -> f64 {
    // this is the rustix version and I feel its way faster than libc wrapper
    let real_time = clock_gettime(ClockId::MonotonicCoarse);
    (real_time.tv_sec as f64) + (real_time.tv_nsec as f64 / 1_000_000_000.0)
}
