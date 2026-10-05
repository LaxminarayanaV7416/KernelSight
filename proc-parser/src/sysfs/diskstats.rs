use crate::common::file_reader::ProcFileReader;
use crate::common::kernel_types::{UnsignedInt, UnsignedLong};
use crate::common::parser_utils::parse_u64_swar;
use crate::common::procfs_constants::SYS_FS_ROOT_PATH;
use crate::configs::sysfs_diskstats_config::DiskStatsConfig;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

/*
Both files contain the same 17 statistics. /sys/block/<device>/stat contains the fields for <device>.
In /proc/diskstats the fields are prefixed with the major and minor device numbers and the device name.

LAX: So we have difficulty in parsing the /proc/diskstats so we are parsing smaller content file easy-peezy

Documentation Link: https://docs.kernel.org/admin-guide/iostats.html

Field 1 -- # of reads completed (unsigned long)
    This is the total number of reads completed successfully.

Field 2 -- # of reads merged, field 6 -- # of writes merged (unsigned long)
    Reads and writes which are adjacent to each other may be merged for efficiency. Thus two 4K reads may become one 8K read before it is ultimately handed to the disk, and so it will be counted (and queued) as only one I/O. This field lets you know how often this was done.

Field 3 -- # of sectors read (unsigned long)
    This is the total number of sectors read successfully.

Field 4 -- # of milliseconds spent reading (unsigned int)
    This is the total number of milliseconds spent by all reads (as measured from blk_mq_alloc_request() to __blk_mq_end_request()).

Field 5 -- # of writes completed (unsigned long)
    This is the total number of writes completed successfully.

Field 6 -- # of writes merged (unsigned long)
    See the description of field 2.

Field 7 -- # of sectors written (unsigned long)
    This is the total number of sectors written successfully.

Field 8 -- # of milliseconds spent writing (unsigned int)
    This is the total number of milliseconds spent by all writes (as measured from blk_mq_alloc_request() to __blk_mq_end_request()).

Field 9 -- # of I/Os currently in progress (unsigned int)
    The only field that should go to zero. Incremented as requests are given to appropriate struct request_queue and decremented as they finish.

Field 10 -- # of milliseconds spent doing I/Os (unsigned int)
    This field increases so long as field 9 is nonzero.
    Since 5.0 this field counts jiffies when at least one request was started or completed. If request runs more than 2 jiffies then some I/O time might be not accounted in case of concurrent requests.

Field 11 -- weighted # of milliseconds spent doing I/Os (unsigned int)
    This field is incremented at each I/O start, I/O completion, I/O merge, or read of these stats by the number of I/Os in progress (field 9) times the number of milliseconds spent doing I/O since the last update of this field. This can provide an easy measure of both I/O completion time and the backlog that may be accumulating.

Field 12 -- # of discards completed (unsigned long)
    This is the total number of discards completed successfully.

Field 13 -- # of discards merged (unsigned long)
    See the description of field 2

Field 14 -- # of sectors discarded (unsigned long)
    This is the total number of sectors discarded successfully.

Field 15 -- # of milliseconds spent discarding (unsigned int)
    This is the total number of milliseconds spent by all discards (as measured from blk_mq_alloc_request() to __blk_mq_end_request()).

Field 16 -- # of flush requests completed
    This is the total number of flush requests completed successfully.

Block layer combines flush requests and executes at most one at a time. This counts flush requests executed by disk. Not tracked for partitions.

Field 17 -- # of milliseconds spent flushing
    This is the total number of milliseconds spent by all flush requests.


LAX Calculations for the memory
*/

pub struct DiskStatsReader {
    reader: ProcFileReader<512, ()>,
    values: DiskStatsValues,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DiskStatsValues {
    pub reads_completed: UnsignedLong,
    pub reads_merged: UnsignedLong,
    pub sectors_read: UnsignedLong,
    pub time_spent_reading: UnsignedInt,
    pub writes_completed: UnsignedLong,
    pub writes_merged: UnsignedLong,
    pub sectors_written: UnsignedLong,
    pub time_spent_writing: UnsignedInt,
    pub ios_in_progress: UnsignedInt,
    pub time_spent_doing_io: UnsignedInt,
    pub weighted_time_spent_doing_io: UnsignedInt,
    pub discards_completed: UnsignedLong,
    pub discards_merged: UnsignedLong,
    pub sectors_discarded: UnsignedLong,
    pub time_spent_discarding: UnsignedInt,
    pub flushes_completed: UnsignedLong,
    pub time_spent_flushing: UnsignedInt,
}

impl DiskStatsReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: DiskStatsValues::default(),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        let mut field = 0usize;
        let mut start: Option<usize> = None;

