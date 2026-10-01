pub mod common {
    pub mod cgroup_constants;
    pub mod config;
    pub mod file_reader;
    pub mod kernel_types;
    pub mod parser_utils;
    pub mod procfs_constants;
}
pub mod configs {
    pub mod procfs_loadavg_config;
    pub mod procfs_meminfo_config;
    pub mod sysfs_diskstats_config;
}
pub mod cgroupfs {}
pub mod procfs {
    pub mod pidfs {
        pub mod cachable_reads;
        pub mod schedstat;
    }
    pub mod pressure {
        pub mod cpu;
        pub mod io;
        pub mod irq;
        pub mod memory;
        mod pressure_util;
    }
    pub mod loadavg;
    pub mod meminfo;
}
pub mod sysfs {
    pub mod diskstats;
}
