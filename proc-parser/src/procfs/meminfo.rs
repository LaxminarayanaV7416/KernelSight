use crate::common::file_reader::ProcFileReader;
use crate::common::kernel_types::UnsignedLong;
use crate::common::parser_utils::line_tracker;
use crate::common::procfs_constants::PROC_FS_ROOT_PATH;
use crate::configs::procfs_meminfo_config::ProcMemInfoConfig;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

/*
MemTotal %lu
       Total usable RAM (i.e., physical RAM minus a few
       reserved bits and the kernel binary code).

MemFree %lu
       The sum of LowFree+HighFree.

MemAvailable %lu (since Linux 3.14)
       An estimate of how much memory is available for
       starting new applications, without swapping.

Buffers %lu
       Relatively temporary storage for raw disk blocks
       that shouldn't get tremendously large (20 MB or so).

Cached %lu
       In-memory cache for files read from the disk (the
       page cache).  Doesn't include SwapCached.

SwapCached %lu
       Memory that once was swapped out, is swapped back in
       but still also is in the swap file.  (If memory
       pressure is high, these pages don't need to be
       swapped out again because they are already in the
       swap file.  This saves I/O.)

Active %lu
       Memory that has been used more recently and usually
       not reclaimed unless absolutely necessary.

Inactive %lu
       Memory which has been less recently used.  It is
       more eligible to be reclaimed for other purposes.

Active(anon) %lu (since Linux 2.6.28)
       [To be documented.]

Inactive(anon) %lu (since Linux 2.6.28)
       [To be documented.]

Active(file) %lu (since Linux 2.6.28)
       [To be documented.]

Inactive(file) %lu (since Linux 2.6.28)
       [To be documented.]

Unevictable %lu (since Linux 2.6.28)
       (From Linux 2.6.28 to Linux 2.6.30,
       CONFIG_UNEVICTABLE_LRU was required.)  [To be
       documented.]

Mlocked %lu (since Linux 2.6.28)
       (From Linux 2.6.28 to Linux 2.6.30,
       CONFIG_UNEVICTABLE_LRU was required.)  [To be
       documented.]

HighTotal %lu
       (Starting with Linux 2.6.19, CONFIG_HIGHMEM is
       required.)  Total amount of highmem.  Highmem is all
       memory above ~860 MB of physical memory.  Highmem
       areas are for use by user-space programs, or for the
       page cache.  The kernel must use tricks to access
       this memory, making it slower to access than lowmem.

HighFree %lu
       (Starting with Linux 2.6.19, CONFIG_HIGHMEM is
       required.)  Amount of free highmem.

LowTotal %lu
       (Starting with Linux 2.6.19, CONFIG_HIGHMEM is
       required.)  Total amount of lowmem.  Lowmem is
       memory which can be used for everything that highmem
       can be used for, but it is also available for the
       kernel's use for its own data structures.  Among
       many other things, it is where everything from Slab
       is allocated.  Bad things happen when you're out of
       lowmem.

LowFree %lu
       (Starting with Linux 2.6.19, CONFIG_HIGHMEM is
       required.)  Amount of free lowmem.

MmapCopy %lu (since Linux 2.6.29)
       (CONFIG_MMU is required.)  [To be documented.]

SwapTotal %lu
       Total amount of swap space available.

SwapFree %lu
       Amount of swap space that is currently unused.

Dirty %lu
       Memory which is waiting to get written back to the
       disk.

Writeback %lu
       Memory which is actively being written back to the
       disk.

AnonPages %lu (since Linux 2.6.18)
       Non-file backed pages mapped into user-space page
       tables.

Mapped %lu
       Files which have been mapped into memory (with
       mmap(2)), such as libraries.

Shmem %lu (since Linux 2.6.32)
       Amount of memory consumed in tmpfs(5) filesystems,
       System V, and POSIX shared memory, as well as shared
       anonymous mappings (MAP_SHARED|MAP_ANONYMOUS).

KReclaimable %lu (since Linux 4.20)
       Kernel allocations that the kernel will attempt to
       reclaim under memory pressure.  Includes
       SReclaimable (below), and other direct allocations
       with a shrinker.

Slab %lu
       In-kernel data structures cache.  (See slabinfo(5).)

SReclaimable %lu (since Linux 2.6.19)
       Part of Slab, that might be reclaimed, such as
       caches.

SUnreclaim %lu (since Linux 2.6.19)
       Part of Slab, that cannot be reclaimed on memory
       pressure.

KernelStack %lu (since Linux 2.6.32)
       Amount of memory allocated to kernel stacks.

PageTables %lu (since Linux 2.6.18)
       Amount of memory dedicated to the lowest level of
       page tables.

Quicklists %lu (since Linux 2.6.27)
       (CONFIG_QUICKLIST is required.)  [To be documented.]

NFS_Unstable %lu (since Linux 2.6.18)
       NFS pages sent to the server, but not yet committed
       to stable storage.

Bounce %lu (since Linux 2.6.18)
       Memory used for block device "bounce buffers".

WritebackTmp %lu (since Linux 2.6.26)
       Memory used by FUSE for temporary writeback buffers.

CommitLimit %lu (since Linux 2.6.10)
       This is the total amount of memory currently
       available to be allocated on the system, expressed
       in kilobytes.  This limit is adhered to only if
       strict overcommit accounting is enabled (mode 2 in
       /proc/sys/vm/overcommit_memory).  The limit is
       calculated according to the formula described under
       /proc/sys/vm/overcommit_memory.  For further
       details, see the kernel source file
       Documentation/vm/overcommit-accounting.rst.

Committed_AS %lu
       The amount of memory presently allocated on the
       system.  The committed memory is a sum of all of the
       memory which has been allocated by processes, even
       if it has not been "used" by them as of yet.  A
       process which allocates 1 GB of memory (using
       malloc(3) or similar), but touches only 300 MB of
       that memory will show up as using only 300 MB of
       memory even if it has the address space allocated
       for the entire 1 GB.

       This 1 GB is memory which has been "committed" to by
       the VM and can be used at any time by the allocating
       application.  With strict overcommit enabled on the
       system (mode 2 in /proc/sys/vm/overcommit_memory),
       allocations which would exceed the CommitLimit will
       not be permitted.  This is useful if one needs to
       guarantee that processes will not fail due to lack
       of memory once that memory has been successfully
       allocated.

VmallocTotal %lu
       Total size of vmalloc memory area.

VmallocUsed %lu
       Amount of vmalloc area which is used.  Since Linux
       4.4, this field is no longer calculated, and is hard
       coded as 0.  See /proc/vmallocinfo.

VmallocChunk %lu
       Largest contiguous block of vmalloc area which is
       free.  Since Linux 4.4, this field is no longer
       calculated and is hard coded as 0.  See
       /proc/vmallocinfo.

HardwareCorrupted %lu (since Linux 2.6.32)
       (CONFIG_MEMORY_FAILURE is required.)  [To be
       documented.]

LazyFree %lu (since Linux 4.12)
       Shows the amount of memory marked by madvise(2)
       MADV_FREE.

AnonHugePages %lu (since Linux 2.6.38)
       (CONFIG_TRANSPARENT_HUGEPAGE is required.)  Non-file
       backed huge pages mapped into user-space page
       tables.

ShmemHugePages %lu (since Linux 4.8)
       (CONFIG_TRANSPARENT_HUGEPAGE is required.)  Memory
       used by shared memory (shmem) and tmpfs(5) allocated
       with huge pages.

ShmemPmdMapped %lu (since Linux 4.8)
       (CONFIG_TRANSPARENT_HUGEPAGE is required.)  Shared
       memory mapped into user space with huge pages.

CmaTotal %lu (since Linux 3.1)
       Total CMA (Contiguous Memory Allocator) pages.
       (CONFIG_CMA is required.)

CmaFree %lu (since Linux 3.1)
       Free CMA (Contiguous Memory Allocator) pages.
       (CONFIG_CMA is required.)

HugePages_Total %lu
       (CONFIG_HUGETLB_PAGE is required.)  The size of the
       pool of huge pages.

HugePages_Free %lu
       (CONFIG_HUGETLB_PAGE is required.)  The number of
       huge pages in the pool that are not yet allocated.

HugePages_Rsvd %lu (since Linux 2.6.17)
       (CONFIG_HUGETLB_PAGE is required.)  This is the
       number of huge pages for which a commitment to
       allocate from the pool has been made, but no
       allocation has yet been made.  These reserved huge
       pages guarantee that an application will be able to
       allocate a huge page from the pool of huge pages at
       fault time.

HugePages_Surp %lu (since Linux 2.6.24)
       (CONFIG_HUGETLB_PAGE is required.)  This is the
       number of huge pages in the pool above the value in
       /proc/sys/vm/nr_hugepages.  The maximum number of
       surplus huge pages is controlled by
       /proc/sys/vm/nr_overcommit_hugepages.

Hugepagesize %lu
       (CONFIG_HUGETLB_PAGE is required.)  The size of huge
       pages.

DirectMap4k %lu (since Linux 2.6.27)
       Number of bytes of RAM linearly mapped by kernel in
       4 kB pages.  (x86.)

DirectMap4M %lu (since Linux 2.6.27)
       Number of bytes of RAM linearly mapped by kernel in
       4 MB pages.  (x86 with CONFIG_X86_64 or
       CONFIG_X86_PAE enabled.)

DirectMap2M %lu (since Linux 2.6.27)
       Number of bytes of RAM linearly mapped by kernel in
       2 MB pages.  (x86 with neither CONFIG_X86_64 nor
       CONFIG_X86_PAE enabled.)

DirectMap1G %lu (since Linux 2.6.27)
       (x86 with CONFIG_X86_64 and
       CONFIG_X86_DIRECT_GBPAGES enabled.)

LAX Notes:
kB -> 107,66
Space -> 32
Newline -> 10
 */

