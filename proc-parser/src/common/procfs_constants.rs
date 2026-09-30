pub const PROC_FS_ROOT_PATH: &str = "/proc";
pub const SYS_FS_ROOT_PATH: &str = "/sys";

// PROC folder constants
// The folder is not important in metrics collection
// =================================================
pub const PROC_PRESSURE_FOLDER_SLUG: &str = "pressure";
pub const PROC_SYS_FOLDER_SLUG: &str = "sys";

// PROC FOLDER pressure files
pub const PROC_PRESSURE_CPU_FILE_SLUG: &str = "cpu";
pub const PROC_PRESSURE_MEMORY_FILE_SLUG: &str = "memory";
pub const PROC_PRESSURE_IO_FILE_SLUG: &str = "io";

// PROC FOLDER sys files
pub const PROC_SYS_KERNEL_FOLDER_SLUG: &str = "kernel";
pub const PROC_SYS_KERNEL_FOLDER_PID_MAX_FILE_SLUG: &str = "pid_max";
pub const PROC_SYS_KERNEL_FOLDER_THREADS_MAX_FILE_SLUG: &str = "threads-max";

// PROC file constants
// =================================================
// PROC root files used by the monitoring parser
// =================================================

pub const RequiredForParserProcFiles: &[&str; 9] = &[
    "diskstats", // done, Got this from /sys/block/<device>/stat (no sudo required)
    "interrupts", // not too much important, lets see later on
    "loadavg", // done, working on this from /proc/loadavg (no sudo required)
    "meminfo", // done, working on this from /proc/meminfo (no sudo required)
    "schedstat", // currently working on this from /proc/schedstat (no sudo required)
    "softirqs", // currently working on this from /proc/softirqs (no sudo required)
    "stat", // currently working on this from /proc/stat (no sudo required)
    "uptime", // currently working on this from /proc/uptime (no sudo required)
    "vmstat", // currently working on this from /proc/vmstat (no sudo required)
];

pub const NotRequiredForParserProcFiles: &[&str; 36] = &[
    "bootconfig",
    "buddyinfo",
    "cgroups",
    "cmdline",
    "consoles",
    "cpuinfo",
    "crypto",
    "devices",
    "dma",
    "execdomains",
    "fb",
    "filesystems",
    "iomem",
    "ioports",
    "kallsyms",
    "kcore",
    "key-users",
    "keys",
    "kmsg",
    "kpagecgroup",
    "kpagecount",
    "kpageflags",
    "locks",
    "misc",
    "modules",
    "mtd",
    "mtrr",
    "pagetypeinfo",
    "partitions",
    "slabinfo",
    "swaps",
    "sysrq-trigger",
    "timer_list",
    "version",
    "vmallocinfo",
    "zoneinfo",
];

// =================================================
// PROC root file constants
// =================================================

/*
 * Contains cumulative block-device I/O statistics for disks and partitions.
 * Includes reads, writes, sectors, I/O time, queue time, discard and flush counters.
 */
pub const PROC_DISKSTATS_FILE_SLUG: &str = "diskstats";

/*
 * Shows hardware interrupt counts independently for each logical CPU.
 * Useful for detecting interrupt load, IRQ imbalance and device-related CPU activity.
 */
pub const PROC_INTERRUPTS_FILE_SLUG: &str = "interrupts";

/*
 * Contains 1, 5 and 15 minute load averages plus runnable/total task counts.
 * Example: 1.42 1.17 0.93 3/825 22341
 */
pub const PROC_LOADAVG_FILE_SLUG: &str = "loadavg";

/*
 * Contains system-wide physical memory and swap utilization statistics.
 * Includes MemAvailable, Cached, Active, Dirty, Slab, SwapFree and many other counters.
 */
pub const PROC_MEMINFO_FILE_SLUG: &str = "meminfo";

/*
 * Contains system-wide scheduler statistics for every CPU and scheduling domain.
 * Useful for studying CPU runtime, scheduler activity and load-balancing behavior.
 */
pub const PROC_SCHEDSTAT_FILE_SLUG: &str = "schedstat";

/*
 * Contains cumulative softirq handler counts independently for each CPU.
 * Useful for monitoring network, timer, scheduler, block and RCU processing activity.
 */
