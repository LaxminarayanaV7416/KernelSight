use super::config::ProcFSConfig;
use super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize)]
pub struct ProcMemInfoConfigFields {
    #[serde(rename = "MemTotal")]
    pub mem_total: bool,
    #[serde(rename = "MemFree")]
    pub mem_free: bool,
    #[serde(rename = "MemAvailable")]
    pub mem_available: bool,
    #[serde(rename = "Buffers")]
    pub mem_buffers: bool,
    #[serde(rename = "Cached")]
    pub mem_cached: bool,
    #[serde(rename = "SwapCached")]
    pub mem_swap_cached: bool,
    #[serde(rename = "Active")]
    pub mem_active: bool,
    #[serde(rename = "Inactive")]
    pub mem_inactive: bool,
    #[serde(rename = "Active_anon")]
    pub mem_active_anon: bool,
    #[serde(rename = "Inactive_anon")]
    pub mem_inactive_anon: bool,
    #[serde(rename = "Active_file")]
    pub mem_active_file: bool,
    #[serde(rename = "Inactive_file")]
    pub mem_inactive_file: bool,
    #[serde(rename = "Unevictable")]
    pub mem_unevictable: bool,
    #[serde(rename = "Mlocked")]
    pub mem_mlocked: bool,
    #[serde(rename = "SwapTotal")]
    pub mem_swap_total: bool,
    #[serde(rename = "SwapFree")]
    pub mem_swap_free: bool,
    #[serde(rename = "Zswap")]
    pub mem_zswap: bool,
    #[serde(rename = "Zswapped")]
    pub mem_zswapped: bool,
    #[serde(rename = "Dirty")]
    pub mem_dirty: bool,
    #[serde(rename = "Writeback")]
    pub mem_write_back: bool,
    #[serde(rename = "AnonPages")]
    pub mem_anon_pages: bool,
    #[serde(rename = "Mapped")]
    pub mem_mapped: bool,
    #[serde(rename = "Shmem")]
    pub mem_shmem: bool,
    #[serde(rename = "KReclaimable")]
    pub mem_k_reclaimable: bool,
    #[serde(rename = "Slab")]
    pub mem_slab: bool,
    #[serde(rename = "SReclaimable")]
    pub mem_s_reclaimable: bool,
    #[serde(rename = "SUnreclaim")]
    pub mem_s_unreclaim: bool,
    #[serde(rename = "KernelStack")]
    pub mem_kernel_stack: bool,
    #[serde(rename = "PageTables")]
    pub mem_page_tables: bool,
    #[serde(rename = "SecPageTables")]
    pub mem_sec_page_tables: bool,
    #[serde(rename = "NFS_Unstable")]
    pub mem_nfs_unstable: bool,
    #[serde(rename = "Bounce")]
    pub mem_bounce: bool,
    #[serde(rename = "WritebackTmp")]
    pub mem_write_back_tmp: bool,
    #[serde(rename = "CommitLimit")]
    pub mem_commit_limit: bool,
    #[serde(rename = "Committed_AS")]
    pub mem_committed_as: bool,
    #[serde(rename = "VmallocTotal")]
    pub mem_vmalloc_total: bool,
    #[serde(rename = "VmallocUsed")]
    pub mem_vmalloc_used: bool,
    #[serde(rename = "VmallocChunk")]
    pub mem_vmalloc_chunk: bool,
    #[serde(rename = "Percpu")]
    pub mem_percpu: bool,
    #[serde(rename = "HardwareCorrupted")]
    pub mem_hardware_corrupted: bool,
    #[serde(rename = "AnonHugePages")]
    pub mem_anon_huge_pages: bool,
    #[serde(rename = "ShmemHugePages")]
    pub mem_shmem_huge_pages: bool,
    #[serde(rename = "ShmemPmdMapped")]
    pub mem_shmem_pmd_mapped: bool,
    #[serde(rename = "FileHugePages")]
    pub mem_file_huge_pages: bool,
    #[serde(rename = "FilePmdMapped")]
    pub mem_file_pmd_mapped: bool,
    #[serde(rename = "CmaTotal")]
    pub mem_cma_total: bool,
    #[serde(rename = "CmaFree")]
    pub mem_cma_free: bool,
    #[serde(rename = "Unaccepted")]
    pub mem_unaccepted: bool,
    #[serde(rename = "Balloon")]
    pub mem_balloon: bool,
    #[serde(rename = "GPUActive")]
    pub mem_gpu_active: bool,
    #[serde(rename = "GPUReclaim")]
    pub mem_gpu_reclaim: bool,
    #[serde(rename = "HugePages_Total")]
    pub mem_huge_pages_total: bool,
    #[serde(rename = "HugePages_Free")]
    pub mem_huge_pages_free: bool,
    #[serde(rename = "HugePages_Rsvd")]
    pub mem_huge_pages_reserved: bool,
    #[serde(rename = "HugePages_Surp")]
    pub mem_huge_pages_surp: bool,
    #[serde(rename = "Hugepagesize")]
    pub mem_huge_page_size: bool,
    #[serde(rename = "Hugetlb")]
    pub mem_hugetlb: bool,
    #[serde(rename = "DirectMap4k")]
    pub mem_direct_map_4k: bool,
    #[serde(rename = "DirectMap2M")]
    pub mem_direct_map_2m: bool,
    #[serde(rename = "DirectMap1G")]
    pub mem_direct_map_1g: bool,
}