pub struct ProcFSMemInfoReader {
    reader: ProcFileReader<4096>,
    values: ProcMemInfoFields,
    lines_map: Option<HashMap<usize, usize>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcMemInfoFields {
    pub mem_total: UnsignedLong,
    pub mem_free: UnsignedLong,
    pub mem_available: UnsignedLong,
    pub mem_buffers: UnsignedLong,
    pub mem_cached: UnsignedLong,
    pub mem_swap_cached: UnsignedLong,

    pub mem_active: UnsignedLong,
    pub mem_inactive: UnsignedLong,
    pub mem_active_anon: UnsignedLong,
    pub mem_inactive_anon: UnsignedLong,
    pub mem_active_file: UnsignedLong,
    pub mem_inactive_file: UnsignedLong,
    pub mem_unevictable: UnsignedLong,
    pub mem_mlocked: UnsignedLong,

    pub mem_swap_total: UnsignedLong,
    pub mem_swap_free: UnsignedLong,
    pub mem_zswap: UnsignedLong,
    pub mem_zswapped: UnsignedLong,

    pub mem_dirty: UnsignedLong,
    pub mem_write_back: UnsignedLong,
    pub mem_anon_pages: UnsignedLong,
    pub mem_mapped: UnsignedLong,
    pub mem_shmem: UnsignedLong,

    pub mem_k_reclaimable: UnsignedLong,
    pub mem_slab: UnsignedLong,
    pub mem_s_reclaimable: UnsignedLong,
    pub mem_s_unreclaim: UnsignedLong,

    pub mem_kernel_stack: UnsignedLong,
    pub mem_page_tables: UnsignedLong,
    pub mem_sec_page_tables: UnsignedLong,

    pub mem_nfs_unstable: UnsignedLong,
    pub mem_bounce: UnsignedLong,
    pub mem_write_back_tmp: UnsignedLong,

    pub mem_commit_limit: UnsignedLong,
    pub mem_committed_as: UnsignedLong,

    pub mem_vmalloc_total: UnsignedLong,
    pub mem_vmalloc_used: UnsignedLong,
    pub mem_vmalloc_chunk: UnsignedLong,

    pub mem_percpu: UnsignedLong,
    pub mem_hardware_corrupted: UnsignedLong,

    pub mem_anon_huge_pages: UnsignedLong,
    pub mem_shmem_huge_pages: UnsignedLong,
    pub mem_shmem_pmd_mapped: UnsignedLong,
    pub mem_file_huge_pages: UnsignedLong,
    pub mem_file_pmd_mapped: UnsignedLong,

    pub mem_cma_total: UnsignedLong,
    pub mem_cma_free: UnsignedLong,

    pub mem_unaccepted: UnsignedLong,
    pub mem_balloon: UnsignedLong,
    pub mem_gpu_active: UnsignedLong,
    pub mem_gpu_reclaim: UnsignedLong,

    pub mem_huge_pages_total: UnsignedLong,
    pub mem_huge_pages_free: UnsignedLong,
    pub mem_huge_pages_reserved: UnsignedLong,
    pub mem_huge_pages_surp: UnsignedLong,
    pub mem_huge_page_size: UnsignedLong,
    pub mem_hugetlb: UnsignedLong,

    pub mem_direct_map_4k: UnsignedLong,
    pub mem_direct_map_2m: UnsignedLong,
    pub mem_direct_map_1g: UnsignedLong,
}

impl ProcFSMemInfoReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: ProcMemInfoFields::default(),
            lines_map: None,
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<&str, (usize, bool)>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &ProcMemInfoFields {
        &self.values
    }

