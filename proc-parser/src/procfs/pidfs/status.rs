use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::line_tracker;
use std::collections::HashMap;
use std::path::PathBuf;

/*
/proc/pid/status
       Provides much of the information in /proc/pid/stat and
       /proc/pid/statm in a format that's easier for humans to
       parse.  Here's an example:

           $ cat /proc/$$/status
           Name:   bash
           Umask:  0022
           State:  S (sleeping)
           Tgid:   17248
           Ngid:   0
           Pid:    17248
           PPid:   17200
           TracerPid:      0
           Uid:    1000    1000    1000    1000
           Gid:    100     100     100     100
           FDSize: 256
           Groups: 16 33 100
           NStgid: 17248
           NSpid:  17248
           NSpgid: 17248
           NSsid:  17200
           VmPeak:     131168 kB
           VmSize:     131168 kB
           VmLck:           0 kB
           VmPin:           0 kB
           VmHWM:       13484 kB
           VmRSS:       13484 kB
           RssAnon:     10264 kB
           RssFile:      3220 kB
           RssShmem:        0 kB
           VmData:      10332 kB
           VmStk:         136 kB
           VmExe:         992 kB
           VmLib:        2104 kB
           VmPTE:          76 kB
           VmPMD:          12 kB
           VmSwap:          0 kB
           HugetlbPages:          0 kB        # 4.4
           CoreDumping:   0                       # 4.15
           Threads:        1
           SigQ:   0/3067
           SigPnd: 0000000000000000
           ShdPnd: 0000000000000000
           SigBlk: 0000000000010000
           SigIgn: 0000000000384004
           SigCgt: 000000004b813efb
           CapInh: 0000000000000000
           CapPrm: 0000000000000000
           CapEff: 0000000000000000
           CapBnd: ffffffffffffffff
           CapAmb:   0000000000000000
           NoNewPrivs:     0
           Seccomp:        0
           Seccomp_filters:        0
           Speculation_Store_Bypass:       vulnerable
           Cpus_allowed:   00000001
           Cpus_allowed_list:      0
           Mems_allowed:   1
           Mems_allowed_list:      0
           voluntary_ctxt_switches:        150
           nonvoluntary_ctxt_switches:     545

       The fields are as follows:

       Name   Command run by this process.  Strings longer than
              TASK_COMM_LEN (16) characters (including the
              terminating null byte) are silently truncated.

       Umask  Process umask, expressed in octal with a leading
              zero; see umask(2).  (Since Linux 4.7.)

       State  Current state of the process.  One of "R (running)",
              "S (sleeping)", "D (disk sleep)", "T (stopped)", "t
              (tracing stop)", "Z (zombie)", or "X (dead)".

       Tgid   Thread group ID (i.e., Process ID).

       Ngid   NUMA group ID (0 if none; since Linux 3.13).

       Pid    Thread ID (see gettid(2)).

       PPid   PID of parent process.

       TracerPid
              PID of process tracing this process (0 if not being
              traced).

       Uid
       Gid    Real, effective, saved set, and filesystem UIDs
              (GIDs).

       FDSize Number of file descriptor slots currently allocated.

       Groups Supplementary group list.

       NStgid Thread group ID (i.e., PID) in each of the PID
              namespaces of which pid is a member.  The leftmost
              entry shows the value with respect to the PID
              namespace of the process that mounted this procfs
              (or the root namespace if mounted by the kernel),
              followed by the value in successively nested inner
              namespaces.  (Since Linux 4.1.)

       NSpid  Thread ID in each of the PID namespaces of which pid
              is a member.  The fields are ordered as for NStgid.
              (Since Linux 4.1.)

       NSpgid Process group ID in each of the PID namespaces of
              which pid is a member.  The fields are ordered as
              for NStgid.  (Since Linux 4.1.)

       NSsid  descendant namespace session ID hierarchy Session ID
              in each of the PID namespaces of which pid is a
              member.  The fields are ordered as for NStgid.
              (Since Linux 4.1.)

       VmPeak Peak virtual memory size.

       VmSize Virtual memory size.

       VmLck  Locked memory size (see mlock(2)).

       VmPin  Pinned memory size (since Linux 3.2).  These are
              pages that can't be moved because something needs to
              directly access physical memory.

       VmHWM  Peak resident set size ("high water mark").  This
              value is inaccurate; see /proc/pid/statm above.

       VmRSS  Resident set size.  Note that the value here is the
              sum of RssAnon, RssFile, and RssShmem.  This value
              is inaccurate; see /proc/pid/statm above.

       RssAnon
              Size of resident anonymous memory.  (since Linux
              4.5).  This value is inaccurate; see /proc/pid/statm
              above.

       RssFile
              Size of resident file mappings.  (since Linux 4.5).
              This value is inaccurate; see /proc/pid/statm above.

       RssShmem
              Size of resident shared memory (includes System V
              shared memory, mappings from tmpfs(5), and shared
              anonymous mappings).  (since Linux 4.5).

       VmData
       VmStk
       VmExe  Size of data, stack, and text segments.  This value
              is inaccurate; see /proc/pid/statm above.

       VmLib  Shared library code size.

       VmPTE  Page table entries size (since Linux 2.6.10).

       VmPMD  Size of second-level page tables (added in Linux
              4.0; removed in Linux 4.15).

       VmSwap Swapped-out virtual memory size by anonymous private
              pages; shmem swap usage is not included (since Linux
              2.6.34).  This value is inaccurate; see
              /proc/pid/statm above.

       HugetlbPages
              Size of hugetlb memory portions (since Linux 4.4).

       CoreDumping
              Contains the value 1 if the process is currently
              dumping core, and 0 if it is not (since Linux 4.15).
              This information can be used by a monitoring process
              to avoid killing a process that is currently dumping
              core, which could result in a corrupted core dump
              file.

       Threads
              Number of threads in process containing this thread.

       SigQ   This field contains two slash-separated numbers that
              relate to queued signals for the real user ID of
              this process.  The first of these is the number of
              currently queued signals for this real user ID, and
              the second is the resource limit on the number of
              queued signals for this process (see the description
              of RLIMIT_SIGPENDING in getrlimit(2)).

       SigPnd
       ShdPnd Mask (expressed in hexadecimal) of signals pending
              for thread and for process as a whole (see
              pthreads(7) and signal(7)).

       SigBlk
       SigIgn
       SigCgt Masks (expressed in hexadecimal) indicating signals
              being blocked, ignored, and caught (see signal(7)).

       CapInh
       CapPrm
       CapEff Masks (expressed in hexadecimal) of capabilities
              enabled in inheritable, permitted, and effective
              sets (see capabilities(7)).

       CapBnd Capability bounding set, expressed in hexadecimal
              (since Linux 2.6.26, see capabilities(7)).

       CapAmb Ambient capability set, expressed in hexadecimal
              (since Linux 4.3, see capabilities(7)).

       NoNewPrivs
              Value of the no_new_privs bit (since Linux 4.10, see
              prctl(2)).

       Seccomp
              Seccomp mode of the process (since Linux 3.8, see
              seccomp(2)).  0 means SECCOMP_MODE_DISABLED; 1 means
              SECCOMP_MODE_STRICT; 2 means SECCOMP_MODE_FILTER.
              This field is provided only if the kernel was built
              with the CONFIG_SECCOMP kernel configuration option
              enabled.

       Seccomp_filters
              Number of seccomp filters attached to the process
              (since Linux 5.9, see seccomp(2)).

       Speculation_Store_Bypass
              Speculation flaw mitigation state (since Linux 4.17,
              see prctl(2)).

       Cpus_allowed
              Hexadecimal mask of CPUs on which this process may
              run (since Linux 2.6.24, see cpuset(7)).

       Cpus_allowed_list
              Same as previous, but in "list format" (since Linux
              2.6.26, see cpuset(7)).

       Mems_allowed
              Mask of memory nodes allowed to this process (since
              Linux 2.6.24, see cpuset(7)).

       Mems_allowed_list
              Same as previous, but in "list format" (since Linux
              2.6.26, see cpuset(7)).

       voluntary_ctxt_switches
       nonvoluntary_ctxt_switches
              Number of voluntary and involuntary context switches
              (since Linux 2.6.23).
*/

