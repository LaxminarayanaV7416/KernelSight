use crate::common::cgroup_constants::CGROUP_V2_MOUNT_PATH;
use crate::procfs::pressure::pressure_util::ProcProcessGeneralReader;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

pub fn parse_cgroup_v2_memory_pressure(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
    cgroup_name: &str,
) -> JoinHandle<()> {
    let cgroup_event_path = Path::new(CGROUP_V2_MOUNT_PATH)
        .join("system.slice")
        .join(cgroup_name)
        .join("memory.pressure");
    let cgroup_cpu_pressure_temp =
        ProcProcessGeneralReader::new(&cgroup_event_path.to_str().unwrap(), is_cachable, is_root);
    let mut cgroup_cpu_pressure = match cgroup_cpu_pressure_temp {
        Ok(reader) => Box::new(reader),
        Err(e) => panic!("Failed to create loadavg reader: {}", e),
    };
    let filter_map = HashMap::from([
        (1, true),
        (2, true),
        (3, true),
        (4, true),
        (5, true),
        (6, true),
        (7, true),
        (8, true),
    ]);
    let thread_handle = thread::spawn(move || {
        while !signal_hook.load(Ordering::Relaxed) {
            let read_status = cgroup_cpu_pressure.read_and_parse(&filter_map);
            if !read_status {
                println!("Failed to read loadavg, ProcFS File closed!");
                break;
            }
            println!("Pressure CPU: {:?}", cgroup_cpu_pressure.values());
            thread::sleep(std::time::Duration::from_millis(monitoring_heartbeat));
        }
        println!("Killed LoadAvg thread!!");
    });
    thread_handle
}
