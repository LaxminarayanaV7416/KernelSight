use serde::Deserialize;
use std::collections::HashMap;

// its the PID smaps_rollup config
// its from the /proc/<PID>/smaps_rollup

#[derive(Debug, Default, Deserialize)]
pub struct ProcPidfsSmapsRollupConfig {
    pub rss: bool,
    pub pss: bool,
    pub pss_dirty: bool,
    pub pss_anon: bool,
    pub pss_file: bool,
    pub pss_shmem: bool,
    pub shared_clean: bool,
    pub shared_dirty: bool,
    pub private_clean: bool,
    pub private_dirty: bool,
    pub referenced: bool,
    pub anonymous: bool,
    pub ksm: bool,
    pub lazy_free: bool,
    pub anon_huge_pages: bool,
    pub shmem_pmd_mapped: bool,
    pub file_pmd_mapped: bool,
    pub shared_hugetlb: bool,
    pub private_hugetlb: bool,
    pub swap: bool,
    pub swap_pss: bool,
    pub locked: bool,
}

impl ProcPidfsSmapsRollupConfig {
    pub fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.rss),
            (2, self.pss),
            (3, self.pss_dirty),
            (4, self.pss_anon),
            (5, self.pss_file),
            (6, self.pss_shmem),
            (7, self.shared_clean),
            (8, self.shared_dirty),
            (9, self.private_clean),
            (10, self.private_dirty),
            (11, self.referenced),
            (12, self.anonymous),
            (13, self.ksm),
            (14, self.lazy_free),
            (15, self.anon_huge_pages),
            (16, self.shmem_pmd_mapped),
            (17, self.file_pmd_mapped),
            (18, self.shared_hugetlb),
            (19, self.private_hugetlb),
            (20, self.swap),
            (21, self.swap_pss),
            (22, self.locked),
        ])
    }
}
