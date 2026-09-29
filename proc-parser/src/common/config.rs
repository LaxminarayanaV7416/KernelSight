use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    heartbeat: u64,
}

#[derive(Debug, Default, Deserialize)]
pub struct DiskStatsConfig {
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

impl DiskStatsConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.reads_completed),
            (2, self.reads_merged),
            (3, self.sectors_read),
            (4, self.time_spent_reading),
            (5, self.writes_completed),
            (6, self.writes_merged),
            (7, self.sectors_written),
            (8, self.time_spent_writing),
            (9, self.ios_in_progress),
            (10, self.time_spent_doing_io),
            (11, self.weighted_time_spent_doing_io),
            (12, self.discards_completed),
            (13, self.discards_merged),
            (14, self.sectors_discarded),
            (15, self.time_spent_discarding),
            (16, self.flushes_completed),
            (17, self.time_spent_flushing),
        ])
    }
}