pub const PROC_SOFTIRQS_FILE_SLUG: &str = "softirqs";

/*
 * Contains system-wide cumulative CPU and kernel activity counters since boot.
 * Includes CPU times, context switches, interrupts, runnable tasks and process creation.
 */
pub const PROC_STAT_FILE_SLUG: &str = "stat";

/*
 * Contains system uptime and accumulated CPU idle time in seconds.
 * Example: 148527.34 1123875.92
 */
pub const PROC_UPTIME_FILE_SLUG: &str = "uptime";

/*
 * Contains cumulative virtual-memory activity counters maintained by the kernel.
 * Includes page faults, paging, swapping, reclaim, NUMA and page allocator activity.
 */
pub const PROC_VMSTAT_FILE_SLUG: &str = "vmstat";

// =================================================
// PROC PID files used by the monitoring parser
// =================================================

pub const RequiredForParserPIDFiles: &[&str; 9] = &[
    "cmdline",
    "comm",
    "cgroup",
    "io",
    "schedstat",
    "smaps_rollup",
    "stat",
    "statm",
    "status",
];

pub const NotRequiredForParserPIDFiles: &[&str; 35] = &[
    "arch_status",
    "autogroup",
    "auxv",
    "clear_refs",
    "coredump_filter",
    "cpu_resctrl_groups",
    "environ",
    "gid_map",
    "ksm_merging_pages",
    "ksm_stat",
    "limits",
    "loginuid",
    "maps",
    "mem",
    "mountinfo",
    "mounts",
    "mountstats",
    "numa_maps",
    "oom_adj",
    "oom_score",
    "oom_score_adj",
    "pagemap",
    "personality",
    "projid_map",
    "sched",
    "sessionid",
    "setgroups",
    "smaps",
    "stack",
    "syscall",
    "timens_offsets",
    "timers",
    "timerslack_ns",
    "uid_map",
    "wchan",
];

// =================================================
// PROC PID file constants
// =================================================

/*
 * Shows the control groups to which the process belongs.
 * Useful for associating processes with containers, services and resource groups.
 */
pub const PROC_PID_CGROUP_FILE_SLUG: &str = "cgroup";

/*
 * Contains the process command-line arguments separated by NUL bytes.
 * Example: /usr/bin/python3\0worker.py\0--port\08000\0
 */
pub const PROC_PID_CMDLINE_FILE_SLUG: &str = "cmdline";

/*
 * Contains the process/task command name and is limited to TASK_COMM_LEN.
 * Example: python3
 */
pub const PROC_PID_COMM_FILE_SLUG: &str = "comm";

/*
 * Contains cumulative per-process read/write syscall and storage I/O counters.
 * Example: read_bytes: 4096, write_bytes: 8192, syscr: 120.
 */
pub const PROC_PID_IO_FILE_SLUG: &str = "io";

/*
 * Contains compact scheduler runtime, run-queue wait time and timeslice count.
 * Useful for detecting CPU scheduling contention with very small parsing overhead.
 */
pub const PROC_PID_SCHEDSTAT_FILE_SLUG: &str = "schedstat";

/*
 * Aggregates smaps statistics across all VMAs into one process-wide record.
 * Provides RSS, PSS, USS-related fields, swap and anonymous/file/shared memory.
 */
pub const PROC_PID_SMAPS_ROLLUP_FILE_SLUG: &str = "smaps_rollup";

/*
 * Compact single-line process statistics including CPU time, faults, PID/PPID and state.
 * Also includes threads, start time, virtual memory, RSS and last scheduled CPU.
 */
pub const PROC_PID_STAT_FILE_SLUG: &str = "stat";

/*
 * Provides seven compact process memory counters measured in pages.
 * Includes virtual size, resident pages and shared pages.
 */
pub const PROC_PID_STATM_FILE_SLUG: &str = "statm";

/*
 * Human-readable process metadata covering IDs, state, memory and context switches.
 * Useful fields include PPid, VmRSS, Threads, Uid/Gid and voluntary context switches.
 */
pub const PROC_PID_STATUS_FILE_SLUG: &str = "status";

// NOTE: We are not going to monitor any subfolders under /proc/<pid>
// This is because it becomes very inappropriate to do so, daemon gets very beefier
