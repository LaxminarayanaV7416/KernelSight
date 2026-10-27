use crate::common::file_reader::ProcFileReader;
use crate::common::parser_error::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/*
Field Number
Field 1 - Float
    1-Minute Load Average
    The average number of scheduling entities (processes/threads) that were either executable (State R)
    or blocked waiting for disk I/O (State D) over the last minute.
Field 2 - Float
    5-Minute Load Average
    The same workload metric averaged over the last 5 minutes.
Field 3 - Float
    15-Minute Load Average
    The same workload metric averaged over the last 15 minutes.
Field 4 - int32/int32
    Runnable / Total Entities
    Consists of two numbers separated by a slash (/):
    Numerator: The number of kernel scheduling entities currently executable and waiting to run.
    Denominator: The total number of kernel scheduling entities that currently exist on the system.
Field 5 - int32
    Last Created PID
    The Process ID (PID) of the most recently allocated process or thread on the system.
 */

#[derive(Debug)]
pub struct ProcFSVmStatReader {
    reader: ProcFileReader<16384>,
    values: Arc<Mutex<ProcVmStatFields>>,
}

#[derive(Debug, Default)]
pub struct ProcVmStatFields {
    pub nr_free_pages: u64,
    pub nr_free_pages_blocks: u64,
    pub nr_zone_inactive_anon: u64,
    pub nr_zone_active_anon: u64,
    pub nr_zone_inactive_file: u64,
    pub nr_zone_active_file: u64,
    pub nr_zone_unevictable: u64,
    pub nr_zone_write_pending: u64,
    pub nr_mlock: u64,
    pub nr_zspages: u64,
    pub nr_free_cma: u64,
    pub nr_unaccepted: u64,
    pub numa_hit: u64,
    pub numa_miss: u64,
    pub numa_foreign: u64,
    pub numa_interleave: u64,
    pub numa_local: u64,
    pub numa_other: u64,
    pub nr_inactive_anon: u64,
    pub nr_active_anon: u64,
    pub nr_inactive_file: u64,
    pub nr_active_file: u64,
    pub nr_unevictable: u64,
    pub nr_slab_reclaimable: u64,
    pub nr_slab_unreclaimable: u64,
    pub nr_isolated_anon: u64,
    pub nr_isolated_file: u64,
    pub workingset_nodes: u64,
    pub workingset_refault_anon: u64,
    pub workingset_refault_file: u64,
    pub workingset_activate_anon: u64,
    pub workingset_activate_file: u64,
    pub workingset_restore_anon: u64,
    pub workingset_restore_file: u64,
    pub workingset_nodereclaim: u64,
    pub nr_anon_pages: u64,
    pub nr_mapped: u64,
    pub nr_file_pages: u64,
    pub nr_dirty: u64,
    pub nr_writeback: u64,
    pub nr_shmem: u64,
    pub nr_shmem_hugepages: u64,
    pub nr_shmem_pmdmapped: u64,
    pub nr_file_hugepages: u64,
    pub nr_file_pmdmapped: u64,
    pub nr_anon_transparent_hugepages: u64,
    pub nr_vmscan_write: u64,
    pub nr_vmscan_immediate_reclaim: u64,
    pub nr_dirtied: u64,
    pub nr_written: u64,
    pub nr_throttled_written: u64,
    pub nr_kernel_misc_reclaimable: u64,
    pub nr_foll_pin_acquired: u64,
    pub nr_foll_pin_released: u64,
    pub nr_vmalloc: u64,
    pub nr_kernel_stack: u64,
    pub nr_page_table_pages: u64,
    pub nr_sec_page_table_pages: u64,
    pub nr_iommu_pages: u64,
    pub nr_swapcached: u64,
    pub pgpromote_success: u64,
    pub pgpromote_candidate: u64,
    pub pgpromote_candidate_nrl: u64,
    pub pgdemote_kswapd: u64,
    pub pgdemote_direct: u64,
    pub pgdemote_khugepaged: u64,
    pub pgdemote_proactive: u64,
    pub pgsteal_kswapd: u64,
    pub pgsteal_direct: u64,
    pub pgsteal_khugepaged: u64,
    pub pgsteal_proactive: u64,
    pub pgsteal_anon: u64,
    pub pgsteal_file: u64,
    pub pgscan_kswapd: u64,
    pub pgscan_direct: u64,
    pub pgscan_khugepaged: u64,
    pub pgscan_proactive: u64,
    pub pgscan_anon: u64,
    pub pgscan_file: u64,
    pub pgrefill: u64,
    pub nr_hugetlb: u64,
    pub nr_balloon_pages: u64,
    pub nr_kernel_file_pages: u64,
    pub nr_gpu_active: u64,
    pub nr_gpu_reclaim: u64,
    pub nr_dirty_threshold: u64,
    pub nr_dirty_background_threshold: u64,
    pub nr_memmap_pages: u64,
    pub nr_memmap_boot_pages: u64,
    pub pgpgin: u64,
    pub pgpgout: u64,
    pub pswpin: u64,
    pub pswpout: u64,
    pub pgalloc_dma: u64,
    pub pgalloc_dma32: u64,
    pub pgalloc_normal: u64,
    pub pgalloc_movable: u64,
    pub pgalloc_device: u64,
    pub allocstall_dma: u64,
    pub allocstall_dma32: u64,
    pub allocstall_normal: u64,
    pub allocstall_movable: u64,
    pub allocstall_device: u64,
    pub pgskip_dma: u64,
    pub pgskip_dma32: u64,
    pub pgskip_normal: u64,
    pub pgskip_movable: u64,
    pub pgskip_device: u64,
    pub pgfree: u64,
    pub pgactivate: u64,
    pub pgdeactivate: u64,
    pub pglazyfree: u64,
    pub pgfault: u64,
    pub pgmajfault: u64,
    pub pglazyfreed: u64,
    pub pgreuse: u64,
    pub pgscan_direct_throttle: u64,
    pub zone_reclaim_success: u64,
    pub zone_reclaim_failed: u64,
    pub pginodesteal: u64,
    pub slabs_scanned: u64,
    pub kswapd_inodesteal: u64,
    pub kswapd_low_wmark_hit_quickly: u64,
    pub kswapd_high_wmark_hit_quickly: u64,
    pub pageoutrun: u64,
    pub pgrotated: u64,
    pub drop_pagecache: u64,
    pub drop_slab: u64,
    pub oom_kill: u64,
    pub numa_pte_updates: u64,
    pub numa_huge_pte_updates: u64,
    pub numa_hint_faults: u64,
    pub numa_hint_faults_local: u64,
    pub numa_pages_migrated: u64,
    pub pgmigrate_success: u64,
    pub pgmigrate_fail: u64,
    pub thp_migration_success: u64,
    pub thp_migration_fail: u64,
    pub thp_migration_split: u64,
    pub compact_migrate_scanned: u64,
    pub compact_free_scanned: u64,
    pub compact_isolated: u64,
    pub compact_stall: u64,
    pub compact_fail: u64,
    pub compact_success: u64,
    pub compact_daemon_wake: u64,
    pub compact_daemon_migrate_scanned: u64,
    pub compact_daemon_free_scanned: u64,
    pub htlb_buddy_alloc_success: u64,
    pub htlb_buddy_alloc_fail: u64,
    pub cma_alloc_success: u64,
    pub cma_alloc_fail: u64,
    pub unevictable_pgs_culled: u64,
    pub unevictable_pgs_scanned: u64,
    pub unevictable_pgs_rescued: u64,
    pub unevictable_pgs_mlocked: u64,
    pub unevictable_pgs_munlocked: u64,
    pub unevictable_pgs_cleared: u64,
    pub unevictable_pgs_stranded: u64,
    pub thp_fault_alloc: u64,
    pub thp_fault_fallback: u64,
    pub thp_fault_fallback_charge: u64,
    pub thp_collapse_alloc: u64,
    pub thp_collapse_alloc_failed: u64,
    pub thp_file_alloc: u64,
    pub thp_file_fallback: u64,
    pub thp_file_fallback_charge: u64,
    pub thp_file_mapped: u64,
    pub thp_split_page: u64,
    pub thp_split_page_failed: u64,
    pub thp_deferred_split_page: u64,
    pub thp_underused_split_page: u64,
    pub thp_split_pmd: u64,
    pub thp_scan_exceed_none_pte: u64,
    pub thp_scan_exceed_swap_pte: u64,
    pub thp_scan_exceed_share_pte: u64,
    pub thp_split_pud: u64,
    pub thp_zero_page_alloc: u64,
    pub thp_zero_page_alloc_failed: u64,
    pub thp_swpout: u64,
    pub thp_swpout_fallback: u64,
    pub balloon_inflate: u64,
    pub balloon_deflate: u64,
    pub balloon_migrate: u64,
    pub swap_ra: u64,
    pub swap_ra_hit: u64,
    pub swpin_zero: u64,
    pub swpout_zero: u64,
    pub ksm_swpin_copy: u64,
    pub cow_ksm: u64,
    pub zswpin: u64,
    pub zswpout: u64,
    pub zswpwb: u64,
    pub direct_map_level2_splits: u64,
    pub direct_map_level3_splits: u64,
    pub direct_map_level2_collapses: u64,
    pub direct_map_level3_collapses: u64,
    pub nr_unstable: u64,
}