    fn set_field(&mut self, field: usize, value: u64) {
        match field {
            1 => self.values.mem_total = value,
            2 => self.values.mem_free = value,
            3 => self.values.mem_available = value,
            4 => self.values.mem_buffers = value,
            5 => self.values.mem_cached = value,
            6 => self.values.mem_swap_cached = value,

            7 => self.values.mem_active = value,
            8 => self.values.mem_inactive = value,
            9 => self.values.mem_active_anon = value,
            10 => self.values.mem_inactive_anon = value,
            11 => self.values.mem_active_file = value,
            12 => self.values.mem_inactive_file = value,
            13 => self.values.mem_unevictable = value,
            14 => self.values.mem_mlocked = value,

            15 => self.values.mem_swap_total = value,
            16 => self.values.mem_swap_free = value,
            17 => self.values.mem_zswap = value,
            18 => self.values.mem_zswapped = value,

            19 => self.values.mem_dirty = value,
            20 => self.values.mem_write_back = value,
            21 => self.values.mem_anon_pages = value,
            22 => self.values.mem_mapped = value,
            23 => self.values.mem_shmem = value,

            24 => self.values.mem_k_reclaimable = value,
            25 => self.values.mem_slab = value,
            26 => self.values.mem_s_reclaimable = value,
            27 => self.values.mem_s_unreclaim = value,

            28 => self.values.mem_kernel_stack = value,
            29 => self.values.mem_page_tables = value,
            30 => self.values.mem_sec_page_tables = value,

            31 => self.values.mem_nfs_unstable = value,
            32 => self.values.mem_bounce = value,
            33 => self.values.mem_write_back_tmp = value,

            34 => self.values.mem_commit_limit = value,
            35 => self.values.mem_committed_as = value,

            36 => self.values.mem_vmalloc_total = value,
            37 => self.values.mem_vmalloc_used = value,
            38 => self.values.mem_vmalloc_chunk = value,

            39 => self.values.mem_percpu = value,
            40 => self.values.mem_hardware_corrupted = value,

            41 => self.values.mem_anon_huge_pages = value,
            42 => self.values.mem_shmem_huge_pages = value,
            43 => self.values.mem_shmem_pmd_mapped = value,
            44 => self.values.mem_file_huge_pages = value,
            45 => self.values.mem_file_pmd_mapped = value,

            46 => self.values.mem_cma_total = value,
            47 => self.values.mem_cma_free = value,

            48 => self.values.mem_unaccepted = value,
            49 => self.values.mem_balloon = value,
            50 => self.values.mem_gpu_active = value,
            51 => self.values.mem_gpu_reclaim = value,

            52 => self.values.mem_huge_pages_total = value,
            53 => self.values.mem_huge_pages_free = value,
            54 => self.values.mem_huge_pages_reserved = value,
            55 => self.values.mem_huge_pages_surp = value,
            56 => self.values.mem_huge_page_size = value,
            57 => self.values.mem_hugetlb = value,

            58 => self.values.mem_direct_map_4k = value,
            59 => self.values.mem_direct_map_2m = value,
            60 => self.values.mem_direct_map_1g = value,

            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<&str, (usize, bool)>) {
        // Build the line-to-field mapping once. Subsequent reads reuse it.
        if self.lines_map.is_none() {
            self.lines_map = Some(line_tracker(
                &self.reader.buffer[..self.reader.buffer_len],
                field_filter,
                b':',
            ));
        }

        let mut parser_line_number = 0usize;
        let mut line_started = false;
        let mut field_number = 0usize;
        let mut seen_colon = false;
        let mut digit_count = 0usize;
        let mut digits_finished = false;
        let mut value = Some(0u64);

        for i in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[i];

            if byte == b'\n' || byte == b'\0' {
                if line_started && field_number > 0 && digit_count > 0 {
                    self.set_field(field_number, value.unwrap_or_default());
                }

                line_started = false;
                field_number = 0;
                seen_colon = false;
                digit_count = 0;
                digits_finished = false;
                value = Some(0);
                continue;
            }

            if !line_started {
                line_started = true;
                parser_line_number += 1;

                field_number = self
                    .lines_map
                    .as_ref()
                    .and_then(|lines_map| lines_map.get(&parser_line_number))
                    .copied()
                    .unwrap_or(0);
            }

            // Do not spend time parsing values for fields that are not selected.
            if field_number == 0 || digits_finished {
                continue;
            }

            if !seen_colon {
                if byte == b':' {
                    seen_colon = true;
                }
                continue;
            }

            match byte {
                b'0'..=b'9' => {
                    digit_count += 1;

                    // Preserve parse_u64_swar's previous limit of 20 digits.
                    if digit_count > 20 {
                        value = None;
                    } else {
                        let digit = u64::from(byte - b'0');

                        value = value
                            .and_then(|current| current.checked_mul(10))
                            .and_then(|current| current.checked_add(digit));
                    }
                }
                _ if digit_count > 0 => {
                    // Stop at the first non-digit after the number.
                    digits_finished = true;
                }
                _ => {
                    // Skip whitespace or other characters before the value.
                }
            }
        }

        // The final line may not have a trailing newline.
        if line_started && field_number > 0 && digit_count > 0 {
            self.set_field(field_number, value.unwrap_or_default());
        }
    }
}