pub type ProcMemInfoConfig = ProcFSConfig<ProcMemInfoConfigFields>;

impl ProcFSConfigTrait for ProcMemInfoConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.mem_total),
            (2, self.fields.mem_free),
            (3, self.fields.mem_available),
            (4, self.fields.mem_buffers),
            (5, self.fields.mem_cached),
            (6, self.fields.mem_swap_cached),
            (7, self.fields.mem_active),
            (8, self.fields.mem_inactive),
            (9, self.fields.mem_active_anon),
            (10, self.fields.mem_inactive_anon),
            (11, self.fields.mem_active_file),
            (12, self.fields.mem_inactive_file),
            (13, self.fields.mem_unevictable),
            (14, self.fields.mem_mlocked),
            (15, self.fields.mem_swap_total),
            (16, self.fields.mem_swap_free),
            (17, self.fields.mem_zswap),
            (18, self.fields.mem_zswapped),
            (19, self.fields.mem_dirty),
            (20, self.fields.mem_write_back),
            (21, self.fields.mem_anon_pages),
            (22, self.fields.mem_mapped),
            (23, self.fields.mem_shmem),
            (24, self.fields.mem_k_reclaimable),
            (25, self.fields.mem_slab),
            (26, self.fields.mem_s_reclaimable),
            (27, self.fields.mem_s_unreclaim),
            (28, self.fields.mem_kernel_stack),
            (29, self.fields.mem_page_tables),
            (30, self.fields.mem_sec_page_tables),
            (31, self.fields.mem_nfs_unstable),
            (32, self.fields.mem_bounce),
            (33, self.fields.mem_write_back_tmp),
            (34, self.fields.mem_commit_limit),
            (35, self.fields.mem_committed_as),
            (36, self.fields.mem_vmalloc_total),
            (37, self.fields.mem_vmalloc_used),
            (38, self.fields.mem_vmalloc_chunk),
            (39, self.fields.mem_percpu),
            (40, self.fields.mem_hardware_corrupted),
            (41, self.fields.mem_anon_huge_pages),
            (42, self.fields.mem_shmem_huge_pages),
            (43, self.fields.mem_shmem_pmd_mapped),
            (44, self.fields.mem_file_huge_pages),
            (45, self.fields.mem_file_pmd_mapped),
            (46, self.fields.mem_cma_total),
            (47, self.fields.mem_cma_free),
            (48, self.fields.mem_unaccepted),
            (49, self.fields.mem_balloon),
            (50, self.fields.mem_gpu_active),
            (51, self.fields.mem_gpu_reclaim),
            (52, self.fields.mem_huge_pages_total),
            (53, self.fields.mem_huge_pages_free),
            (54, self.fields.mem_huge_pages_reserved),
            (55, self.fields.mem_huge_pages_surp),
            (56, self.fields.mem_huge_page_size),
            (57, self.fields.mem_hugetlb),
            (58, self.fields.mem_direct_map_4k),
            (59, self.fields.mem_direct_map_2m),
            (60, self.fields.mem_direct_map_1g),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::from([
            ("MemTotal", (1, self.fields.mem_active)),
            ("MemFree", (2, self.fields.mem_free)),
            ("MemAvailable", (3, self.fields.mem_available)),
            ("Buffers", (4, self.fields.mem_buffers)),
            ("Cached", (5, self.fields.mem_cached)),
            ("SwapCached", (6, self.fields.mem_swap_cached)),
            ("Active", (7, self.fields.mem_active)),
            ("Inactive", (8, self.fields.mem_inactive)),
            ("Active(anon)", (9, self.fields.mem_active_anon)),
            ("Inactive(anon)", (10, self.fields.mem_inactive_anon)),
            ("Active(file)", (11, self.fields.mem_active_file)),
            ("Inactive(file)", (12, self.fields.mem_inactive_file)),
            ("Unevictable", (13, self.fields.mem_unevictable)),
            ("Mlocked", (14, self.fields.mem_mlocked)),
            ("SwapTotal", (15, self.fields.mem_swap_total)),
            ("SwapFree", (16, self.fields.mem_swap_free)),
            ("Zswap", (17, self.fields.mem_zswap)),
            ("Zswapped", (18, self.fields.mem_zswapped)),
            ("Dirty", (19, self.fields.mem_dirty)),
            ("Writeback", (20, self.fields.mem_write_back)),
            ("AnonPages", (21, self.fields.mem_anon_pages)),
            ("Mapped", (22, self.fields.mem_mapped)),
            ("Shmem", (23, self.fields.mem_shmem)),
            ("KReclaimable", (24, self.fields.mem_k_reclaimable)),
            ("Slab", (25, self.fields.mem_slab)),
            ("SReclaimable", (26, self.fields.mem_s_reclaimable)),
            ("SUnreclaim", (27, self.fields.mem_s_unreclaim)),
            ("KernelStack", (28, self.fields.mem_kernel_stack)),
            ("PageTables", (29, self.fields.mem_page_tables)),
            ("SecPageTables", (30, self.fields.mem_sec_page_tables)),
            ("NFS_Unstable", (31, self.fields.mem_nfs_unstable)),
            ("Bounce", (32, self.fields.mem_bounce)),
            ("WritebackTmp", (33, self.fields.mem_write_back_tmp)),
            ("CommitLimit", (34, self.fields.mem_commit_limit)),
            ("Committed_AS", (35, self.fields.mem_committed_as)),
            ("VmallocTotal", (36, self.fields.mem_vmalloc_total)),
            ("VmallocUsed", (37, self.fields.mem_vmalloc_used)),
            ("VmallocChunk", (38, self.fields.mem_vmalloc_chunk)),
            ("Percpu", (39, self.fields.mem_percpu)),
            (
                "HardwareCorrupted",
                (40, self.fields.mem_hardware_corrupted),
            ),
            ("AnonHugePages", (41, self.fields.mem_anon_huge_pages)),
            ("ShmemHugePages", (42, self.fields.mem_shmem_huge_pages)),
            ("ShmemPmdMapped", (43, self.fields.mem_shmem_pmd_mapped)),
            ("FileHugePages", (44, self.fields.mem_file_huge_pages)),
            ("FilePmdMapped", (45, self.fields.mem_file_pmd_mapped)),
            ("CmaTotal", (46, self.fields.mem_cma_total)),
            ("CmaFree", (47, self.fields.mem_cma_free)),
            ("Unaccepted", (48, self.fields.mem_unaccepted)),
            ("Balloon", (49, self.fields.mem_balloon)),
            ("GPUActive", (50, self.fields.mem_gpu_active)),
            ("GPUReclaim", (51, self.fields.mem_gpu_reclaim)),
            ("HugePages_Total", (52, self.fields.mem_huge_pages_total)),
            ("HugePages_Free", (53, self.fields.mem_huge_pages_free)),
            ("HugePages_Rsvd", (54, self.fields.mem_huge_pages_reserved)),
            ("HugePages_Surp", (55, self.fields.mem_huge_pages_surp)),
            ("Hugepagesize", (56, self.fields.mem_huge_page_size)),
            ("Hugetlb", (57, self.fields.mem_hugetlb)),
            ("DirectMap4k", (58, self.fields.mem_direct_map_4k)),
            ("DirectMap2M", (59, self.fields.mem_direct_map_2m)),
            ("DirectMap1G", (60, self.fields.mem_direct_map_1g)),
        ])
    }

    fn is_enabled(&self) -> bool {
        self.allow
            & self.fields.mem_total
            & self.fields.mem_free
            & self.fields.mem_available
            & self.fields.mem_buffers
            & self.fields.mem_cached
            & self.fields.mem_swap_cached
            & self.fields.mem_active
            & self.fields.mem_inactive
            & self.fields.mem_active_anon
            & self.fields.mem_inactive_anon
            & self.fields.mem_active_file
            & self.fields.mem_inactive_file
            & self.fields.mem_unevictable
            & self.fields.mem_mlocked
            & self.fields.mem_swap_total
            & self.fields.mem_swap_free
            & self.fields.mem_zswap
            & self.fields.mem_zswapped
            & self.fields.mem_dirty
            & self.fields.mem_write_back
            & self.fields.mem_anon_pages
            & self.fields.mem_mapped
            & self.fields.mem_shmem
            & self.fields.mem_k_reclaimable
            & self.fields.mem_slab
            & self.fields.mem_s_reclaimable
            & self.fields.mem_s_unreclaim
            & self.fields.mem_kernel_stack
            & self.fields.mem_page_tables
            & self.fields.mem_sec_page_tables
            & self.fields.mem_nfs_unstable
            & self.fields.mem_bounce
            & self.fields.mem_write_back_tmp
            & self.fields.mem_commit_limit
            & self.fields.mem_committed_as
            & self.fields.mem_vmalloc_total
            & self.fields.mem_vmalloc_used
            & self.fields.mem_vmalloc_chunk
            & self.fields.mem_percpu
            & self.fields.mem_hardware_corrupted
            & self.fields.mem_anon_huge_pages
            & self.fields.mem_shmem_huge_pages
            & self.fields.mem_shmem_pmd_mapped
            & self.fields.mem_file_huge_pages
            & self.fields.mem_file_pmd_mapped
            & self.fields.mem_cma_total
            & self.fields.mem_cma_free
            & self.fields.mem_unaccepted
            & self.fields.mem_balloon
            & self.fields.mem_gpu_active
            & self.fields.mem_gpu_reclaim
            & self.fields.mem_huge_pages_total
            & self.fields.mem_huge_pages_free
            & self.fields.mem_huge_pages_reserved
            & self.fields.mem_huge_pages_surp
            & self.fields.mem_huge_page_size
            & self.fields.mem_hugetlb
            & self.fields.mem_direct_map_4k
            & self.fields.mem_direct_map_2m
            & self.fields.mem_direct_map_1g
    }
}
