use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::parse_u64_swar;
use crate::common::parser_utils::stat_line_tracker;
use crate::common::procfs_constants::AVAILABLE_LOGICAL_CPUS;
use crate::common::procfs_constants::PROC_FS_ROOT_PATH;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

/*
/proc/stat
       kernel/system statistics.  Varies with architecture.
       Common entries include:

       cpu 10132153 290696 3084719 46828483 16683 0 25195 0 175628
       0
       cpu0 1393280 32966 572056 13343292 6130 0 17875 0 23933 0
              The amount of time, measured in units of USER_HZ
              (1/100ths of a second on most architectures, use
              sysconf(_SC_CLK_TCK) to obtain the right value),
              that the system ("cpu" line) or the specific CPU
              ("cpuN" line) spent in various states:

              user   (1) Time spent in user mode.

              nice   (2) Time spent in user mode with low priority
                     (nice).

              system (3) Time spent in system mode.

              idle   (4) Time spent in the idle task.  This value
                     should be USER_HZ times the second entry in
                     the /proc/uptime pseudo-file.

              iowait (since Linux 2.5.41)
                     (5) Time waiting for I/O to complete.  This
                     value is not reliable, for the following
                     reasons:

                     •  The CPU will not wait for I/O to complete;
                        iowait is the time that a task is waiting
                        for I/O to complete.  When a CPU goes into
                        idle state for outstanding task I/O,
                        another task will be scheduled on this
                        CPU.

                     •  On a multi-core CPU, the task waiting for
                        I/O to complete is not running on any CPU,
                        so the iowait of each CPU is difficult to
                        calculate.

                     •  The value in this field may decrease in
                        certain conditions.

              irq (since Linux 2.6.0)
                     (6) Time servicing interrupts.

              softirq (since Linux 2.6.0)
                     (7) Time servicing softirqs.

              steal (since Linux 2.6.11)
                     (8) Stolen time, which is the time spent in
                     other operating systems when running in a
                     virtualized environment

              guest (since Linux 2.6.24)
                     (9) Time spent running a virtual CPU for
                     guest operating systems under the control of
                     the Linux kernel.

              guest_nice (since Linux 2.6.33)
                     (10) Time spent running a niced guest
                     (virtual CPU for guest operating systems
                     under the control of the Linux kernel).

       page 5741 1808
              The number of pages the system paged in and the
              number that were paged out (from disk).

       swap 1 0
              The number of swap pages that have been brought in
              and out.

       intr 1462898
              This line shows counts of interrupts serviced since
              boot time, for each of the possible system
              interrupts.  The first column is the total of all
              interrupts serviced including unnumbered
              architecture specific interrupts; each subsequent
              column is the total for that particular numbered
              interrupt.  Unnumbered interrupts are not shown,
              only summed into the total.

       disk_io: (2,0):(31,30,5764,1,2) (3,0):...
              (major,disk_idx):(noinfo, read_io_ops, blks_read,
              write_io_ops, blks_written)
              (Linux 2.4 only)

       ctxt 115315
              The number of context switches that the system
              underwent.

       btime 769041601
              boot time, in seconds since the Epoch, 1970-01-01
              00:00:00 +0000 (UTC).

       processes 86031
              Number of forks since boot.

       procs_running 6
              Number of processes in runnable state.  (Linux
              2.5.45 onward.)

       procs_blocked 2
              Number of processes blocked waiting for I/O to
              complete.  (Linux 2.5.45 onward.)

       softirq 229245889 94 60001584 13619 5175704 2471304 28
       51212741 59130143 0 51240672
              This line shows the number of softirq for all CPUs.
              The first column is the total of all softirqs and
              each subsequent column is the total for particular
              softirq.  (Linux 2.6.31 onward.)
 */

// #[derive(Debug, Default, Clone, Copy)]
// pub struct SoftIrqStats {
//     pub total: u64,
//     pub hi: u64,
//     pub timer: u64,
//     pub net_tx: u64,
//     pub net_rx: u64,
//     pub block: u64,
//     pub irq_poll: u64,
//     pub tasklet: u64,
//     pub sched: u64,
//     pub hrtimer: u64,
//     pub rcu: u64,
// }

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcFSStatCPUFields {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
    pub guest: u64,
    pub guest_nice: u64,
}

pub struct ProcFSStatReader {
    reader: ProcFileReader<6144, ()>,
    values: ProcStatFields,
    lines_map: Option<HashMap<usize, usize>>,
}

#[derive(Debug, Default, Clone)]
pub struct ProcStatFields {
    pub cpu: ProcFSStatCPUFields,
    pub cpu_cores: Vec<ProcFSStatCPUFields>,
    pub intr: u64, // here we dont need to scrape just get the total value
    pub ctxt: u64,
    pub btime: u64,
    pub processes: u64,
    pub processes_running: u64,
    pub processes_blocked: u64,
    pub softirq: u64, // here we dont need to scrape just get the total value
}

