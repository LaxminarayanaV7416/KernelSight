use crate::common::file_reader::ProcFileReader;
use crate::common::parser_error::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

const fn hash_key(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    let mut index = 0usize;

    while index < bytes.len() {
        hash ^= bytes[index] as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        index += 1;
    }

    hash
}

const INTR_HASH: u64 = hash_key(b"intr");
const CTXT_HASH: u64 = hash_key(b"ctxt");
const BTIME_HASH: u64 = hash_key(b"btime");
const PROCESSES_HASH: u64 = hash_key(b"processes");
const PROCS_RUNNING_HASH: u64 = hash_key(b"procs_running");
const PROCS_BLOCKED_HASH: u64 = hash_key(b"procs_blocked");
const SOFTIRQ_HASH: u64 = hash_key(b"softirq");

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

impl ProcFSStatCPUFields {
    fn from_values(values: [u64; 10]) -> Self {
        Self {
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
}

#[derive(Debug)]
pub struct ProcFSStatReader {
    reader: ProcFileReader<6144>,
    values: Arc<Mutex<ProcStatFields>>,
}

#[derive(Debug, Default)]
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
    pub fn new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            reader: ProcFileReader::new(path)?,
            values: Arc::new(Mutex::new(ProcStatFields::default())),
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<usize, bool>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            if let Ok(mut values) = self.values.lock() {
                // Reuse the allocation between samples.
                values.cpu_cores.clear();
                self.parse_buffer(&mut *values, &field_filter);
            }
        }
        read_status
    }

    pub fn values(&self) -> &Arc<Mutex<ProcStatFields>> {
        &self.values
    }

    fn set_non_cpu_field(&self, values: &mut ProcStatFields, field: usize, value: u64) {
        match field {
            3 => values.intr = value,
            4 => values.ctxt = value,
            5 => values.btime = value,
            6 => values.processes = value,
            7 => values.processes_running = value,
            8 => values.processes_blocked = value,
            9 => values.softirq = value,
            _ => {}
        }
    }

    fn classify_key(key_hash: u64, key_length: usize, is_cpu_key: bool) -> usize {
        if is_cpu_key {
            return match key_length {
                3 => 1,   // cpu
                4.. => 2, // cpu0, cpu1, ...
                _ => 0,
            };
        }

        match (key_hash, key_length) {
            (INTR_HASH, 4) => 3,
            (CTXT_HASH, 4) => 4,
            (BTIME_HASH, 5) => 5,
            (PROCESSES_HASH, 9) => 6,
            (PROCS_RUNNING_HASH, 13) => 7,
            (PROCS_BLOCKED_HASH, 13) => 8,
            (SOFTIRQ_HASH, 7) => 9,
            _ => 0,
        }
    }

    fn parse_buffer(&self, values: &mut ProcStatFields, field_filter: &HashMap<usize, bool>) {
        if field_filter.is_empty() {
            return;
        }

        let mut parsing_key = true;
        let mut key_hash = FNV_OFFSET_BASIS;
        let mut key_length = 0usize;

        // Tracks whether the key has the form "cpu" or "cpu<digits>".
        let mut is_cpu_key = true;

        let mut field = 0usize;
        let mut selected = false;
        let mut is_cpu_line = false;

        let mut cpu_values = [0u64; 10];
        let mut cpu_value_index = 0usize;
        let mut first_value: Option<u64> = None;

        let mut parsing_number = false;
        let mut number = 0u64;
        let mut digit_count = 0usize;
        let mut number_valid = true;

        for index in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[index];
            let is_line_end = matches!(byte, b'\n' | b'\0');
            let is_separator = matches!(byte, b' ' | b'\t' | b'\n' | b'\0');

            if parsing_key {
                if is_separator {
                    if key_length > 0 {
                        field = Self::classify_key(key_hash, key_length, is_cpu_key);

                        selected = field > 0 && field_filter.get(&field).copied().unwrap_or(false);

                        is_cpu_line = matches!(field, 1 | 2);
                        parsing_key = false;
                    }
                } else {
                    key_hash ^= u64::from(byte);
                    key_hash = key_hash.wrapping_mul(FNV_PRIME);

                    is_cpu_key &= match key_length {
                        0 => byte == b'c',
                        1 => byte == b'p',
                        2 => byte == b'u',
                        _ => byte.is_ascii_digit(),
                    };

                    key_length += 1;
                }
            } else if selected {
                if is_separator {
                    if parsing_number {
                        let parsed_value = if number_valid && digit_count > 0 {
                            number
                        } else {
                            0
                        };

                        if is_cpu_line {
                            if cpu_value_index < cpu_values.len() {
                                cpu_values[cpu_value_index] = parsed_value;
                                cpu_value_index += 1;
                            }
                        } else if first_value.is_none() {
                            // intr and softirq contain many counters, but only
                            // their first numeric value is needed.
                            first_value = Some(parsed_value);
                        }

                        parsing_number = false;
                        number = 0;
                        digit_count = 0;
                        number_valid = true;
                    }
                } else if is_cpu_line && cpu_value_index == cpu_values.len() {
                    // The ten CPU fields that we consume have already been read.
                } else if !is_cpu_line && first_value.is_some() {
                    // Only the first numeric value is needed for non-CPU lines.
                } else {
                    parsing_number = true;

                    match byte {
                        b'0'..=b'9' => {
                            digit_count += 1;

                            // Preserve parse_u64_swar's maximum token length.
                            if digit_count > 20 {
                                number_valid = false;
                                continue;
                            }

                            let digit = u64::from(byte - b'0');

                            number = match number
                                .checked_mul(10)
                                .and_then(|value| value.checked_add(digit))
                            {
                                Some(value) => value,
                                None => {
                                    number_valid = false;
                                    continue;
                                }
                            };
                        }

                        _ => {
                            number_valid = false;
                        }
                    }
                }
            }

            if is_line_end {
                if selected {
                    match field {
                        1 => {
                            values.cpu = ProcFSStatCPUFields::from_values(cpu_values);
                        }

                        2 => {
                            values
                                .cpu_cores
                                .push(ProcFSStatCPUFields::from_values(cpu_values));
                        }

                        3..=9 => {
                            self.set_non_cpu_field(
                                &mut *values,
                                field,
                                first_value.unwrap_or_default(),
                            );
                        }

                        _ => {}
                    }
                }

                parsing_key = true;
                key_hash = FNV_OFFSET_BASIS;
                key_length = 0;
                is_cpu_key = true;

                field = 0;
                selected = false;
                is_cpu_line = false;

                cpu_values = [0; 10];
                cpu_value_index = 0;
                first_value = None;

                parsing_number = false;
                number = 0;
                digit_count = 0;
                number_valid = true;
            }
        }

        // Commit the final line when the buffer does not end with newline/NUL.
        if !parsing_key {
            if parsing_number && selected {
                let parsed_value = if number_valid && digit_count > 0 {
                    number
                } else {
                    0
                };

                if is_cpu_line {
                    if cpu_value_index < cpu_values.len() {
                        cpu_values[cpu_value_index] = parsed_value;
                    }
                } else if first_value.is_none() {
                    first_value = Some(parsed_value);
                }
            }

            if selected {
                match field {
                    1 => {
                        values.cpu = ProcFSStatCPUFields::from_values(cpu_values);
                    }

                    2 => {
                        values
                            .cpu_cores
                            .push(ProcFSStatCPUFields::from_values(cpu_values));
                    }

                    3..=9 => {
                        self.set_non_cpu_field(
                            &mut *values,
                            field,
                            first_value.unwrap_or_default(),
                        );
                    }

                    _ => {}
                }
            }
        }
    }
}
