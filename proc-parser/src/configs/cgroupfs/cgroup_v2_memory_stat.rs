use super::super::config::ProcFSConfig;
use super::super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize)]
pub struct CgroupV2MemoryStatConfigFields {
    pub anon: bool,
    pub file: bool,
    pub kernel: bool,
    pub kernel_stack: bool,
    pub pagetables: bool,
    pub sec_pagetables: bool,
    pub percpu: bool,
    pub sock: bool,
    pub vmalloc: bool,
    pub shmem: bool,
    pub zswap: bool,
    pub zswapped: bool,
    pub zswap_incomp: bool,
    pub file_mapped: bool,
    pub file_dirty: bool,
    pub file_writeback: bool,
    pub swapcached: bool,
    pub anon_thp: bool,
    pub file_thp: bool,
    pub shmem_thp: bool,
    pub inactive_anon: bool,
    pub active_anon: bool,
    pub inactive_file: bool,
    pub active_file: bool,
    pub unevictable: bool,
    pub slab_reclaimable: bool,
    pub slab_unreclaimable: bool,
    pub slab: bool,
    pub hugetlb: bool,
    pub workingset_refault_anon: bool,
    pub workingset_refault_file: bool,
    pub workingset_activate_anon: bool,
    pub workingset_activate_file: bool,
    pub workingset_restore_anon: bool,
    pub workingset_restore_file: bool,
    pub workingset_nodereclaim: bool,
    pub pgdemote_kswapd: bool,
    pub pgdemote_direct: bool,
    pub pgdemote_khugepaged: bool,
    pub pgdemote_proactive: bool,
    pub pgsteal_kswapd: bool,
    pub pgsteal_direct: bool,
    pub pgsteal_khugepaged: bool,
    pub pgsteal_proactive: bool,
    pub pgscan_kswapd: bool,
    pub pgscan_direct: bool,
    pub pgscan_khugepaged: bool,
    pub pgscan_proactive: bool,
    pub pgrefill: bool,
    pub pgpromote_success: bool,
    pub pgscan: bool,
    pub pgsteal: bool,
    pub pswpin: bool,
    pub pswpout: bool,
    pub pgfault: bool,
    pub pgmajfault: bool,
    pub pgactivate: bool,
    pub pgdeactivate: bool,
    pub pglazyfree: bool,
    pub pglazyfreed: bool,
    pub swpin_zero: bool,
    pub swpout_zero: bool,
    pub zswpin: bool,
    pub zswpout: bool,
    pub zswpwb: bool,
    pub thp_fault_alloc: bool,
    pub thp_collapse_alloc: bool,
    pub thp_swpout: bool,
    pub thp_swpout_fallback: bool,
    pub numa_pages_migrated: bool,
    pub numa_pte_updates: bool,
    pub numa_hint_faults: bool,
}

pub type CgroupV2MemoryStatConfig = ProcFSConfig<CgroupV2MemoryStatConfigFields>;

