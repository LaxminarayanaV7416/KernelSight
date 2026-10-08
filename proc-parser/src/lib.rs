pub mod common {
    pub mod cgroup_constants;
    pub mod config;
    pub mod file_reader;
    pub mod kernel_types;
    pub mod parser_utils;
    pub mod procfs_constants;
    mod system_call_utils;
}
pub mod configs {
    pub mod procfs_loadavg_config;
    pub mod procfs_meminfo_config;
    pub mod procfs_pidfs_stat_config;
    pub mod procfs_pidfs_status_config;
    pub mod sysfs_diskstats_config;
}
pub mod cgroupfs {
    pub mod v2 {
        pub mod cgroup_events;
        pub mod cgroup_procs;
        pub mod cpu_pressure;
        pub mod cpu_stat;
        pub mod io_pressure;
        pub mod memory_current;
        pub mod memory_events;
        pub mod memory_peak;
        pub mod memory_stat;
        pub mod pids_current;
        pub mod pids_events;
        pub mod pids_peak;
    }
    pub mod v1 {}
}
pub mod procfs {
    pub mod pidfs {
        pub mod cachable_reads;
        pub mod schedstat;
        pub mod stat;
        pub mod statm;
        pub mod status;
    }
    pub mod pressure {
        pub mod cpu;
        pub mod io;
        pub mod irq;
        pub mod memory;
        pub mod pressure_util;
    }
    pub mod loadavg;
    pub mod meminfo;
    pub mod stat;
    pub mod uptime;
}
pub mod sysfs {
    pub mod diskstats;
}
