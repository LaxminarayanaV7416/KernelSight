// The path to the cgroup v2 mount point
pub const CGROUP_V2_MOUNT_PATH: &str = "/sys/fs/cgroup";
// eg., /sys/fs/cgroup/system.slice/docker-a7c6777640a24c37f2f2fe1a894556fbb2c876379122ec2eca6f6f9fc4c4a8a1.scope
/*
 * Lists PIDs belonging directly to this cgroup.
 * Useful for mapping a Docker container cgroup to its member processes.
 */
pub const CGROUP_PROCS_FILE_SLUG: &str = "cgroup.procs";

/*
 * Reports cgroup lifecycle state such as whether it contains live processes.
 * Example: populated 1, frozen 0.
 */
pub const CGROUP_EVENTS_FILE_SLUG: &str = "cgroup.events";

/*
 * Contains cumulative CPU usage for all processes in the cgroup.
 * Includes usage_usec, user_usec, system_usec and CPU throttling counters.
 */
pub const CGROUP_CPU_STAT_FILE_SLUG: &str = "cpu.stat";

/*
 * Reports CPU Pressure Stall Information for this cgroup.
 * Shows how much time workloads waited because CPU resources were unavailable.
 */
pub const CGROUP_CPU_PRESSURE_FILE_SLUG: &str = "cpu.pressure";
/*
 * Contains the total amount of memory currently charged to the cgroup.
 * Value is reported in bytes and includes memory used by descendant cgroups.
 */
pub const CGROUP_MEMORY_CURRENT_FILE_SLUG: &str = "memory.current";

/*
 * Reports the highest memory usage recorded by the cgroup.
 * Useful for determining peak container memory consumption.
 */
pub const CGROUP_MEMORY_PEAK_FILE_SLUG: &str = "memory.peak";

/*
 * Contains detailed memory accounting for the cgroup.
 * Includes anon, file, kernel, slab, page faults, reclaim and many other counters.
 */
pub const CGROUP_MEMORY_STAT_FILE_SLUG: &str = "memory.stat";

/*
 * Contains memory-management events such as high, max, oom and oom_kill.
 * Useful for detecting container memory pressure and OOM-related failures.
 */
pub const CGROUP_MEMORY_EVENTS_FILE_SLUG: &str = "memory.events";

/*
 * Reports Memory Pressure Stall Information for tasks in this cgroup.
 * Shows time workloads were stalled waiting for available memory.
 */
pub const CGROUP_MEMORY_PRESSURE_FILE_SLUG: &str = "memory.pressure";

/*
 * Contains cumulative block-device I/O statistics for this cgroup.
 * Includes read/write bytes and operation counts separated by device.
 */
pub const CGROUP_IO_STAT_FILE_SLUG: &str = "io.stat";

/*
 * Reports I/O Pressure Stall Information for the cgroup.
 * Shows time tasks were stalled waiting for block I/O resources.
 */
pub const CGROUP_IO_PRESSURE_FILE_SLUG: &str = "io.pressure";

/*
 * Reports the current number of processes in the cgroup hierarchy.
 * Useful for tracking process counts associated with a container.
 */
pub const CGROUP_PIDS_CURRENT_FILE_SLUG: &str = "pids.current";

/*
 * Reports the largest number of processes observed in the cgroup hierarchy.
 * Useful for identifying peak process population for a container.
 */
pub const CGROUP_PIDS_PEAK_FILE_SLUG: &str = "pids.peak";

/*
 * Reports events where process creation encountered the configured PID limit.
 * Useful for diagnosing fork/clone failures caused by pids.max.
 */
pub const CGROUP_PIDS_EVENTS_FILE_SLUG: &str = "pids.events";
