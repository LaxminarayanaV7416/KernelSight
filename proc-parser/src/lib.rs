pub mod common {
    pub mod cgroup_constants;
    pub mod config;
    pub mod file_reader;
    pub mod kernel_types;
    pub mod message_passing_utils;
    pub mod parser_utils;
    pub mod procfs_constants;
    pub mod system_call_utils;
}
pub mod configs {
    pub mod cgroupfs {
        pub mod cgroup_v2_cgroup_events;
        pub mod cgroup_v2_cpu_pressure;
        pub mod cgroup_v2_cpu_stat;
        pub mod cgroup_v2_io_pressure;
        pub mod cgroup_v2_memory_current;
        pub mod cgroup_v2_memory_events;
        pub mod cgroup_v2_memory_peak;
        pub mod cgroup_v2_memory_pressure;
        pub mod cgroup_v2_memory_stat;
        pub mod cgroup_v2_pids_current;
        pub mod cgroup_v2_pids_events;
        pub mod cgroup_v2_pids_peak;
    }
    pub mod config;
    pub mod procfs_diskstats_config;
    pub mod procfs_loadavg_config;
    pub mod procfs_meminfo_config;
    pub mod procfs_pidfs_schedstat_config;
    pub mod procfs_pidfs_stat_config;
    pub mod procfs_pidfs_statm_config;
    pub mod procfs_pidfs_status_config;
    pub mod procfs_pressure_config;
    // pub mod procfs_softirqs_config;
    pub mod procfs_stat_config;
    pub mod procfs_uptime_config;
    pub mod procfs_vmstat_config;
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
    pub mod vmstat;
}
pub mod sysfs {
    pub mod diskstats;
}
pub mod gpu {
    pub mod nvidia;
}
pub mod parser {
    pub mod pidsfs_utils;
    pub mod procfs_parse_initiater;
    pub mod procfs_parser;
    pub mod procfs_pid_parser;
    pub mod sensors;
}
pub mod writer {
    pub mod writer;
    pub mod writer_utils;
}