#[derive(Debug)]
pub struct ProcFSPIDStatusReader {
    reader: ProcFileReader<4096>,
    values: Box<ProcPIDStatusFields>,
    lines_map: Option<HashMap<usize, usize>>,
}

#[derive(Debug, Default, Clone)]
pub struct ProcPIDStatusFields {
    pub name: String,
    pub umask: String,
    pub state: String,
    pub tgid: u64,
    pub ngid: u64,
    pub pid: u64,
    pub ppid: u64,
    pub tracer_pid: u64,
    pub uid: u64,
    pub gid: u64,
    pub fd_size: u64,
    pub groups: String,
    pub nstgid: u64,
    pub nspid: u64,
    pub nspgid: u64,
    pub nssid: u64,
    pub kthread: u64,
    pub vm_peak: u64,
    pub vm_size: u64,
    pub vm_lck: u64,
    pub vm_pin: u64,
    pub vm_hwm: u64,
    pub vm_rss: u64,
    pub rss_anon: u64,
    pub rss_file: u64,
    pub rss_shmem: u64,
    pub vm_data: u64,
    pub vm_stk: u64,
    pub vm_exe: u64,
    pub vm_lib: u64,
    pub vm_pte: u64,
    pub vm_swap: u64,
    pub hugetlb_pages: u64,
    pub core_dumping: u64,
    pub thp_enabled: u64,
    pub untag_mask: String,
    pub threads: u64,
    pub sigq: String,
    pub sigpnd: String,
    pub shdpnd: String,
    pub sigblk: String,
    pub sigign: String,
    pub sigcgt: String,
    pub capinh: String,
    pub capprm: String,
    pub capeff: String,
    pub capbnd: String,
    pub capamb: String,
    pub nonewprivs: u64,
    pub seccomp: u64,
    pub seccomp_filters: u64,
    pub speculation_store_bypass: String,
    pub speculationindirectbranch: String,
    pub cpus_allowed: String,
    pub cpus_allowed_list: String,
    pub mems_allowed: String,
    pub mems_allowed_list: String,
    pub voluntary_ctxt_switches: u64,
    pub nonvoluntary_ctxt_switches: u64,
    pub x86_thread_features: u64,
    pub x86_thread_features_locked: u64,
}

