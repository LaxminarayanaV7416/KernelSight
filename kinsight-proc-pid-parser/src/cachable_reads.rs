use kinsight_common::procfs_constants::{
    PROC_FS_ROOT_PATH, PROC_PID_CGROUP_FILE_SLUG, PROC_PID_CMDLINE_FILE_SLUG,
    PROC_PID_COMM_FILE_SLUG,
};
use kinsight_common::{ProcParserError, Result};
use std::fs;
use std::path::Path;

fn parse_procfs_pid_cachable_file(pid: u64, file_name: &str) -> Result<String> {
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH)
        .join(pid.to_string())
        .join(file_name);
    fs::read_to_string(&load_avg_path.to_str().unwrap())
        .map_err(|_| ProcParserError::FileReadError(load_avg_path.to_string_lossy().into_owned()))
}

pub fn parse_procfs_pid_cgroup(pid: u64) -> Result<String> {
    parse_procfs_pid_cachable_file(pid, PROC_PID_CGROUP_FILE_SLUG)
}

pub fn parse_procfs_pid_cmdline(pid: u64) -> Result<String> {
    parse_procfs_pid_cachable_file(pid, PROC_PID_CMDLINE_FILE_SLUG)
}

pub fn parse_procfs_pid_comm(pid: u64) -> Result<String> {
    parse_procfs_pid_cachable_file(pid, PROC_PID_COMM_FILE_SLUG)
}