pub fn parse_procfs_meminfo(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
) -> JoinHandle<()> {
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH).join("meminfo");
    let load_avg_temp_reader =
        ProcFSMemInfoReader::new(&load_avg_path.to_str().unwrap(), is_cachable, is_root);
    let mut load_avg_reader = match load_avg_temp_reader {
        Ok(reader) => Box::new(reader),
        Err(e) => panic!("Failed to create loadavg reader: {}", e),
    };
    // TODO: from here get the config which have HashMap<&str, (usize, bool)>
    // Just use the method get_field_string_to_struct_ids
    // let filter_map = ProcMemInfoConfig.get_field_string_to_struct_ids()
    let filter_map = HashMap::from([
        ("MemTotal", (1, true)),
        ("MemFree", (2, true)),
        ("MemAvailable", (3, true)),
        ("Buffers", (4, true)),
        ("Cached", (5, true)),
        ("SwapCached", (6, true)),
        ("Active", (7, true)),
        ("Inactive", (8, true)),
        ("Active(anon)", (9, true)),
        ("Inactive(anon)", (10, true)),
        ("Active(file)", (11, true)),
        ("Inactive(file)", (12, true)),
        ("Unevictable", (13, true)),
        ("Mlocked", (14, true)),
        ("SwapTotal", (15, true)),
        ("SwapFree", (16, true)),
        ("Zswap", (17, true)),
        ("Zswapped", (18, true)),
        ("Dirty", (19, true)),
        ("Writeback", (20, true)),
        ("AnonPages", (21, true)),
        ("Mapped", (22, true)),
        ("Shmem", (23, true)),
        ("KReclaimable", (24, true)),
        ("Slab", (25, true)),
        ("SReclaimable", (26, true)),
        ("SUnreclaim", (27, true)),
        ("KernelStack", (28, true)),
        ("PageTables", (29, true)),
        ("SecPageTables", (30, true)),
        ("NFS_Unstable", (31, true)),
        ("Bounce", (32, true)),
        ("WritebackTmp", (33, true)),
        ("CommitLimit", (34, true)),
        ("Committed_AS", (35, true)),
        ("VmallocTotal", (36, true)),
        ("VmallocUsed", (37, true)),
        ("VmallocChunk", (38, true)),
        ("Percpu", (39, true)),
        ("HardwareCorrupted", (40, true)),
        ("AnonHugePages", (41, true)),
        ("ShmemHugePages", (42, true)),
        ("ShmemPmdMapped", (43, true)),
        ("FileHugePages", (44, true)),
        ("FilePmdMapped", (45, true)),
        ("CmaTotal", (46, true)),
        ("CmaFree", (47, true)),
        ("Unaccepted", (48, true)),
        ("Balloon", (49, true)),
        ("GPUActive", (50, true)),
        ("GPUReclaim", (51, true)),
        ("HugePages_Total", (52, true)),
        ("HugePages_Free", (53, true)),
        ("HugePages_Rsvd", (54, true)),
        ("HugePages_Surp", (55, true)),
        ("Hugepagesize", (56, true)),
        ("Hugetlb", (57, true)),
        ("DirectMap4k", (58, true)),
        ("DirectMap2M", (59, true)),
        ("DirectMap1G", (60, true)),
    ]);
    let thread_handle = thread::spawn(move || {
        while !signal_hook.load(Ordering::Relaxed) {
            let read_status = load_avg_reader.read_and_parse(&filter_map);
            if read_status {
                println!("Parsed: {:?}", load_avg_reader.values());
                println!("=====================================");
                thread::sleep(std::time::Duration::from_millis(monitoring_heartbeat));
            } else {
                println!("Failed to read meminfo, ProcFS File closed!");
                break;
            }
        }
    });
    println!("Killed LoadAvg thread!!");
    thread_handle
}
