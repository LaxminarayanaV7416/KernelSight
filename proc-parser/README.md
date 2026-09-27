

```rust
// pub const PROC_ACPI_FOLDER_SLUG: &str = "acpi";
// pub const PROC_ASOUND_FOLDER_SLUG: &str = "asound";
// pub const PROC_BUS_FOLDER_SLUG: &str = "bus";
// pub const PROC_DRIVER_FOLDER_SLUG: &str = "driver";
// pub const PROC_DYNAMIC_DEBUG_FOLDER_SLUG: &str = "dynamic_debug";
// // The above folder contains only config file which is used to debug
// // running kernel, not important for metrics collection but we can
// // still collect it for debugging purposes; enable in future
// pub const PROC_FS_FOLDER_SLUG: &str = "fs";
// pub const PROC_IRQ_FOLDER_SLUG: &str = "irq";
// pub const PROC_SYSVIPC_FOLDER_SLUG: &str = "sysvipc";
// pub const PROC_TTY_FOLDER_SLUG: &str = "tty";
// pub const PROC_NET_FOLDER_SLUG: &str = "net";
// pub const PROC_THREAD_SELF_FILE_SLUG: &str = "thread-self";
// pub const PROC_SELF_FILE_SLUG: &str = "self";
// pub const PROC_SCSI_FOLDER_SLUG: &str = "scsi";


/*
 * Contains the kernel boot configuration when CONFIG_BOOT_CONFIG is enabled.
 * Mostly static information describing options supplied to the kernel at boot.
 */
pub const PROC_BOOTCONFIG_FILE_SLUG: &str = "bootconfig";

/*
 * Shows free physical-memory blocks grouped by NUMA node, zone and allocation order.
 * Useful for diagnosing memory fragmentation but not needed for normal monitoring.
 */
pub const PROC_BUDDYINFO_FILE_SLUG: &str = "buddyinfo";

/*
 * Lists registered cgroup subsystems and whether each controller is enabled.
 * This describes cgroup capabilities rather than runtime process resource usage.
 */
pub const PROC_CGROUPS_FILE_SLUG: &str = "cgroups";

/*
 * Contains the command-line parameters passed to the Linux kernel during boot.
 * Example: BOOT_IMAGE=... root=UUID=... ro quiet
 */
pub const PROC_CMDLINE_FILE_SLUG: &str = "cmdline";

/*
 * Lists console devices registered with the kernel and their capabilities.
 * Primarily useful for console and kernel debugging rather than performance monitoring.
 */
pub const PROC_CONSOLES_FILE_SLUG: &str = "consoles";

/*
 * Contains detailed information about each logical CPU and its supported features.
 * Includes vendor, model, cache size, frequency and CPU feature flags.
 */
pub const PROC_CPUINFO_FILE_SLUG: &str = "cpuinfo";

/*
 * Lists cryptographic algorithms currently registered with the Linux Crypto API.
 * Includes algorithm name, driver, priority, block size and implementation details.
 */
pub const PROC_CRYPTO_FILE_SLUG: &str = "crypto";

/*
 * Lists character and block device major numbers currently registered with the kernel.
 * Useful for device-driver inspection but generally static during normal execution.
 */
pub const PROC_DEVICES_FILE_SLUG: &str = "devices";

/*
 * Lists ISA DMA channels currently allocated by device drivers.
 * Relevant mainly to older hardware and low-level driver debugging.
 */
pub const PROC_DMA_FILE_SLUG: &str = "dma";

/*
 * Lists execution domains/personality handlers registered with the kernel.
 * Legacy kernel interface that is normally irrelevant for system monitoring.
 */
pub const PROC_EXECDOMAINS_FILE_SLUG: &str = "execdomains";

/*
 * Lists framebuffer devices currently registered with the kernel.
 * Relevant for graphics/framebuffer debugging rather than performance telemetry.
 */
pub const PROC_FB_FILE_SLUG: &str = "fb";

/*
 * Lists filesystems supported by the currently running kernel.
 * Entries marked nodev do not require an underlying block device.
 */
pub const PROC_FILESYSTEMS_FILE_SLUG: &str = "filesystems";

/*
 * Shows the system physical I/O memory map and devices using each address range.
 * Primarily useful for kernel/device debugging and usually mostly static.
 */
pub const PROC_IOMEM_FILE_SLUG: &str = "iomem";

/*
 * Shows I/O port address ranges reserved by kernel drivers and hardware devices.
 * Mainly useful on architectures using port-mapped I/O and for driver debugging.
 */
pub const PROC_IOPORTS_FILE_SLUG: &str = "ioports";

/*
 * Exposes kernel symbol names and their addresses when permitted by kernel security settings.
 * Used by debuggers, tracing tools and profilers rather than ordinary monitoring.
 */
pub const PROC_KALLSYMS_FILE_SLUG: &str = "kallsyms";

/*
 * Represents the kernel's virtual address space as a very large ELF core file.
 * Used by kernel debugging tools and should never be scraped for routine telemetry.
 */
pub const PROC_KCORE_FILE_SLUG: &str = "kcore";

/*
 * Lists kernel key users and statistics about keys owned by each UID.
 * Useful for key-management debugging rather than general performance monitoring.
 */
pub const PROC_USERS_FILE_SLUG: &str = "key-users";

/*
 * Lists keys visible to the calling process through the kernel key-management subsystem.
 * Access depends on permissions and the output may contain security-related metadata.
 */
pub const PROC_KEYS_FILE_SLUG: &str = "keys";

/*
 * Provides access to the kernel printk message ring buffer.
 * Reading it consumes/messages or requires special permissions and is not normal telemetry.
 */
pub const PROC_KMSG_FILE_SLUG: &str = "kmsg";

/*
 * Binary file mapping every physical page frame to its associated memory cgroup ID.
 * Intended for low-level memory analysis and can be extremely large.
 */
pub const PROC_KPAGECGROUP_FILE_SLUG: &str = "kpagecgroup";

/*
 * Binary array containing the reference count for every physical page frame.
 * Useful for page-level memory analysis but far too large for normal periodic scraping.
 */
pub const PROC_KPAGECOUNT_FILE_SLUG: &str = "kpagecount";

/*
 * Binary array containing kernel flags describing every physical page frame.
 * Used by specialized memory-analysis tools rather than lightweight monitoring.
 */
pub const PROC_KPAGEFLAGS_FILE_SLUG: &str = "kpageflags";

/*
 * Lists active kernel file locks such as POSIX, flock and lease locks.
 * Useful for diagnosing lock contention but potentially variable and expensive to parse.
 */
pub const PROC_LOCKS_FILE_SLUG: &str = "locks";

/*
 * Lists miscellaneous character devices registered with the kernel.
 * Mostly static device-registration information rather than performance data.
 */
pub const PROC_MISC_FILE_SLUG: &str = "misc";

/*
 * Lists currently loaded kernel modules together with size and dependency information.
 * Useful for system inventory but changes rarely during normal workloads.
 */
pub const PROC_MODULES_FILE_SLUG: &str = "modules";

/*
 * Lists Memory Technology Device partitions when the MTD subsystem is enabled.
 * Relevant primarily to flash devices and embedded systems.
 */
pub const PROC_MTD_FILE_SLUG: &str = "mtd";

/*
 * Describes x86 Memory Type Range Register configuration when supported.
 * Hardware configuration information primarily used for low-level debugging.
 */
pub const PROC_MTRR_FILE_SLUG: &str = "mtrr";

/*
 * Provides detailed page allocator statistics grouped by page type and allocation order.
 * Excellent for fragmentation debugging but substantially more detailed than normal telemetry.
 */
pub const PROC_PAGETYPEINFO_FILE_SLUG: &str = "pagetypeinfo";

/*
 * Lists block-device partitions known to the kernel and their approximate sizes.
 * Useful for inventory, but diskstats is better suited for runtime I/O monitoring.
 */
pub const PROC_PARTITIONS_FILE_SLUG: &str = "partitions";

/*
 * Contains detailed statistics for kernel slab allocator caches.
 * Useful for kernel-memory analysis but can be comparatively large and expensive to parse.
 */
pub const PROC_SLABINFO_FILE_SLUG: &str = "slabinfo";

/*
 * Lists configured swap areas together with capacity, usage and priority.
 * Useful mainly when swap behavior is part of the monitoring requirements.
 */
pub const PROC_SWAPS_FILE_SLUG: &str = "swaps";

/*
 * Writable interface used to immediately invoke Linux Magic SysRq operations.
 * This is a control/debug interface and must never be scraped as telemetry.
 */
pub const PROC_TRIGGER_FILE_SLUG: &str = "sysrq-trigger";

/*
 * Contains detailed information about timers currently managed by the kernel.
 * Primarily intended for timer debugging and can be large on busy systems.
 */
pub const PROC_TIMER_LIST_FILE_SLUG: &str = "timer_list";

/*
 * Contains the running kernel version and compiler/build information.
 * Static system metadata that normally only needs to be collected once.
 */
pub const PROC_VERSION_FILE_SLUG: &str = "version";

/*
 * Lists regions allocated from the kernel vmalloc virtual address space.
 * Useful for diagnosing kernel virtual-memory allocations but can be large.
 */
pub const PROC_VMALLOCINFO_FILE_SLUG: &str = "vmallocinfo";

/*
 * Provides detailed memory-zone allocator information including watermarks and free pages.
 * Useful for memory-pressure diagnostics but much more detailed than meminfo/vmstat.
 */
pub const PROC_ZONEINFO_FILE_SLUG: &str = "zoneinfo";
// pub const PROC_CONFIG_GZ_FILE_SLUG: &str = "config.gz";
// pub const PROC_MOUNTS_FILE_SLUG: &str = "mounts";


// ============================
// PROC FILE CONTENTS
// ===========================

/*
 * Shows architecture-specific state maintained for the process.
 * Example on x86: AVX512_elapsed_ms: 8
 */
pub const PROC_PID_ARCH_STATUS_FILE_SLUG: &str = "arch_status";

/*
 * Shows the scheduler autogroup containing the process and its nice value.
 * Example: /autogroup-123 nice 0
 */
pub const PROC_PID_AUTOGROUP_FILE_SLUG: &str = "autogroup";

/*
 * Binary ELF auxiliary vector supplied to the process when exec() occurred.
 * Contains pairs of machine-word IDs and values such as AT_PAGESZ and AT_ENTRY.
 */
pub const PROC_PID_AUXV_FILE_SLUG: &str = "auxv";

/*
 * Writable interface used to reset referenced, soft-dirty or high-water RSS state.
 * It is a control interface rather than process monitoring information.
 */
pub const PROC_PID_CLEAR_REFS_FILE_SLUG: &str = "clear_refs";

/*
 * Bit mask controlling which memory mappings are included in a core dump.
 * Example: 00000033
 */
pub const PROC_PID_COREDUMP_FILTER_FILE_SLUG: &str = "coredump_filter";

/*
 * Reports resource-control groups associated with Intel resctrl monitoring.
 * Relevant mainly when CPU cache/memory-bandwidth resctrl is configured.
 */
pub const PROC_PID_CPU_RECTRL_GROUPS_FILE_SLUG: &str = "cpu_resctrl_groups";

/*
 * Contains the process environment variables separated by NUL bytes.
 * Example: PATH=/usr/bin\0HOME=/home/user\0
 */
pub const PROC_PID_ENVIRON_FILE_SLUG: &str = "environ";
// somewhat useful especially in case of SADE

/*
 * Describes mappings between GIDs inside and outside a user namespace.
 * Example: 0 1000 1
 */
pub const PROC_PID_GID_MAP_FILE_SLUG: &str = "gid_map";

/*
 * Reports the number of pages currently merged by Kernel Samepage Merging.
 * Only useful when analyzing KSM behavior for the process.
 */
pub const PROC_PID_KSM_MERGING_PAGES_FILE_SLUG: &str = "ksm_merging_pages";

/*
 * Contains detailed per-process Kernel Samepage Merging statistics.
 * Example fields include ksm_rmap_items and ksm_merging_pages.
 */
pub const PROC_PID_KSM_STAT_FILE_SLUG: &str = "ksm_stat";

/*
 * Lists the process resource limits including CPU, files, stack and address space.
 * Example: Max open files 1024 1048576 files
 */
pub const PROC_PID_LIMITS_FILE_SLUG: &str = "limits";
// interesting file can be useful if we monitor some sandbox with limited resources

/*
 * Contains the audit login UID associated with the process.
 * Used primarily by the Linux audit subsystem rather than performance monitoring.
 */
pub const PROC_PID_LOGINUID_FILE_SLUG: &str = "loginuid";

/*
 * Lists every virtual memory area mapped into the process address space.
 * Each entry includes address range, permissions, offset, device, inode and pathname.
 */
pub const PROC_PID_MAPS_FILE_SLUG: &str = "maps";
// this is also heavily used by Debuggers like LLDB/GDB along with ptrace

/*
 * Provides direct access to the process virtual address space as binary memory.
 * Requires elevated permissions and is intended for debugging rather than monitoring.
 */
pub const PROC_PID_MEM_FILE_SLUG: &str = "mem";
// This file is heavily used by the Debuggers like LLDB/GDB

/*
 * Describes mounts visible inside the process mount namespace in detailed form.
 * Includes mount IDs, parents, mount points, filesystem type and propagation state.
 */
pub const PROC_PID_MOUNTINFO_FILE_SLUG: &str = "mountinfo";

/*
 * Lists filesystems mounted inside the process's mount namespace.
 * Similar to /proc/mounts but resolved according to the target process namespace.
 */
pub const PROC_PID_MOUNTS_FILE_SLUG: &str = "mounts";

/*
 * Provides statistics and configuration information for process-visible mounts.
 * Particularly useful for filesystems such as NFS that expose detailed statistics.
 */
pub const PROC_PID_MOUNTSTATS_FILE_SLUG: &str = "mountstats";

/*
 * Describes memory mappings together with their NUMA node placement and policies.
 * Useful for NUMA locality analysis but expensive/unnecessary for routine polling.
 */
pub const PROC_PID_NUMA_MAPS_FILE_SLUG: &str = "numa_maps";

/*
 * Legacy interface for adjusting the process OOM-killer preference.
 * Deprecated in favor of oom_score_adj.
 */
pub const PROC_PID_OOM_ADJ_FILE_SLUG: &str = "oom_adj";

/*
 * Shows the kernel's current OOM-killer score for the process.
 * Larger values generally make the process a stronger candidate for OOM termination.
 */
pub const PROC_PID_OOM_SCORE_FILE_SLUG: &str = "oom_score";

/*
 * Allows users to influence a process's OOM-killer badness score.
 * Values range from -1000 to 1000.
 */
pub const PROC_PID_OOM_SCORE_ADJ_FILE_SLUG: &str = "oom_score_adj";

/*
 * Binary interface exposing page-table information for each virtual memory page.
 * Used for low-level page analysis and normally requires elevated privileges.
 */
pub const PROC_PID_PAGEMAP_FILE_SLUG: &str = "pagemap";

/*
 * Contains the process execution-domain/personality value in hexadecimal.
 * Example: 00000000
 */
pub const PROC_PID_PERSONALITY_FILE_SLUG: &str = "personality";

/*
 * Describes project-ID mappings associated with a user namespace.
 * Mainly relevant to namespace and filesystem project-quota management.
 */
pub const PROC_PID_PROJID_MAP_FILE_SLUG: &str = "projid_map";

/*
 * Contains detailed Linux scheduler information about the process.
 * Includes runtime, switches, priorities and scheduler-specific statistics.
 */
pub const PROC_PID_SCHED_FILE_SLUG: &str = "sched";

/*
 * Contains the audit session identifier associated with the process.
 * Primarily useful for Linux auditing and login-session tracking.
 */
pub const PROC_PID_SESSIONID_FILE_SLUG: &str = "sessionid";

/*
 * Controls whether setgroups() may be used inside a process user namespace.
 * Usually contains either "allow" or "deny".
 */
pub const PROC_PID_SETGROUPS_FILE_SLUG: &str = "setgroups";

/*
 * Shows the process kernel-space stack trace when kernel configuration permits it.
 * Intended primarily for debugging blocked or stalled processes.
 */
pub const PROC_PID_STACK_FILE_SLUG: &str = "stack";

/*
 * Contains detailed memory statistics separately for every process VMA.
 * Provides RSS/PSS/private/shared details but can be expensive for frequent polling.
 */
pub const PROC_PID_SMAPS_FILE_SLUG: &str = "smaps";

/*
 * Shows the system call currently being executed by the process/thread.
 * Contains syscall number, arguments, stack pointer and instruction pointer.
 */
pub const PROC_PID_SYSCALL_FILE_SLUG: &str = "syscall";

/*
 * Contains offsets used by a process time namespace for monotonic and boot clocks.
 * Relevant mainly when diagnosing Linux time namespaces.
 */
pub const PROC_PID_TIMENS_OFFSETS_FILE_SLUG: &str = "timens_offsets";

/*
 * Lists POSIX timers currently owned by the process.
 * Includes timer ID, signal notification method and clock ID.
 */
pub const PROC_PID_TIMERS_FILE_SLUG: &str = "timers";

/*
 * Shows the process timer-slack value in nanoseconds.
 * Controls how much timers may be delayed to coalesce wakeups and save power.
 */
pub const PROC_PID_TIMERSLACK_NS_FILE_SLUG: &str = "timerslack_ns";

/*
 * Describes mappings between UIDs inside and outside a user namespace.
 * Example: 0 1000 1
 */
pub const PROC_PID_UID_MAP_FILE_SLUG: &str = "uid_map";

/*
 * Shows the kernel function in which a sleeping process is currently waiting.
 * Example: futex_wait_queue or ep_poll.
 */
pub const PROC_PID_WCHAN_FILE_SLUG: &str = "wchan";
// pub const PROC_PID_CWD_FILE_SLUG: &str = "cwd";
// pub const PROC_PID_EXE_FILE_SLUG: &str = "exe";
// pub const PROC_PID_ROOT_FILE_SLUG: &str = "root";


// =================================================
// CGROUP CORE FILE CONSTANTS
// =================================================

/*
 * Lists PIDs belonging directly to this cgroup.
 * Useful for mapping a Docker container cgroup to its member processes.
 */
pub const CGROUP_PROCS_FILE_SLUG: &str = "cgroup.procs";

/*
 * Reports cgroup lifecycle state such as whether it contains live processes.
 * Example: populated 1, frozen 0.
 */
pub const CGROUP_EVENTS_FILE_SLUG: &str = "cgroup.events";

/*
 * Lists resource controllers available to child cgroups.
 * Example: cpuset cpu io memory hugetlb pids.
 */
pub const CGROUP_CONTROLLERS_FILE_SLUG: &str = "cgroup.controllers";

/*
 * Shows controllers enabled for distribution to child cgroups.
 * This is primarily configuration information rather than runtime telemetry.
 */
pub const CGROUP_SUBTREE_CONTROL_FILE_SLUG: &str = "cgroup.subtree_control";

/*
 * Describes whether the cgroup is domain, threaded or another cgroup type.
 * Mostly structural information and generally does not change during monitoring.
 */
pub const CGROUP_TYPE_FILE_SLUG: &str = "cgroup.type";

/*
 * Lists individual thread IDs belonging directly to this cgroup.
 * Normally unnecessary when container monitoring is performed at process level.
 */
pub const CGROUP_THREADS_FILE_SLUG: &str = "cgroup.threads";

/*
 * Controls whether execution of tasks inside the cgroup is frozen.
 * This is a control interface rather than a resource accounting interface.
 */
pub const CGROUP_FREEZE_FILE_SLUG: &str = "cgroup.freeze";

/*
 * Writable interface that kills all processes contained in a cgroup tree.
 * This should never be read or written by a telemetry-only monitoring daemon.
 */
pub const CGROUP_KILL_FILE_SLUG: &str = "cgroup.kill";


// =================================================
// CPU CONTROLLER
// =================================================

/*
 * Contains cumulative CPU usage for all processes in the cgroup.
 * Includes usage_usec, user_usec, system_usec and CPU throttling counters.
 */
pub const CGROUP_CPU_STAT_FILE_SLUG: &str = "cpu.stat";

/*
 * Reports CPU Pressure Stall Information for this cgroup.
 * Shows how much time workloads waited because CPU resources were unavailable.
 */
pub const CGROUP_CPU_PRESSURE_FILE_SLUG: &str = "cpu.pressure";

/*
 * Defines the maximum CPU bandwidth allowed for the cgroup.
 * This is configuration information such as "50000 100000" or "max 100000".
 */
pub const CGROUP_CPU_MAX_FILE_SLUG: &str = "cpu.max";

/*
 * Controls the relative CPU scheduling weight of the cgroup.
 * Usually ranges from 1 to 10000 with 100 as the default.
 */
pub const CGROUP_CPU_WEIGHT_FILE_SLUG: &str = "cpu.weight";

/*
 * Provides a nice-like representation of the cgroup CPU scheduling weight.
 * It controls scheduling policy rather than reporting consumed CPU resources.
 */
pub const CGROUP_CPU_WEIGHT_NICE_FILE_SLUG: &str = "cpu.weight.nice";

/*
 * Configures additional CPU bandwidth that may temporarily exceed cpu.max.
 * Useful for CPU controller configuration but unnecessary for normal monitoring.
 */
pub const CGROUP_CPU_MAX_BURST_FILE_SLUG: &str = "cpu.max.burst";


// =================================================
// MEMORY CONTROLLER
// =================================================

/*
 * Contains the total amount of memory currently charged to the cgroup.
 * Value is reported in bytes and includes memory used by descendant cgroups.
 */
pub const CGROUP_MEMORY_CURRENT_FILE_SLUG: &str = "memory.current";

/*
 * Reports the highest memory usage recorded by the cgroup.
 * Useful for determining peak container memory consumption.
 */
pub const CGROUP_MEMORY_PEAK_FILE_SLUG: &str = "memory.peak";

/*
 * Contains detailed memory accounting for the cgroup.
 * Includes anon, file, kernel, slab, page faults, reclaim and many other counters.
 */
pub const CGROUP_MEMORY_STAT_FILE_SLUG: &str = "memory.stat";

/*
 * Contains memory-management events such as high, max, oom and oom_kill.
 * Useful for detecting container memory pressure and OOM-related failures.
 */
pub const CGROUP_MEMORY_EVENTS_FILE_SLUG: &str = "memory.events";

/*
 * Reports Memory Pressure Stall Information for tasks in this cgroup.
 * Shows time workloads were stalled waiting for available memory.
 */
pub const CGROUP_MEMORY_PRESSURE_FILE_SLUG: &str = "memory.pressure";

/*
 * Defines the hard upper memory limit for the cgroup.
 * Value is a byte count or "max" when no hard limit is configured.
 */
pub const CGROUP_MEMORY_MAX_FILE_SLUG: &str = "memory.max";

/*
 * Defines the memory usage threshold where aggressive reclaim begins.
 * Primarily a resource-control configuration value rather than usage telemetry.
 */
pub const CGROUP_MEMORY_HIGH_FILE_SLUG: &str = "memory.high";

/*
 * Defines best-effort memory protection for the cgroup.
 * Memory below this boundary is preferentially protected from reclaim.
 */
pub const CGROUP_MEMORY_LOW_FILE_SLUG: &str = "memory.low";

/*
 * Defines hard memory protection for the cgroup.
 * Memory below this boundary cannot normally be reclaimed from the cgroup.
 */
pub const CGROUP_MEMORY_MIN_FILE_SLUG: &str = "memory.min";

/*
 * Contains per-NUMA-node memory accounting for the cgroup.
 * Useful primarily when analyzing NUMA placement on multi-socket HPC systems.
 */
pub const CGROUP_MEMORY_NUMA_STAT_FILE_SLUG: &str = "memory.numa_stat";


// =================================================
// SWAP MEMORY
// =================================================

/*
 * Reports the amount of swap currently used by the cgroup.
 * Value is expressed in bytes.
 */
pub const CGROUP_MEMORY_SWAP_CURRENT_FILE_SLUG: &str =
    "memory.swap.current";

/*
 * Reports swap-related events for the cgroup.
 * Useful mainly when swap activity is important to workload analysis.
 */
pub const CGROUP_MEMORY_SWAP_EVENTS_FILE_SLUG: &str =
    "memory.swap.events";

/*
 * Defines the maximum amount of swap available to the cgroup.
 * This is primarily a resource-limit configuration interface.
 */
pub const CGROUP_MEMORY_SWAP_MAX_FILE_SLUG: &str =
    "memory.swap.max";


// =================================================
// I/O CONTROLLER
// =================================================

/*
 * Contains cumulative block-device I/O statistics for this cgroup.
 * Includes read/write bytes and operation counts separated by device.
 */
pub const CGROUP_IO_STAT_FILE_SLUG: &str = "io.stat";

/*
 * Reports I/O Pressure Stall Information for the cgroup.
 * Shows time tasks were stalled waiting for block I/O resources.
 */
pub const CGROUP_IO_PRESSURE_FILE_SLUG: &str = "io.pressure";

/*
 * Defines I/O bandwidth or IOPS limits for block devices used by the cgroup.
 * It controls resources rather than reporting actual I/O consumption.
 */
pub const CGROUP_IO_MAX_FILE_SLUG: &str = "io.max";

/*
 * Configures relative I/O scheduling weight for the cgroup.
 * This is configuration state rather than runtime usage information.
 */
pub const CGROUP_IO_WEIGHT_FILE_SLUG: &str = "io.weight";


// =================================================
// PID CONTROLLER
// =================================================

/*
 * Reports the current number of processes in the cgroup hierarchy.
 * Useful for tracking process counts associated with a container.
 */
pub const CGROUP_PIDS_CURRENT_FILE_SLUG: &str = "pids.current";

/*
 * Reports the largest number of processes observed in the cgroup hierarchy.
 * Useful for identifying peak process population for a container.
 */
pub const CGROUP_PIDS_PEAK_FILE_SLUG: &str = "pids.peak";

/*
 * Reports events where process creation encountered the configured PID limit.
 * Useful for diagnosing fork/clone failures caused by pids.max.
 */
pub const CGROUP_PIDS_EVENTS_FILE_SLUG: &str = "pids.events";

/*
 * Defines the maximum number of processes allowed in the cgroup hierarchy.
 * This is configuration rather than current process-count telemetry.
 */
pub const CGROUP_PIDS_MAX_FILE_SLUG: &str = "pids.max";


// =================================================
// CPUSET CONTROLLER
// =================================================

/*
 * Specifies CPUs explicitly assigned to the cgroup.
 * This represents scheduling configuration rather than consumed CPU resources.
 */
pub const CGROUP_CPUSET_CPUS_FILE_SLUG: &str = "cpuset.cpus";

/*
 * Reports the CPUs actually available to the cgroup after inheritance is applied.
 * Useful occasionally when diagnosing container CPU placement or CPU affinity.
 */
pub const CGROUP_CPUSET_CPUS_EFFECTIVE_FILE_SLUG: &str =
    "cpuset.cpus.effective";

/*
 * Specifies NUMA memory nodes assigned to the cgroup.
 * Relevant mainly to NUMA placement and affinity analysis.
 */
pub const CGROUP_CPUSET_MEMS_FILE_SLUG: &str = "cpuset.mems";

/*
 * Reports effective NUMA nodes available after hierarchy inheritance.
 * Useful for NUMA diagnostics but unnecessary for routine telemetry.
 */
pub const CGROUP_CPUSET_MEMS_EFFECTIVE_FILE_SLUG: &str =
    "cpuset.mems.effective";

```