impl ProcFSPIDStatusReader {
    pub fn new(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path)?,
            values: Box::new(ProcPIDStatusFields::default()),
            lines_map: None,
        })
    }

    pub fn read_and_parse(&mut self, field_filter: &HashMap<&str, (usize, bool)>) -> bool {
        let read_status = self.reader.read();
        if read_status {
            self.parse_buffer(&field_filter);
        }
        read_status
    }

    pub fn values(&self) -> &ProcPIDStatusFields {
        &self.values
    }

    #[inline]
    fn is_string_field(field: usize) -> bool {
        matches!(
            field,
            1 | 2
                | 3
                | 12
                | 36
                | 38
                | 39
                | 40
                | 41
                | 42
                | 43
                | 44
                | 45
                | 46
                | 47
                | 48
                | 52
                | 53
                | 54
                | 55
                | 56
                | 57
        )
    }

    fn set_numeric_field(&mut self, field: usize, value: u64) {
        match field {
            4 => self.values.tgid = value,
            5 => self.values.ngid = value,
            6 => self.values.pid = value,
            7 => self.values.ppid = value,
            8 => self.values.tracer_pid = value,
            9 => self.values.uid = value,
            10 => self.values.gid = value,
            11 => self.values.fd_size = value,
            13 => self.values.nstgid = value,
            14 => self.values.nspid = value,
            15 => self.values.nspgid = value,
            16 => self.values.nssid = value,
            17 => self.values.kthread = value,
            18 => self.values.vm_peak = value,
            19 => self.values.vm_size = value,
            20 => self.values.vm_lck = value,
            21 => self.values.vm_pin = value,
            22 => self.values.vm_hwm = value,
            23 => self.values.vm_rss = value,
            24 => self.values.rss_anon = value,
            25 => self.values.rss_file = value,
            26 => self.values.rss_shmem = value,
            27 => self.values.vm_data = value,
            28 => self.values.vm_stk = value,
            29 => self.values.vm_exe = value,
            30 => self.values.vm_lib = value,
            31 => self.values.vm_pte = value,
            32 => self.values.vm_swap = value,
            33 => self.values.hugetlb_pages = value,
            34 => self.values.core_dumping = value,
            35 => self.values.thp_enabled = value,
            37 => self.values.threads = value,
            49 => self.values.nonewprivs = value,
            50 => self.values.seccomp = value,
            51 => self.values.seccomp_filters = value,
            58 => self.values.voluntary_ctxt_switches = value,
            59 => self.values.nonvoluntary_ctxt_switches = value,
            60 => self.values.x86_thread_features = value,
            61 => self.values.x86_thread_features_locked = value,
            _ => {}
        }
    }

    fn finish_string_field(&mut self, field: usize, start: Option<usize>, end: usize) {
        let Some(start) = start else {
            return;
        };

        if start >= end {
            self.set_string_field(field, String::new());
            return;
        }

        let value = {
            let bytes = &self.reader.buffer[start..end];

            match std::str::from_utf8(bytes) {
                Ok(value) => value.to_owned(),
                Err(_) => String::new(),
            }
        };

        self.set_string_field(field, value);
    }

    fn set_string_field(&mut self, field: usize, value: String) {
        match field {
            1 => self.values.name = value,
            2 => self.values.umask = value,
            3 => self.values.state = value,
            12 => self.values.groups = value,
            36 => self.values.untag_mask = value,
            38 => self.values.sigq = value,
            39 => self.values.sigpnd = value,
            40 => self.values.shdpnd = value,
            41 => self.values.sigblk = value,
            42 => self.values.sigign = value,
            43 => self.values.sigcgt = value,
            44 => self.values.capinh = value,
            45 => self.values.capprm = value,
            46 => self.values.capeff = value,
            47 => self.values.capbnd = value,
            48 => self.values.capamb = value,
            52 => self.values.speculation_store_bypass = value,
            53 => self.values.speculationindirectbranch = value,
            54 => self.values.cpus_allowed = value,
            55 => self.values.cpus_allowed_list = value,
            56 => self.values.mems_allowed = value,
            57 => self.values.mems_allowed_list = value,
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<&str, (usize, bool)>) {
        if self.lines_map.is_none() {
            self.lines_map = Some(line_tracker(
                &self.reader.buffer[..self.reader.buffer_len],
                field_filter,
                b':',
            ));
        }
        let mut line_number = 1usize;
        let mut field = self
            .lines_map
            .as_ref()
            .and_then(|lines| lines.get(&line_number))
            .copied()
            .unwrap_or(0);
        let mut seen_colon = false;
        let mut is_string = Self::is_string_field(field);
        let mut string_start: Option<usize> = None;
        let mut number_started = false;
        let mut number_finished = false;
        let mut number = 0u64;
        let mut digit_count = 0usize;
        let mut number_valid = true;
        for index in 0..self.reader.buffer_len {
            // Copy the byte so no buffer borrow survives calls taking &mut self.
            let byte = self.reader.buffer[index];
            if matches!(byte, b'\n' | b'\0') {
                if field > 0 {
                    if is_string {
                        self.finish_string_field(field, string_start, index);
                    } else if number_started {
                        let value = if number_valid && digit_count > 0 {
                            number
                        } else {
                            0
                        };
                        self.set_numeric_field(field, value);
                    }
                }
                line_number += 1;
                field = self
                    .lines_map
                    .as_ref()
                    .and_then(|lines| lines.get(&line_number))
                    .copied()
                    .unwrap_or(0);

                seen_colon = false;
                is_string = Self::is_string_field(field);
                string_start = None;
                number_started = false;
                number_finished = false;
                number = 0;
                digit_count = 0;
                number_valid = true;
                continue;
            }
            // line_tracker maps unselected and unknown lines to field zero.
            if field == 0 {
                continue;
            }
            if !seen_colon {
                if byte == b':' {
                    seen_colon = true;
                }
                continue;
            }
            if is_string {
                if string_start.is_none() {
                    // Skip the whitespace separating ':' from the value.
                    if byte.is_ascii_whitespace() {
                        continue;
                    }
                    string_start = Some(index);
                }
                continue;
            }
            if number_finished {
                continue;
            }
            match byte {
                b'0'..=b'9' => {
                    number_started = true;
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
                byte if byte.is_ascii_whitespace() => {
                    if number_started {
                        // Ignore units such as "kB" and additional values such
                        // as the effective/saved UID columns.
                        number_finished = true;
                    }
                }
                _ if number_started => {
                    number_finished = true;
                }
                _ => {
                    // Ignore characters before the first decimal value.
                }
            }
        }
        // Handle the final line if it has no trailing newline or NUL.
        if field > 0 {
            if is_string {
                self.finish_string_field(field, string_start, self.reader.buffer_len);
            } else if number_started {
                let value = if number_valid && digit_count > 0 {
                    number
                } else {
                    0
                };

                self.set_numeric_field(field, value);
            }
        }
    }
}