impl ProcFSConfigTrait for CgroupV2MemoryStatConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.anon),
            (2, self.fields.file),
            (3, self.fields.kernel),
            (4, self.fields.kernel_stack),
            (5, self.fields.pagetables),
            (6, self.fields.sec_pagetables),
            (7, self.fields.percpu),
            (8, self.fields.sock),
            (9, self.fields.vmalloc),
            (10, self.fields.shmem),
            (11, self.fields.zswap),
            (12, self.fields.zswapped),
            (13, self.fields.zswap_incomp),
            (14, self.fields.file_mapped),
            (15, self.fields.file_dirty),
            (16, self.fields.file_writeback),
            (17, self.fields.swapcached),
            (18, self.fields.anon_thp),
            (19, self.fields.file_thp),
            (20, self.fields.shmem_thp),
            (21, self.fields.inactive_anon),
            (22, self.fields.active_anon),
            (23, self.fields.inactive_file),
            (24, self.fields.active_file),
            (25, self.fields.unevictable),
            (26, self.fields.slab_reclaimable),
            (27, self.fields.slab_unreclaimable),
            (28, self.fields.slab),
            (29, self.fields.hugetlb),
            (30, self.fields.workingset_refault_anon),
            (31, self.fields.workingset_refault_file),
            (32, self.fields.workingset_activate_anon),
            (33, self.fields.workingset_activate_file),
            (34, self.fields.workingset_restore_anon),
            (35, self.fields.workingset_restore_file),
            (36, self.fields.workingset_nodereclaim),
            (37, self.fields.pgdemote_kswapd),
            (38, self.fields.pgdemote_direct),
            (39, self.fields.pgdemote_khugepaged),
            (40, self.fields.pgdemote_proactive),
            (41, self.fields.pgsteal_kswapd),
            (42, self.fields.pgsteal_direct),
            (43, self.fields.pgsteal_khugepaged),
            (44, self.fields.pgsteal_proactive),
            (45, self.fields.pgscan_kswapd),
            (46, self.fields.pgscan_direct),
            (47, self.fields.pgscan_khugepaged),
            (48, self.fields.pgscan_proactive),
            (49, self.fields.pgrefill),
            (50, self.fields.pgpromote_success),
            (51, self.fields.pgscan),
            (52, self.fields.pgsteal),
            (53, self.fields.pswpin),
            (54, self.fields.pswpout),
            (55, self.fields.pgfault),
            (56, self.fields.pgmajfault),
            (57, self.fields.pgactivate),
            (58, self.fields.pgdeactivate),
            (59, self.fields.pglazyfree),
            (60, self.fields.pglazyfreed),
            (61, self.fields.swpin_zero),
            (62, self.fields.swpout_zero),
            (63, self.fields.zswpin),
            (64, self.fields.zswpout),
            (65, self.fields.zswpwb),
            (66, self.fields.thp_fault_alloc),
            (67, self.fields.thp_collapse_alloc),
            (68, self.fields.thp_swpout),
            (69, self.fields.thp_swpout_fallback),
            (70, self.fields.numa_pages_migrated),
            (71, self.fields.numa_pte_updates),
            (72, self.fields.numa_hint_faults),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow
            & self.fields.anon
            & self.fields.file
            & self.fields.kernel
            & self.fields.kernel_stack
            & self.fields.pagetables
            & self.fields.sec_pagetables
            & self.fields.percpu
            & self.fields.sock
            & self.fields.vmalloc
            & self.fields.shmem
            & self.fields.zswap
            & self.fields.zswapped
            & self.fields.zswap_incomp
            & self.fields.file_mapped
            & self.fields.file_dirty
            & self.fields.file_writeback
            & self.fields.swapcached
            & self.fields.anon_thp
            & self.fields.file_thp
            & self.fields.shmem_thp
            & self.fields.inactive_anon
            & self.fields.active_anon
            & self.fields.inactive_file
            & self.fields.active_file
            & self.fields.unevictable
            & self.fields.slab_reclaimable
            & self.fields.slab_unreclaimable
            & self.fields.slab
            & self.fields.hugetlb
            & self.fields.workingset_refault_anon
            & self.fields.workingset_refault_file
            & self.fields.workingset_activate_anon
            & self.fields.workingset_activate_file
            & self.fields.workingset_restore_anon
            & self.fields.workingset_restore_file
            & self.fields.workingset_nodereclaim
            & self.fields.pgdemote_kswapd
            & self.fields.pgdemote_direct
            & self.fields.pgdemote_khugepaged
            & self.fields.pgdemote_proactive
            & self.fields.pgsteal_kswapd
            & self.fields.pgsteal_direct
            & self.fields.pgsteal_khugepaged
            & self.fields.pgsteal_proactive
            & self.fields.pgscan_kswapd
            & self.fields.pgscan_direct
            & self.fields.pgscan_khugepaged
            & self.fields.pgscan_proactive
            & self.fields.pgrefill
            & self.fields.pgpromote_success
            & self.fields.pgscan
            & self.fields.pgsteal
            & self.fields.pswpin
            & self.fields.pswpout
            & self.fields.pgfault
            & self.fields.pgmajfault
            & self.fields.pgactivate
            & self.fields.pgdeactivate
            & self.fields.pglazyfree
            & self.fields.pglazyfreed
            & self.fields.swpin_zero
            & self.fields.swpout_zero
            & self.fields.zswpin
            & self.fields.zswpout
            & self.fields.zswpwb
            & self.fields.thp_fault_alloc
            & self.fields.thp_collapse_alloc
            & self.fields.thp_swpout
            & self.fields.thp_swpout_fallback
            & self.fields.numa_pages_migrated
            & self.fields.numa_pte_updates
            & self.fields.numa_hint_faults
    }
}
