use crate::common::procfs_constants::{
    PROC_FS_ROOT_PATH, PROC_PID_CGROUP_FILE_SLUG, PROC_PID_CMDLINE_FILE_SLUG,
    PROC_PID_COMM_FILE_SLUG,
};
use std::error::Error;
use std::fs;
use std::path::Path;

fn parse_procfs_pid_cachable_file(pid: usize, file_name: &str) -> Result<String, Box<dyn Error>> {
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH)
        .join(pid.to_string())
        .join(file_name);
    let reader = fs::read_to_string(&load_avg_path.to_str().unwrap());
    let res = match reader {
        Ok(reader) => reader,
        Err(err) => {
            println!("String like File Opener Error: {}", err);
            return Err(err.into());
        }
    };
    Ok(res)
}

pub fn parse_procfs_pid_cgroup(pid: usize) -> Result<String, Box<dyn Error>> {
    parse_procfs_pid_cachable_file(pid, PROC_PID_CGROUP_FILE_SLUG)
}

pub fn parse_procfs_pid_cmdline(pid: usize) -> Result<String, Box<dyn Error>> {
    parse_procfs_pid_cachable_file(pid, PROC_PID_CMDLINE_FILE_SLUG)
}

pub fn parse_procfs_pid_comm(pid: usize) -> Result<String, Box<dyn Error>> {
    parse_procfs_pid_cachable_file(pid, PROC_PID_COMM_FILE_SLUG)
}
