use crate::common::procfs_constants::PROC_FS_ROOT_PATH;
use std::fs;
use std::path::Path;

fn parse_procfs_pid_cachable_file(pid: usize, file_name: &str) -> String {
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH)
        .join(pid.to_string())
        .join(file_name);
    let reader = fs::read_to_string(&load_avg_path.to_str().unwrap());
    match reader {
        Ok(reader) => reader,
        Err(_) => String::new(),
    }
}

pub fn parse_procfs_pid_cgroup(pid: usize) -> String {
    parse_procfs_pid_cachable_file(pid, "cgroup")
}

pub fn parse_procfs_pid_cmdline(pid: usize) -> String {
    parse_procfs_pid_cachable_file(pid, "cmdline")
}

pub fn parse_procfs_pid_comm(pid: usize) -> String {
    parse_procfs_pid_cachable_file(pid, "comm")
}
