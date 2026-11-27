use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CpuSystemMetrics {
    pub cpu_utilization_percentage: f64,         // Done
    pub cpu_core_utilization_per_core: Vec<f64>, // Done
    pub ctxt_per_time_delta: f64,                // curr - prev / time
    pub total_ctxt: u64,                         // direct value from /proc/stat
    pub processes_running: u64,                  // direct value fro, /proc/stat
    pub io_blocked_task_count: u64,              // direct value from /proc/stat
    pub cpu_pressure: f64,                       // /proc/pressure/cpu total diff / time delta
    pub softirq_time_delta: f64,                 // /proc/stat softirq diff / time delta
    pub total_thread_count: u64,                 // direct value from /proc/loadavg
    pub total_thread_running: u64,               // direct value from /proc/loadavg
}

#[derive(Debug, Default)]
pub struct MemorySystemMetrics {
    pub total_ram: u64, // Read MemTotal directly (total usable RAM after kernel binary & reserved boot allocations).
    pub available_ram: u64, // Read MemAvailable directly (kernel estimate of RAM available for new workloads without swapping).
    pub ram_used: u64,      // memtotal - memavailable
    pub ram_usage_percentage: f64, // memtotal - memavaialble / memtotal
    pub free_ram: u64,      // memfree from /proc/meminfo
    pub page_cache: u64, // Read Cached directly. (Note: Cached includes Shmem and tmpfs; for pure disk-backed file cache, calculate Cached - Shmem).
    pub buffer: u64,     // Read Buffers directly (raw block-device metadata/superblock buffers).
    pub anonymous_memory: u64, // Read AnonPages directly (process heap, stack, and anonymous mmap pages not backed by files).
    pub shared_memory: u64, // Read Shmem directly (System V/POSIX shared memory and tmpfs files).
    pub slab_memory: u64, // Read Slab directly (total kernel data-structure cache; equals SReclaimable + SUnreclaim).
    pub reclaimable_slab: u64, // Read SReclaimable directly (dentry and inode caches that can be freed under pressure).
    pub unreclaimable_slab: u64, // Read SUnreclaim directly (pinned kernel structures that cannot be reclaimed).
    pub kernel_stack: u64, // Read KernelStack directly (memory allocated to kernel thread stacks).
    pub page_tables: u64, // Read PageTables directly (memory used to store virtual-to-physical page table entries).
    pub swap_used: u64,   // SwapUsed=SwapTotal−SwapFreeSwapUsed equals SwapTotal minus SwapFree
    pub page_fault_time_delta: f64, // pgfault from /proc/vmstat
    pub page_major_fault_time_delta: f64, // pgmajfault from /proc/vmstat
    pub oom_kill_time_delta: f64, // oom_kill from /proc/vmstat time delta
    pub memory_pressure: f64, // /proc/pressure/memory total
}

#[derive(Debug, Default)]
pub struct NetworkMetrics {
    pub total_download_bytes: u64,     // direct map from /proc/net/dev
    pub total_donwload_packets: u64,   // direct map from /proc/net/dev
    pub bad_packets_downloaded: u64,   // direct map from /proc/net/dev
    pub recieved_packets_dropped: u64, // direct map from /proc/net/dev
    pub total_upload_bytes: u64,       // direct map from /proc/net/dev
    pub total_upload_packets: u64,     // direct map from /proc/net/dev
    pub bad_packets_uploaded: u64,     // direct map from /proc/net/dev
    pub sent_packets_dropped: u64,     // direct map from /proc/net/dev
    pub rate_download_bytes: f64,      // curr - prev/ time_delta
    pub rate_upload_bytes: f64,        // curr - prev / time_delta
    pub rate_donwload_packets: f64,
    pub rate_upload_packets: f64,
}

#[derive(Debug, Default)]
pub struct SystemWideMetrics {
    pub time: f64,
    pub time_delta: f64,
    pub cpu_metrics: CpuSystemMetrics,
    pub memory_metrics: MemorySystemMetrics,
    pub network_metrics: HashMap<String, NetworkMetrics>,
}

#[derive(Debug, Default)]
pub struct CpuPIDMetrics {
    pub time: f64,
    pub time_delta: f64,
    pub pid: u64,                    //direct value from /proc/pid/stat
    pub ppid: u64,                   // direct value from /proc/pid/stat
    pub process_state: char,         // direct value from /proc/pid/stat
    pub user_cpu_time: u64,          // direct valure utime from stat
    pub system_cpu_time: u64,        // direct value stime from stat
    pub total_cpu_time: u64,         // sum pf abpve two values
    pub num_threads: i64,            // number of threads from stat
    pub voluntary_ctxt: u64,         // volumntary context switches from status
    pub non_voluntary_ctxt: u64,     // non volunary context switches from status
    pub ctxt_switch_time_delta: u64, // ctxt switches from delta time
    pub minor_page_fault: u64,       // stat minflt
    pub major_page_fault: u64,       // stat majflt
    pub scheduling_runtime: u64,     // schedstat direct value
    pub runqueue_wait_time: u64,     // schedstat direct value
    pub timeslices: u64,             // schedstat direct value
    pub last_cpu_used: i64,          // stat processor
    pub scheduling_policy: u64,      // stat policy
    pub cpu_affinity: u64,           // schedstat direct values cpu_allowed_list
    pub process_start_time: u64,     // stat direct value from starttime
    pub total_program_size: u64,     // vmsize from direct value from status
    pub resident_set_size: u64,      // vmrss from status
    pub shared_data: u64,            // rss_file + rssshmem from status
    pub cgroup: String,
    pub comm: String,
    pub cmdline: String,
}
