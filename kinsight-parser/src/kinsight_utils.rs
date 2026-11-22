use std::hash::{BuildHasherDefault, Hasher};

#[derive(Default)]
pub struct KinsightHasher {
    hash: u64,
}

impl Hasher for KinsightHasher {
    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.hash = i as u32 as u64;
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash = i as u64;
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.hash = i;
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.hash = i as u64;
    }

    fn write(&mut self, _bytes: &[u8]) {
        unreachable!("KinsightHasher only supports i32, u32, u64, i64 keys");
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

pub type KinsightHashBuild = BuildHasherDefault<KinsightHasher>;

pub const PROCFS_PIDS_PARSER_THREAD_NAME: &str = "procfs_pids_detection_scanner_thread";
pub const PROCFS_ROOT_PARSER_THREAD_NAME: &str = "procfs_root_parser_thread";
