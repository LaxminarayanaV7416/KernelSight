use crate::common::file_reader::ProcFileReader;
use crate::common::kernel_types::{UNSIGNED_INT, UNSIGNED_LONG};
use crate::common::parser_utils::{CharacterType, parse_u64_swar};
use crate::common::procfs_constants::SYS_FS_ROOT_PATH;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

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
struct ProcLoadAvgFields<'a> {
    pub load_avg_1m: (u8, &'a [u8], f64),
    pub load_avg_5m: (u8, &'a [u8], f64),
    pub load_avg_15m: (u8, &'a [u8], f64),
    pub num_runnable: (u8, &'a [u8], u64),
    pub num_total: (u8, &'a [u8], u64),
    pub last_pid: (u8, &'a [u8], u64),
}

impl<'a> ProcLoadAvgFields<'a> {
    pub fn new(upates: Vec<usize, usize, usize>, buffer: &'a [u8]) -> Self {
        // start with empty slices.
        Self {
            load_avg_1m: upates[0] as f64,
            load_avg_5m: upates[1] as f64,
            load_avg_15m: upates[2] as f64,
            num_runnable: 0,
            num_total: 0,
            last_pid: 0,
        }
    }

    pub fn parse_as_human_readable(&mut self, field_ids: &[u8]) {
        for &field_id in field_ids {
            match field_id {
                0 => self.load_avg_1m = 10,
                _ => {}
            }
        }
    }
}

pub type ProcFileLoadAvg = ProcFileReader<32, ProcLoadAvgFields<'static>>;

impl ProcFileLoadAvg {
    pub fn parse(
        &mut self,
        character_type_map: &mut HashMap<usize, CharacterType>,
    ) -> ProcLoadAvgFields<'_> {
    }
}

pub fn parse_procfs_loadavg(signal_hook: Arc<AtomicBool>) -> JoinHandle<()> {
    let thread_handle = thread::spawn(move || {
        loop {
            while !signal_hook.load(Ordering::Relaxed) {}
            println!("Killed LoadAvg thread!!");
        }
    });
    thread_handle
}
