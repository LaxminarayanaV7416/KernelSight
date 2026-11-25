use super::cgroup_constants::CGROUP_V2_MOUNT_PATH;
use super::get_available_core_count;
use rustix::param::clock_ticks_per_second;
use std::process::Command;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CgroupType {
    V1,
    V2,
}

// TODO: Work around here, you can figure out from the mount path
// that is /proc/mounts search for cgroup2 line it will help
// you determine if the cgroup version is V1 or V2
pub fn cgroup_classifier() -> CgroupType {
    let output = Command::new("stat")
        .arg("-fc")
        .arg("%T")
        .arg(CGROUP_V2_MOUNT_PATH)
        .output()
        .expect("failed to execute cgroup command");
    let result = String::from_utf8_lossy(&output.stdout).to_string();
    if result == "cgroup2fs\n" {
        CgroupType::V2
    } else {
        CgroupType::V1
    }
}

// run once and determine during runtime
// you have access these using dereferncing as of C style
// eg., *CPU_CORES gives you the number of available logical CPUs
pub static CPU_CORES: LazyLock<usize> = LazyLock::new(|| get_available_core_count().unwrap_or(1));
pub static CGROUP_TYPE: LazyLock<CgroupType> = LazyLock::new(cgroup_classifier);
pub static CPU_HZ: LazyLock<u64> = LazyLock::new(clock_ticks_per_second);
