use std::thread;

use crate::common::file_reader::ProcFileReader;
use crate::common::kernel_types::{UNSIGNED_INT, UNSIGNED_LONG};
use crate::common::parser_utils::{CharacterType, parse_u64_swar};
use crate::common::procfs_constants::SYS_FS_ROOT_PATH;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::Path;

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

#[derive(Debug)]
struct DiskStatsFields<'a> {
    pub ReadsCompleted: (u8, &'a [u8], UNSIGNED_LONG),
    pub ReadsMerged: (u8, &'a [u8], UNSIGNED_LONG),
    pub SectorsRead: (u8, &'a [u8], UNSIGNED_LONG),
    pub TimeSpentReading: (u8, &'a [u8], UNSIGNED_INT),
    pub WritesCompleted: (u8, &'a [u8], UNSIGNED_LONG),
    pub WritesMerged: (u8, &'a [u8], UNSIGNED_LONG),
    pub SectorsWritten: (u8, &'a [u8], UNSIGNED_LONG),
    pub TimeSpentWriting: (u8, &'a [u8], UNSIGNED_INT),
    pub IOsInProgress: (u8, &'a [u8], UNSIGNED_INT),
    pub TimeSpentDoingIO: (u8, &'a [u8], UNSIGNED_INT),
    pub WeightedNumberofSecondsSpentDoingIO: (u8, &'a [u8], UNSIGNED_INT),
    pub DiscardsCompleted: (u8, &'a [u8], UNSIGNED_LONG),
    pub DiscardsMerged: (u8, &'a [u8], UNSIGNED_LONG),
    pub SectorsDiscarded: (u8, &'a [u8], UNSIGNED_LONG),
    pub TimeSpentDiscarding: (u8, &'a [u8], UNSIGNED_INT),
    pub FlushesCompleted: (u8, &'a [u8], UNSIGNED_LONG),
    pub TimeSpentFlushing: (u8, &'a [u8], UNSIGNED_INT),
}

impl<'a> DiskStatsFields<'a> {
    pub fn new(updates: Vec<(usize, usize, usize)>, buffer: &'a [u8]) -> Self {
        // Start with empty slices.
        let empty: &[u8] = &buffer[0..0];
        let mut fields = Self {
            ReadsCompleted: (1, empty, 0),
            ReadsMerged: (2, empty, 0),
            SectorsRead: (3, empty, 0),
            TimeSpentReading: (4, empty, 0),
            WritesCompleted: (5, empty, 0),
            WritesMerged: (6, empty, 0),
            SectorsWritten: (7, empty, 0),
            TimeSpentWriting: (8, empty, 0),
            IOsInProgress: (9, empty, 0),
            TimeSpentDoingIO: (10, empty, 0),
            WeightedNumberofSecondsSpentDoingIO: (11, empty, 0),
            DiscardsCompleted: (12, empty, 0),
            DiscardsMerged: (13, empty, 0),
            SectorsDiscarded: (14, empty, 0),
            TimeSpentDiscarding: (15, empty, 0),
            FlushesCompleted: (16, empty, 0),
            TimeSpentFlushing: (17, empty, 0),
        };

        for &(field_id, start, end) in updates.iter() {
            let value = &buffer[start..end];

            match field_id {
                1 => fields.ReadsCompleted.1 = value,
                2 => fields.ReadsMerged.1 = value,
                3 => fields.SectorsRead.1 = value,
                4 => fields.TimeSpentReading.1 = value,
                5 => fields.WritesCompleted.1 = value,
                6 => fields.WritesMerged.1 = value,
                7 => fields.SectorsWritten.1 = value,
                8 => fields.TimeSpentWriting.1 = value,
                9 => fields.IOsInProgress.1 = value,
                10 => fields.TimeSpentDoingIO.1 = value,
                11 => fields.WeightedNumberofSecondsSpentDoingIO.1 = value,
                12 => fields.DiscardsCompleted.1 = value,
                13 => fields.DiscardsMerged.1 = value,
                14 => fields.SectorsDiscarded.1 = value,
                15 => fields.TimeSpentDiscarding.1 = value,
                16 => fields.FlushesCompleted.1 = value,
                17 => fields.TimeSpentFlushing.1 = value,
                // TODO handled the panic later
                _ => panic!("Invalid DiskStats field id: {}", field_id),
            }
        }

        fields
    }

    pub fn parse_as_human_readable(&mut self, field_ids: &[u8]) {
        for &field_id in field_ids {
            match field_id {
                1 => self.ReadsCompleted.2 = parse_u64_swar(self.ReadsCompleted.1).unwrap(),
                2 => self.ReadsMerged.2 = parse_u64_swar(self.ReadsMerged.1).unwrap(),
                3 => self.SectorsRead.2 = parse_u64_swar(self.SectorsRead.1).unwrap(),
                4 => {
                    self.TimeSpentReading.2 =
                        parse_u64_swar(self.TimeSpentReading.1).unwrap() as UNSIGNED_INT
                }
                5 => self.WritesCompleted.2 = parse_u64_swar(self.WritesCompleted.1).unwrap(),
                6 => self.WritesMerged.2 = parse_u64_swar(self.WritesMerged.1).unwrap(),
                7 => self.SectorsWritten.2 = parse_u64_swar(self.SectorsWritten.1).unwrap(),
                8 => {
                    self.TimeSpentWriting.2 =
                        parse_u64_swar(self.TimeSpentWriting.1).unwrap() as UNSIGNED_INT
                }
                9 => {
                    self.IOsInProgress.2 =
                        parse_u64_swar(self.IOsInProgress.1).unwrap() as UNSIGNED_INT
                }
                10 => {
                    self.TimeSpentDoingIO.2 =
                        parse_u64_swar(self.TimeSpentDoingIO.1).unwrap() as UNSIGNED_INT
                }
                11 => {
                    self.WeightedNumberofSecondsSpentDoingIO.2 =
                        parse_u64_swar(self.WeightedNumberofSecondsSpentDoingIO.1).unwrap()
                            as UNSIGNED_INT
                }
                12 => self.DiscardsCompleted.2 = parse_u64_swar(self.DiscardsCompleted.1).unwrap(),
                13 => self.DiscardsMerged.2 = parse_u64_swar(self.DiscardsMerged.1).unwrap(),
                14 => self.SectorsDiscarded.2 = parse_u64_swar(self.SectorsDiscarded.1).unwrap(),
                15 => {
                    self.TimeSpentDiscarding.2 =
                        parse_u64_swar(self.TimeSpentDiscarding.1).unwrap() as UNSIGNED_INT
                }
                16 => self.FlushesCompleted.2 = parse_u64_swar(self.FlushesCompleted.1).unwrap(),
                17 => {
                    self.TimeSpentFlushing.2 =
                        parse_u64_swar(self.TimeSpentFlushing.1).unwrap() as UNSIGNED_INT
                }
                // TODO handled the panic later
                _ => panic!("Invalid DiskStats field id: {}", field_id),
            }
        }
    }
}

pub type ProcFileDiskStats = ProcFileReader<512, DiskStatsFields<'static>>;

impl ProcFileDiskStats {
    pub fn parse(
        &mut self,
        character_type_map: &mut HashMap<usize, CharacterType>,
    ) -> DiskStatsFields<'_> {
        self.buffer_counter = 0;
        let mut found_character: bool = false;
        let mut field: usize = 0;
        let mut start: usize = 0;
        let mut prev_char_was_special: bool = false;
        let mut index_traces: Vec<(usize, usize, usize)> = Vec::new();
        while self.buffer_counter < self.buffer_len {
            let byte = self.buffer[self.buffer_counter];
            if byte == b'\n' {
                character_type_map.insert(self.buffer_counter, CharacterType::NewLine);
                if prev_char_was_special == false && found_character == true {
                    index_traces.push((field, start, self.buffer_counter));
                }
                prev_char_was_special = true;
                found_character = false;
            } else if byte == b' ' {
                character_type_map.insert(self.buffer_counter, CharacterType::Space);
                if prev_char_was_special == false && found_character == true {
                    index_traces.push((field, start, self.buffer_counter));
                }
                prev_char_was_special = true;
                found_character = false;
            } else if byte == b'\0' {
                character_type_map.insert(self.buffer_counter, CharacterType::EndOfFile);
                if prev_char_was_special == false && found_character == true {
                    index_traces.push((field, start, self.buffer_counter));
                }
                prev_char_was_special = true;
                found_character = false;
            } else if found_character == false {
                field += 1;
                start = self.buffer_counter;
                found_character = true;
                prev_char_was_special = false;
            }
            self.buffer_counter += 1;
        }
        DiskStatsFields::new(index_traces, &self.buffer.as_slice()[..self.buffer_counter])
    }
}

// this function parses the diskstats files from each block device including zram
// its thread so it runs in parallel with other parsers
pub fn parse_diskstats() {
    let sys_block_path = Path::new(SYS_FS_ROOT_PATH).join("block");
    let mut diskstats_paths = Vec::new();
    for entry in read_dir(&sys_block_path).into_iter().flatten().flatten() {
        let device_name = entry.file_name().to_string_lossy().into_owned();
        let block_device_path = sys_block_path.join(&device_name).join("stat");
        diskstats_paths.push(block_device_path);
    }
    let mut diskstats = Vec::new();
    for path in diskstats_paths {
        let diskstat = ProcFileDiskStats::new(&path.to_str().unwrap(), false);
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
    thread::spawn(move || {
        loop {
            for diskstat in diskstats.iter_mut() {
                diskstat.read();
                let string_converted = diskstat.parse_bytes();
                println!("String converted: {}", string_converted);
                // println!("Bytes: {:?}", diskstat.buffer);
                let mut character_type_map = HashMap::new();
                let mut parsed = diskstat.parse(&mut character_type_map);
                parsed.parse_as_human_readable(&[
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                ]);
                println!("Parsed: {:?}", parsed);
                println!("=====================================");
                // println!("Parsed: {:?}", character_type_map);
                thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    });
}
