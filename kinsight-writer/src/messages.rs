#[derive(Debug)]
pub struct CpuSystemMetrics {
    time: f64,
    time_delta: f64,
    cpu_utilization_percentage: f64,         // Done
    cpu_core_utilization_per_core: Vec<f64>, // Done
    ctxt_per_time_delta: u64,                // curr - prev / time
    total_ctxt: u64,                         // direct value from /proc/stat
    runnable_thread_count: u64,              // direct value fro, /proc/stat
    io_blocked_task_count: u64,              // direct value from /proc/stat
    cpu_pressure: f64,                       // /proc/pressure/cpu total diff / time delta
    cpu_steal: u64,
    softirq_time_delta: u64, // /proc/stat softirq diff / time delta
}

#[derive(Debug)]
pub struct MemorySystemMetrics {
    time: f64,
    time_delta: f64,
    total_ram: u64, // Read MemTotal directly (total usable RAM after kernel binary & reserved boot allocations).
    available_ram: u64, // Read MemAvailable directly (kernel estimate of RAM available for new workloads without swapping).
    ram_used: u64,      // memtotal - memavailable
    ram_usage_percentage: f64, // memtotal - memavaialble / memtotal
    free_ram: u64,      // memfree from /proc/meminfo
    page_cache: u64, // Read Cached directly. (Note: Cached includes Shmem and tmpfs; for pure disk-backed file cache, calculate Cached - Shmem).
    buffer: u64,     // Read Buffers directly (raw block-device metadata/superblock buffers).
    anonymous_memory: u64, // Read AnonPages directly (process heap, stack, and anonymous mmap pages not backed by files).
    shared_memory: u64,    // Read Shmem directly (System V/POSIX shared memory and tmpfs files).
    slab_memory: u64, // Read Slab directly (total kernel data-structure cache; equals SReclaimable + SUnreclaim).
    reclaimable_slab: u64, // Read SReclaimable directly (dentry and inode caches that can be freed under pressure).
    unreclaimable_slab: u64, // Read SUnreclaim directly (pinned kernel structures that cannot be reclaimed).
    kernel_stack: u64, // Read KernelStack directly (memory allocated to kernel thread stacks).
    page_tables: u64, // Read PageTables directly (memory used to store virtual-to-physical page table entries).
    swap_used: u64,   // SwapUsed=SwapTotal−SwapFreeSwapUsed equals SwapTotal minus SwapFree
    page_fault_time_delta: f64, // pgfault from /proc/vmstat
    page_major_fault_time_delta: f64, // pgmajfault from /proc/vmstat
    oom_kill_time_delta: f64, // oom_kill from /proc/vmstat time delta
    memory_pressure: f64, // /proc/pressure/memory total
}