impl ProcFSVmStatReader {
    pub fn new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            reader: ProcFileReader::new(path)?,
            values: Arc::new(Mutex::new(ProcVmStatFields::default())),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            if let Ok(mut values) = self.values.lock() {
                self.parse_buffer(&mut *values, &field_filter);
            }
        }
        read_status
    }

    pub fn values(&self) -> &Arc<Mutex<ProcVmStatFields>> {
        &self.values
    }

    fn set_field(&self, values: &mut ProcVmStatFields, field: usize, value: u64) {
        match field {
            1 => values.nr_free_pages = value,
            2 => values.nr_free_pages_blocks = value,
            3 => values.nr_zone_inactive_anon = value,
            4 => values.nr_zone_active_anon = value,
            5 => values.nr_zone_inactive_file = value,
            6 => values.nr_zone_active_file = value,
            7 => values.nr_zone_unevictable = value,
            8 => values.nr_zone_write_pending = value,
            9 => values.nr_mlock = value,
            10 => values.nr_zspages = value,
            11 => values.nr_free_cma = value,
            12 => values.nr_unaccepted = value,
            13 => values.numa_hit = value,
            14 => values.numa_miss = value,
            15 => values.numa_foreign = value,
            16 => values.numa_interleave = value,
            17 => values.numa_local = value,
            18 => values.numa_other = value,
            19 => values.nr_inactive_anon = value,
            20 => values.nr_active_anon = value,
            21 => values.nr_inactive_file = value,
            22 => values.nr_active_file = value,
            23 => values.nr_unevictable = value,
            24 => values.nr_slab_reclaimable = value,
            25 => values.nr_slab_unreclaimable = value,
            26 => values.nr_isolated_anon = value,
            27 => values.nr_isolated_file = value,
            28 => values.workingset_nodes = value,
            29 => values.workingset_refault_anon = value,
            30 => values.workingset_refault_file = value,
            31 => values.workingset_activate_anon = value,
            32 => values.workingset_activate_file = value,
            33 => values.workingset_restore_anon = value,
            34 => values.workingset_restore_file = value,
            35 => values.workingset_nodereclaim = value,
            36 => values.nr_anon_pages = value,
            37 => values.nr_mapped = value,
            38 => values.nr_file_pages = value,
            39 => values.nr_dirty = value,
            40 => values.nr_writeback = value,
            41 => values.nr_shmem = value,
            42 => values.nr_shmem_hugepages = value,
            43 => values.nr_shmem_pmdmapped = value,
            44 => values.nr_file_hugepages = value,
            45 => values.nr_file_pmdmapped = value,
            46 => values.nr_anon_transparent_hugepages = value,
            47 => values.nr_vmscan_write = value,
            48 => values.nr_vmscan_immediate_reclaim = value,
            49 => values.nr_dirtied = value,
            50 => values.nr_written = value,
            51 => values.nr_throttled_written = value,
            52 => values.nr_kernel_misc_reclaimable = value,
            53 => values.nr_foll_pin_acquired = value,
            54 => values.nr_foll_pin_released = value,
            55 => values.nr_vmalloc = value,
            56 => values.nr_kernel_stack = value,
            57 => values.nr_page_table_pages = value,
            58 => values.nr_sec_page_table_pages = value,
            59 => values.nr_iommu_pages = value,
            60 => values.nr_swapcached = value,
            61 => values.pgpromote_success = value,
            62 => values.pgpromote_candidate = value,
            63 => values.pgpromote_candidate_nrl = value,
            64 => values.pgdemote_kswapd = value,
            65 => values.pgdemote_direct = value,
            66 => values.pgdemote_khugepaged = value,
            67 => values.pgdemote_proactive = value,
            68 => values.pgsteal_kswapd = value,
            69 => values.pgsteal_direct = value,
            70 => values.pgsteal_khugepaged = value,
            71 => values.pgsteal_proactive = value,
            72 => values.pgsteal_anon = value,
            73 => values.pgsteal_file = value,
            74 => values.pgscan_kswapd = value,
            75 => values.pgscan_direct = value,
            76 => values.pgscan_khugepaged = value,
            77 => values.pgscan_proactive = value,
            78 => values.pgscan_anon = value,
            79 => values.pgscan_file = value,
            80 => values.pgrefill = value,
            81 => values.nr_hugetlb = value,
            82 => values.nr_balloon_pages = value,
            83 => values.nr_kernel_file_pages = value,
            84 => values.nr_gpu_active = value,
            85 => values.nr_gpu_reclaim = value,
            86 => values.nr_dirty_threshold = value,
            87 => values.nr_dirty_background_threshold = value,
            88 => values.nr_memmap_pages = value,
            89 => values.nr_memmap_boot_pages = value,
            90 => values.pgpgin = value,
            91 => values.pgpgout = value,
            92 => values.pswpin = value,
            93 => values.pswpout = value,
            94 => values.pgalloc_dma = value,
            95 => values.pgalloc_dma32 = value,
            96 => values.pgalloc_normal = value,
            97 => values.pgalloc_movable = value,
            98 => values.pgalloc_device = value,
            99 => values.allocstall_dma = value,
            100 => values.allocstall_dma32 = value,
            101 => values.allocstall_normal = value,
            102 => values.allocstall_movable = value,
            103 => values.allocstall_device = value,
            104 => values.pgskip_dma = value,
            105 => values.pgskip_dma32 = value,
            106 => values.pgskip_normal = value,
            107 => values.pgskip_movable = value,
            108 => values.pgskip_device = value,
            109 => values.pgfree = value,
            110 => values.pgactivate = value,
            111 => values.pgdeactivate = value,
            112 => values.pglazyfree = value,
            113 => values.pgfault = value,
            114 => values.pgmajfault = value,
            115 => values.pglazyfreed = value,
            116 => values.pgreuse = value,
            117 => values.pgscan_direct_throttle = value,
            118 => values.zone_reclaim_success = value,
            119 => values.zone_reclaim_failed = value,
            120 => values.pginodesteal = value,
            121 => values.slabs_scanned = value,
            122 => values.kswapd_inodesteal = value,
            123 => values.kswapd_low_wmark_hit_quickly = value,
            124 => values.kswapd_high_wmark_hit_quickly = value,
            125 => values.pageoutrun = value,
            126 => values.pgrotated = value,
            127 => values.drop_pagecache = value,
            128 => values.drop_slab = value,
            129 => values.oom_kill = value,
            130 => values.numa_pte_updates = value,
            131 => values.numa_huge_pte_updates = value,
            132 => values.numa_hint_faults = value,
            133 => values.numa_hint_faults_local = value,
            134 => values.numa_pages_migrated = value,
            135 => values.pgmigrate_success = value,
            136 => values.pgmigrate_fail = value,
            137 => values.thp_migration_success = value,
            138 => values.thp_migration_fail = value,
            139 => values.thp_migration_split = value,
            140 => values.compact_migrate_scanned = value,
            141 => values.compact_free_scanned = value,
            142 => values.compact_isolated = value,
            143 => values.compact_stall = value,
            144 => values.compact_fail = value,
            145 => values.compact_success = value,
            146 => values.compact_daemon_wake = value,
            147 => values.compact_daemon_migrate_scanned = value,
            148 => values.compact_daemon_free_scanned = value,
            149 => values.htlb_buddy_alloc_success = value,
            150 => values.htlb_buddy_alloc_fail = value,
            151 => values.cma_alloc_success = value,
            152 => values.cma_alloc_fail = value,
            153 => values.unevictable_pgs_culled = value,
            154 => values.unevictable_pgs_scanned = value,
            155 => values.unevictable_pgs_rescued = value,
            156 => values.unevictable_pgs_mlocked = value,
            157 => values.unevictable_pgs_munlocked = value,
            158 => values.unevictable_pgs_cleared = value,
            159 => values.unevictable_pgs_stranded = value,
            160 => values.thp_fault_alloc = value,
            161 => values.thp_fault_fallback = value,
            162 => values.thp_fault_fallback_charge = value,
            163 => values.thp_collapse_alloc = value,
            164 => values.thp_collapse_alloc_failed = value,
            165 => values.thp_file_alloc = value,
            166 => values.thp_file_fallback = value,
            167 => values.thp_file_fallback_charge = value,
            168 => values.thp_file_mapped = value,
            169 => values.thp_split_page = value,
            170 => values.thp_split_page_failed = value,
            171 => values.thp_deferred_split_page = value,
            172 => values.thp_underused_split_page = value,
            173 => values.thp_split_pmd = value,
            174 => values.thp_scan_exceed_none_pte = value,
            175 => values.thp_scan_exceed_swap_pte = value,
            176 => values.thp_scan_exceed_share_pte = value,
            177 => values.thp_split_pud = value,
            178 => values.thp_zero_page_alloc = value,
            179 => values.thp_zero_page_alloc_failed = value,
            180 => values.thp_swpout = value,
            181 => values.thp_swpout_fallback = value,
            182 => values.balloon_inflate = value,
            183 => values.balloon_deflate = value,
            184 => values.balloon_migrate = value,
            185 => values.swap_ra = value,
            186 => values.swap_ra_hit = value,
            187 => values.swpin_zero = value,
            188 => values.swpout_zero = value,
            189 => values.ksm_swpin_copy = value,
            190 => values.cow_ksm = value,
            191 => values.zswpin = value,
            192 => values.zswpout = value,
            193 => values.zswpwb = value,
            194 => values.direct_map_level2_splits = value,
            195 => values.direct_map_level3_splits = value,
            196 => values.direct_map_level2_collapses = value,
            197 => values.direct_map_level3_collapses = value,
            198 => values.nr_unstable = value,
            _ => {}
        }
    }

    fn parse_buffer(&self, values: &mut ProcVmStatFields, field_filter: &HashMap<usize, bool>) {
        if field_filter.is_empty() {
            return;
        }
        let mut field = 1usize;
        let mut line_started = false;
        let mut key_finished = false;
        let mut selected = field_filter.get(&field).copied().unwrap_or(false);
        let mut value_started = false;
        let mut value_finished = false;
        let mut value = 0u64;
        let mut digit_count = 0usize;
        let mut value_valid = true;

        for index in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[index];
            if matches!(byte, b'\n' | b'\0') {
                if line_started {
                    if selected && value_started {
                        let parsed_value = if value_valid && digit_count > 0 {
                            value
                        } else {
                            0
                        };

                        self.set_field(&mut *values, field, parsed_value);
                    }

                    field += 1;
                }

                line_started = false;
                key_finished = false;
                selected = field_filter.get(&field).copied().unwrap_or(false);

                value_started = false;
                value_finished = false;
                value = 0;
                digit_count = 0;
                value_valid = true;

                continue;
            }

            if !line_started {
                line_started = true;
            }

            if !key_finished {
                if byte.is_ascii_whitespace() {
                    key_finished = true;
                }

                continue;
            }

            // A disabled field only needs its newline to be detected.
            if !selected || value_finished {
                continue;
            }

            match byte {
                b'0'..=b'9' => {
                    value_started = true;
                    digit_count += 1;

                    // Preserve parse_u64_swar's 20-digit input limit.
                    if digit_count > 20 {
                        value_valid = false;
                        continue;
                    }

                    let digit = u64::from(byte - b'0');

                    value = match value
                        .checked_mul(10)
                        .and_then(|current| current.checked_add(digit))
                    {
                        Some(value) => value,

                        None => {
                            value_valid = false;
                            continue;
                        }
                    };
                }
                byte if byte.is_ascii_whitespace() => {
                    if value_started {
                        // The decimal value has ended. Ignore anything else
                        // on this line.
                        value_finished = true;
                    }
                }
                _ if value_started => {
                    // Match the previous parser's behavior: only the initial
                    // contiguous decimal value is parsed.
                    value_finished = true;
                }
                _ => {
                    // A non-whitespace, non-digit byte before the value makes
                    // this line malformed.
                    value_valid = false;
                    value_finished = true;
                }
            }
        }
        // Handle a final line without a trailing newline or NUL.
        if line_started && selected && value_started {
            let parsed_value = if value_valid && digit_count > 0 {
                value
            } else {
                0
            };

            self.set_field(&mut *values, field, parsed_value);
        }
    }
}