impl ProcFSStatReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: ProcStatFields::default(),
            lines_map: None,
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &ProcStatFields {
        &self.values
    }

    fn parse_cpus_line(buffer: &[u8]) -> ProcFSStatCPUFields {
        let mut values = [0u64; 10];
        let mut value_index = 0usize;

        let mut token_start: Option<usize> = None;
        let mut token_number = 0usize;

        for index in 0..=buffer.len() {
            let is_delimiter = index == buffer.len()
                || buffer[index] == b' '
                || buffer[index] == b'\t'
                || buffer[index] == b'\n'
                || buffer[index] == b'\0';

            if is_delimiter {
                if let Some(start) = token_start.take() {
                    token_number += 1;

                    // Token 1 is "cpu", "cpu0", "cpu1", etc.
                    if token_number > 1 && value_index < values.len() {
                        values[value_index] =
                            parse_u64_swar(&buffer[start..index]).unwrap_or_default();

                        value_index += 1;
                    }
                }
            } else if token_start.is_none() {
                token_start = Some(index);
            }
        }

        ProcFSStatCPUFields {
            user: values[0],
            nice: values[1],
            system: values[2],
            idle: values[3],
            iowait: values[4],
            irq: values[5],
            softirq: values[6],
            steal: values[7],
            guest: values[8],
            guest_nice: values[9],
        }
    }

    fn parse_remaining_fields(buffer: &[u8]) -> u64 {
        let mut token_start: Option<usize> = None;
        let mut token_number = 0usize;

        for index in 0..=buffer.len() {
            let is_delimiter = index == buffer.len()
                || buffer[index] == b' '
                || buffer[index] == b'\t'
                || buffer[index] == b'\n'
                || buffer[index] == b'\0';
            if is_delimiter {
                if let Some(start) = token_start.take() {
                    token_number += 1;

                    // Token 1 is the key, token 2 is the value we need.
                    if token_number == 2 {
                        return parse_u64_swar(&buffer[start..index]).unwrap_or_default();
                    }
                }
            } else if token_start.is_none() {
                token_start = Some(index);
            }
        }
        0
    }

    fn set_field(&mut self, field: usize, line_number: usize, start: usize, end: usize) {
        let bytes = &self.reader.buffer[start..end];

        match field {
            1 => {
                self.values.cpu = Self::parse_cpus_line(bytes);
            }
            2 => {
                // /proc/stat line 2 is cpu0, line 3 is cpu1, etc.
                // let core_index = line_number.saturating_sub(2);

                // if core_index < self.values.cpu_cores.len() {
                self.values.cpu_cores.push(Self::parse_cpus_line(bytes));
                // }
            }
            3 => self.values.intr = Self::parse_remaining_fields(bytes),
            4 => self.values.ctxt = Self::parse_remaining_fields(bytes),
            5 => self.values.btime = Self::parse_remaining_fields(bytes),
            6 => self.values.processes = Self::parse_remaining_fields(bytes),
            7 => self.values.processes_running = Self::parse_remaining_fields(bytes),
            8 => self.values.processes_blocked = Self::parse_remaining_fields(bytes),
            9 => self.values.softirq = Self::parse_remaining_fields(bytes),
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<usize, bool>) {
        if field_filter.is_empty() {
            return;
        }
        if self.lines_map.is_none() {
            self.lines_map = Some(stat_line_tracker(
                &self.reader.buffer[..self.reader.buffer_len],
            ));
        }
        let mut line_number = 0usize;
        let mut line_start = 0usize;
        for index in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[index];

            if byte == b'\n' || byte == b'\0' {
                line_number += 1;

                if line_start < index {
                    if let Some(field) = self
                        .lines_map
                        .as_ref()
                        .and_then(|lines| lines.get(&line_number))
                        .copied()
                    {
                        if field_filter.get(&field).copied().unwrap_or(false) {
                            self.set_field(field, line_number, line_start, index);
                        }
                    }
                }
                line_start = index + 1;
            }
        }
        // Handle a final line when the buffer does not end with '\n'.
        if line_start < self.reader.buffer_len {
            line_number += 1;

            if let Some(field) = self
                .lines_map
                .as_ref()
                .and_then(|lines| lines.get(&line_number))
                .copied()
            {
                if field_filter.get(&field).copied().unwrap_or(false) {
                    self.set_field(field, line_number, line_start, self.reader.buffer_len);
                }
            }
        }
    }
}

pub fn parse_procfs_stat(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
) -> JoinHandle<()> {
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH).join("stat");
    let load_avg_temp_reader =
        ProcFSStatReader::new(&load_avg_path.to_str().unwrap(), is_cachable, is_root);
    let mut load_avg_reader = match load_avg_temp_reader {
        Ok(reader) => Box::new(reader),
        Err(e) => panic!("Failed to create loadavg reader: {}", e),
    };
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
    ]);
    let thread_handle = thread::spawn(move || {
        while !signal_hook.load(Ordering::Relaxed) {
            let read_status = load_avg_reader.read_and_parse(&filter_map);
            if !read_status {
                println!("Failed to read loadavg, ProcFS File closed!");
                break;
            }
            println!("Parsed: {:?}", load_avg_reader.values());
            println!("=====================================");
            thread::sleep(std::time::Duration::from_millis(monitoring_heartbeat));
        }
        println!("Killed LoadAvg thread!!");
    });
    thread_handle
}
