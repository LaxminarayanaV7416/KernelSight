use std::thread;

use crate::common::file_reader::{LinearParser, ProcFileReader};
use crate::common::parser_utils::CharacterType;
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

enum DiskStatsField {
    ReadsCompleted = 1,
    ReadsMerged = 2,
    SectorsRead = 3,
    TimeSpentReading = 4, // in milliseconds
    WritesCompleted = 5,
    WritesMerged = 6,
    SectorsWritten = 7,
    TimeSpentWriting = 8, // in milliseconds
    IOsInProgress = 9,
    TimeSpentDoingIO = 10,                    // in milliseconds
    WeightedNumberofSecondsSpentDoingIO = 11, // in milliseconds
    DiscardsCompleted = 12,
    DiscardsMerged = 13,
    SectorsDiscarded = 14,
    TimeSpentDiscarding = 15, // in milliseconds
    FlushesCompleted = 16,
    TimeSpentFlushing = 17, // in milliseconds
}

pub type ProcFileDiskStats = ProcFileReader<1024, DiskStatsField>;

impl LinearParser for ProcFileDiskStats {
    fn parse(&mut self, character_type_map: &mut HashMap<usize, CharacterType>) {
        self.buffer_counter = 0;
        while self.buffer_counter < self.buffer_len {
            let byte = self.buffer[self.buffer_counter];
            if byte == b'\n' {
                character_type_map.insert(self.buffer_counter, CharacterType::NewLine);
            } else if byte == b' ' {
                character_type_map.insert(self.buffer_counter, CharacterType::Space);
            } else if byte == b'\t' {
                character_type_map.insert(self.buffer_counter, CharacterType::Tab);
            } else if byte == b'(' {
                character_type_map.insert(self.buffer_counter, CharacterType::ParanthesisStart);
            } else if byte == b')' {
                character_type_map.insert(self.buffer_counter, CharacterType::ParanthesisEnd);
            } else if byte == b'[' {
                character_type_map
                    .insert(self.buffer_counter, CharacterType::SquareParanthesisStart);
            } else if byte == b']' {
                character_type_map.insert(self.buffer_counter, CharacterType::SquareParanthesisEnd);
            } else if byte == b':' {
                character_type_map.insert(self.buffer_counter, CharacterType::Colon);
            } else if byte == b',' {
                character_type_map.insert(self.buffer_counter, CharacterType::Comma);
            } else if byte == b';' {
                character_type_map.insert(self.buffer_counter, CharacterType::SemiColon);
            } else if byte == b'\0' {
                character_type_map.insert(self.buffer_counter, CharacterType::EndOfFile);
            }
            self.buffer_counter += 1;
        }

        // TODO optimize this later with help of AI
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
                println!("Bytes: {:?}", diskstat.buffer);
                let mut character_type_map = HashMap::new();
                diskstat.parse(&mut character_type_map);
                println!("Parsed: {:?}", character_type_map);
                thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    });
}