        for i in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[i];

            if byte == b' ' || byte == b'\n' || byte == b'\0' {
                if let Some(s) = start.take() {
                    field += 1;
                    if field_filter.get(&field).copied().unwrap_or(false) {
                        self.set_field(field, s, i);
                    }
                }
            } else if start.is_none() {
                start = Some(i);
            }
        }

        if let Some(s) = start {
            field += 1;
            if field_filter.get(&field).copied().unwrap_or(false) {
                self.set_field(field, s, self.reader.buffer_len);
            }
        }
    }

    fn set_field(&mut self, field: usize, start: usize, end: usize) {
        let bytes = &self.reader.buffer[start..end];
        let value = parse_u64_swar(bytes).unwrap_or(0);

        match field {
            1 => self.values.reads_completed = value,
            2 => self.values.reads_merged = value,
            3 => self.values.sectors_read = value,
            4 => self.values.time_spent_reading = value as UnsignedInt,
            5 => self.values.writes_completed = value,
            6 => self.values.writes_merged = value,
            7 => self.values.sectors_written = value,
            8 => self.values.time_spent_writing = value as UnsignedInt,
            9 => self.values.ios_in_progress = value as UnsignedInt,
            10 => self.values.time_spent_doing_io = value as UnsignedInt,
            11 => self.values.weighted_time_spent_doing_io = value as UnsignedInt,
            12 => self.values.discards_completed = value,
            13 => self.values.discards_merged = value,
            14 => self.values.sectors_discarded = value,
            15 => self.values.time_spent_discarding = value as UnsignedInt,
            16 => self.values.flushes_completed = value,
            17 => self.values.time_spent_flushing = value as UnsignedInt,
            _ => {}
        }
    }

    pub fn values(&self) -> &DiskStatsValues {
        &self.values
    }
}

// this function parses the diskstats files from each block device including zram
// its thread so it runs in parallel with other parsers
pub fn parse_diskstats(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
) -> JoinHandle<()> {
    let sys_block_path = Path::new(SYS_FS_ROOT_PATH).join("block");
    let mut diskstats_paths = Vec::new();
    for entry in read_dir(&sys_block_path).into_iter().flatten().flatten() {
        let device_name = entry.file_name().to_string_lossy().into_owned();
        let block_device_path = sys_block_path.join(&device_name).join("stat");
        diskstats_paths.push(block_device_path);
    }
    let mut diskstats = Vec::new();
    for path in diskstats_paths {
        let diskstat = DiskStatsReader::new(&path.to_str().unwrap(), is_cachable, is_root);
        match diskstat {
            Ok(diskstat) => diskstats.push(diskstat),
            Err(_) => {
                println!(
                    "Failed to parse diskstats for device: {}",
                    path.to_string_lossy()
                );
            }
        }
    }
    // TODO: Hook the config path here
    let filter_map = HashMap::from([
        (1, true),
        (2, true),
        (3, true),
        (4, true),
        (5, true),
        (6, true),
        (7, true),
        (8, true),
        (9, true),
        (10, true),
        (11, true),
        (12, true),
        (13, true),
        (14, true),
        (15, true),
        (16, true),
        (17, true),
    ]);
    let threads_info = thread::spawn(move || {
        while !signal_hook.load(Ordering::Relaxed) {
            for diskstat in diskstats.iter_mut() {
                if signal_hook.load(Ordering::Relaxed) {
                    break;
                }
                let read_status = diskstat.read_and_parse(&filter_map);
                if !read_status {
                    println!("Failed to read diskstats, ProcFS File closed!");
                    break;
                }
                println!("Parsed: {:?}", diskstat.values());
                println!("=====================================");
            }
            thread::sleep(std::time::Duration::from_millis(monitoring_heartbeat));
        }
        println!("diskstats thread exiting");
    });
    threads_info
}
