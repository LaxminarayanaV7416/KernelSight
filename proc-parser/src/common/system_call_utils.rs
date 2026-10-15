use libc::{
    CLOCK_MONOTONIC_COARSE, CPU_COUNT, CPU_ZERO, clock_gettime, cpu_set_t, sched_getaffinity,
    timespec,
};
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

pub fn monotonic() -> i64 {
    let mut time = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe { clock_gettime(CLOCK_MONOTONIC_COARSE, &mut time) };
    time.tv_nsec
}
