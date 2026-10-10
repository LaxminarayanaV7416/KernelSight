use super::config::ProcFSConfig;
use super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

// its the disk stats config
// its from the /sys/block/<device>/stat
// its same as the /proc/diskstats

#[derive(Debug, Default, Deserialize)]
pub struct ProcDiskStatsConfigFields {
    pub reads_completed: bool,
    pub reads_merged: bool,
    pub sectors_read: bool,
    pub time_spent_reading: bool,
    pub writes_completed: bool,
    pub writes_merged: bool,
    pub sectors_written: bool,
    pub time_spent_writing: bool,
    pub ios_in_progress: bool,
    pub time_spent_doing_io: bool,
    pub weighted_time_spent_doing_io: bool,
    pub discards_completed: bool,
    pub discards_merged: bool,
    pub sectors_discarded: bool,
    pub time_spent_discarding: bool,
    pub flushes_completed: bool,
    pub time_spent_flushing: bool,
}

pub type ProcDiskStatsConfig = ProcFSConfig<ProcDiskStatsConfigFields>;

impl ProcFSConfigTrait for ProcDiskStatsConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.reads_completed),
            (2, self.fields.reads_merged),
            (3, self.fields.sectors_read),
            (4, self.fields.time_spent_reading),
            (5, self.fields.writes_completed),
            (6, self.fields.writes_merged),
            (7, self.fields.sectors_written),
            (8, self.fields.time_spent_writing),
            (9, self.fields.ios_in_progress),
            (10, self.fields.time_spent_doing_io),
            (11, self.fields.weighted_time_spent_doing_io),
            (12, self.fields.discards_completed),
            (13, self.fields.discards_merged),
            (14, self.fields.sectors_discarded),
            (15, self.fields.time_spent_discarding),
            (16, self.fields.flushes_completed),
            (17, self.fields.time_spent_flushing),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow
            & self.fields.reads_completed
            & self.fields.reads_merged
            & self.fields.sectors_read
            & self.fields.time_spent_reading
            & self.fields.writes_completed
            & self.fields.writes_merged
            & self.fields.sectors_written
            & self.fields.time_spent_writing
            & self.fields.ios_in_progress
            & self.fields.time_spent_doing_io
            & self.fields.weighted_time_spent_doing_io
            & self.fields.discards_completed
            & self.fields.discards_merged
            & self.fields.sectors_discarded
            & self.fields.time_spent_discarding
            & self.fields.flushes_completed
            & self.fields.time_spent_flushing
    }
}
