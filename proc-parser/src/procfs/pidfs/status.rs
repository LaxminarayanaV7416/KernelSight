use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::line_tracker;
use crate::common::parser_utils::parse_string;
use crate::common::parser_utils::parse_u64_swar;
use crate::common::procfs_constants::PROC_FS_ROOT_PATH;
use crate::configs::procfs_loadavg_config::ProcLoadAvgConfig;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

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

pub struct ProcFSPIDStatusReader {
    reader: ProcFileReader<4096, ()>,
    values: ProcPIDStatusFields,
    lines_map: Option<HashMap<usize, usize>>,
    string_field_ids: [usize; 22],
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
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: ProcPIDStatusFields::default(),
            lines_map: None,
            string_field_ids: [
                1, 2, 3, 12, 36, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 52, 53, 54, 55, 56, 57,
            ],
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

    fn set_field(&mut self, field: usize, start: usize, end: usize) {
        let bytes = &self.reader.buffer[start..end];
        let bytes_length = bytes.len();
        let mut seen_colon = false;
        let mut digit_start: Option<usize> = None;
        let mut digit_end: usize = 0;
        if self.string_field_ids.contains(&field) {
            for (i, &byte) in bytes.iter().enumerate() {
                if !seen_colon {
                    if byte == b':' {
                        seen_colon = true;
                    }
                    continue;
                }
                if digit_start.is_none() {
                    digit_start = Some(i);
                    digit_end = bytes_length;
                    break;
                }
            }
        } else {
            for (i, &byte) in bytes.iter().enumerate() {
                if !seen_colon {
                    if byte == b':' {
                        seen_colon = true;
                    }
                    continue;
                }
                match byte {
                    // TODO because of this the string parsing is not working
                    b'0'..=b'9' => {
                        if digit_start.is_none() {
                            digit_start = Some(i);
                        }

                        digit_end = i + 1;
                    }
                    _ => {
                        if digit_start.is_some() {
                            break;
                        }
                    }
                }
            }
        }
        let Some(digit_start) = digit_start else {
            return;
        };
        let value_bytes = &bytes[digit_start..digit_end];
        match field {
            1 => self.values.name = parse_string(value_bytes).unwrap_or_default(),
            2 => self.values.umask = parse_string(value_bytes).unwrap_or_default(),
            3 => self.values.state = parse_string(value_bytes).unwrap_or_default(),
            4 => self.values.tgid = parse_u64_swar(value_bytes).unwrap_or(0),
            5 => self.values.ngid = parse_u64_swar(value_bytes).unwrap_or(0),
            6 => self.values.pid = parse_u64_swar(value_bytes).unwrap_or(0),
            7 => self.values.ppid = parse_u64_swar(value_bytes).unwrap_or(0),
            8 => self.values.tracer_pid = parse_u64_swar(value_bytes).unwrap_or(0),
            9 => self.values.uid = parse_u64_swar(value_bytes).unwrap_or(0),
            10 => self.values.gid = parse_u64_swar(value_bytes).unwrap_or(0),
            11 => self.values.fd_size = parse_u64_swar(value_bytes).unwrap_or(0),
            12 => self.values.groups = parse_string(value_bytes).unwrap_or_default(),
            13 => self.values.nstgid = parse_u64_swar(value_bytes).unwrap_or(0),
            14 => self.values.nspid = parse_u64_swar(value_bytes).unwrap_or(0),
            15 => self.values.nspgid = parse_u64_swar(value_bytes).unwrap_or(0),
            16 => self.values.nssid = parse_u64_swar(value_bytes).unwrap_or(0),
            17 => self.values.kthread = parse_u64_swar(value_bytes).unwrap_or(0),
            18 => self.values.vm_peak = parse_u64_swar(value_bytes).unwrap_or(0),
            19 => self.values.vm_size = parse_u64_swar(value_bytes).unwrap_or(0),
            20 => self.values.vm_lck = parse_u64_swar(value_bytes).unwrap_or(0),
            21 => self.values.vm_pin = parse_u64_swar(value_bytes).unwrap_or(0),
            22 => self.values.vm_hwm = parse_u64_swar(value_bytes).unwrap_or(0),
            23 => self.values.vm_rss = parse_u64_swar(value_bytes).unwrap_or(0),
            24 => self.values.rss_anon = parse_u64_swar(value_bytes).unwrap_or(0),
            25 => self.values.rss_file = parse_u64_swar(value_bytes).unwrap_or(0),
            26 => self.values.rss_shmem = parse_u64_swar(value_bytes).unwrap_or(0),
            27 => self.values.vm_data = parse_u64_swar(value_bytes).unwrap_or(0),
            28 => self.values.vm_stk = parse_u64_swar(value_bytes).unwrap_or(0),
            29 => self.values.vm_exe = parse_u64_swar(value_bytes).unwrap_or(0),
            30 => self.values.vm_lib = parse_u64_swar(value_bytes).unwrap_or(0),
            31 => self.values.vm_pte = parse_u64_swar(value_bytes).unwrap_or(0),
            32 => self.values.vm_swap = parse_u64_swar(value_bytes).unwrap_or(0),
            33 => self.values.hugetlb_pages = parse_u64_swar(value_bytes).unwrap_or(0),
            34 => self.values.core_dumping = parse_u64_swar(value_bytes).unwrap_or(0),
            35 => self.values.thp_enabled = parse_u64_swar(value_bytes).unwrap_or(0),
            36 => self.values.untag_mask = parse_string(value_bytes).unwrap_or_default(),
            37 => self.values.threads = parse_u64_swar(value_bytes).unwrap_or(0),
            38 => self.values.sigq = parse_string(value_bytes).unwrap_or_default(),
            39 => self.values.sigpnd = parse_string(value_bytes).unwrap_or_default(),
            40 => self.values.shdpnd = parse_string(value_bytes).unwrap_or_default(),
            41 => self.values.sigblk = parse_string(value_bytes).unwrap_or_default(),
            42 => self.values.sigign = parse_string(value_bytes).unwrap_or_default(),
            43 => self.values.sigcgt = parse_string(value_bytes).unwrap_or_default(),
            44 => self.values.capinh = parse_string(value_bytes).unwrap_or_default(),
            45 => self.values.capprm = parse_string(value_bytes).unwrap_or_default(),
            46 => self.values.capeff = parse_string(value_bytes).unwrap_or_default(),
            47 => self.values.capbnd = parse_string(value_bytes).unwrap_or_default(),
            48 => self.values.capamb = parse_string(value_bytes).unwrap_or_default(),
            49 => self.values.nonewprivs = parse_u64_swar(value_bytes).unwrap_or(0),
            50 => self.values.seccomp = parse_u64_swar(value_bytes).unwrap_or(0),
            51 => self.values.seccomp_filters = parse_u64_swar(value_bytes).unwrap_or(0),
            52 => {
                self.values.speculation_store_bypass = parse_string(value_bytes).unwrap_or_default()
            }
            53 => {
                self.values.speculationindirectbranch =
                    parse_string(value_bytes).unwrap_or_default()
            }
            54 => self.values.cpus_allowed = parse_string(value_bytes).unwrap_or_default(),
            55 => self.values.cpus_allowed_list = parse_string(value_bytes).unwrap_or_default(),
            56 => self.values.mems_allowed = parse_string(value_bytes).unwrap_or_default(),
            57 => self.values.mems_allowed_list = parse_string(value_bytes).unwrap_or_default(),
            58 => self.values.voluntary_ctxt_switches = parse_u64_swar(value_bytes).unwrap_or(0),
            59 => self.values.nonvoluntary_ctxt_switches = parse_u64_swar(value_bytes).unwrap_or(0),
            60 => self.values.x86_thread_features = parse_u64_swar(value_bytes).unwrap_or(0),
            61 => self.values.x86_thread_features_locked = parse_u64_swar(value_bytes).unwrap_or(0),
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<&str, (usize, bool)>) {
        if self.lines_map.is_none() {
            self.lines_map = Some(line_tracker(
                &self.reader.buffer[..self.reader.buffer_len],
                field_filter, b':'
            ));
        }
        let mut parser_line_number = 0usize;
        let mut start: Option<usize> = None;

        for i in 0..self.reader.buffer_len {
            let byte = self.reader.buffer[i];

            if byte == b'\n' || byte == b'\0' {
                if let Some(s) = start.take() {
                    parser_line_number += 1;
                    // now get the field number from the line number
                    let field_number = self
                        .lines_map
                        .as_ref()
                        .unwrap()
                        .get(&parser_line_number)
                        .copied()
                        .unwrap_or(0);

                    if field_number > 0 {
                        self.set_field(field_number, s, i);
                    }
                }
            } else if start.is_none() {
                start = Some(i);
            }
        }
        if let Some(s) = start {
            parser_line_number += 1;
            let field_number = self
                .lines_map
                .as_ref()
                .unwrap()
                .get(&parser_line_number)
                .copied()
                .unwrap_or(0);

            if field_number > 0 {
                self.set_field(field_number, s, self.reader.buffer_len);
            }
        }
    }
}

pub fn parse_procfs_pid_status(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
    pid: usize,
) -> JoinHandle<()> {
    let load_avg_path = Path::new(PROC_FS_ROOT_PATH)
        .join(pid.to_string())
        .join("status");
    let load_avg_temp_reader =
        ProcFSPIDStatusReader::new(&load_avg_path.to_str().unwrap(), is_cachable, is_root);
    let mut load_avg_reader = match load_avg_temp_reader {
        Ok(reader) => Box::new(reader),
        Err(e) => panic!("Failed to create loadavg reader: {}", e),
    };
    let filter_map = HashMap::from([
        ("Name", (1, true)),
        ("Umask", (2, true)),
        ("State", (3, true)),
        ("Tgid", (4, true)),
        ("Ngid", (5, true)),
        ("Pid", (6, true)),
        ("PPid", (7, true)),
        ("TracerPid", (8, true)),
        ("Uid", (9, true)),
        ("Gid", (10, true)),
        ("FDSize", (11, true)),
        ("Groups", (12, true)),
        ("NStgid", (13, true)),
        ("NSpid", (14, true)),
        ("NSpgid", (15, true)),
        ("NSsid", (16, true)),
        ("Kthread", (17, true)),
        ("VmPeak", (18, true)),
        ("VmSize", (19, true)),
        ("VmLck", (20, true)),
        ("VmPin", (21, true)),
        ("VmHWM", (22, true)),
        ("VmRSS", (23, true)),
        ("RssAnon", (24, true)),
        ("RssFile", (25, true)),
        ("RssShmem", (26, true)),
        ("VmData", (27, true)),
        ("VmStk", (28, true)),
        ("VmExe", (29, true)),
        ("VmLib", (30, true)),
        ("VmPTE", (31, true)),
        ("VmSwap", (32, true)),
        ("HugetlbPages", (33, true)),
        ("CoreDumping", (34, true)),
        ("THP_enabled", (35, true)),
        ("untag_mask", (36, true)),
        ("Threads", (37, true)),
        ("SigQ", (38, true)),
        ("SigPnd", (39, true)),
        ("ShdPnd", (40, true)),
        ("SigBlk", (41, true)),
        ("SigIgn", (42, true)),
        ("SigCgt", (43, true)),
        ("CapInh", (44, true)),
        ("CapPrm", (45, true)),
        ("CapEff", (46, true)),
        ("CapBnd", (47, true)),
        ("CapAmb", (48, true)),
        ("NoNewPrivs", (49, true)),
        ("Seccomp", (50, true)),
        ("Seccomp_filters", (51, true)),
        ("Speculation_Store_Bypass", (52, true)),
        ("SpeculationIndirectBranch", (53, true)),
        ("Cpus_allowed", (54, true)),
        ("Cpus_allowed_list", (55, true)),
        ("Mems_allowed", (56, true)),
        ("Mems_allowed_list", (57, true)),
        ("voluntary_ctxt_switches", (58, true)),
        ("nonvoluntary_ctxt_switches", (59, true)),
        ("x86_Thread_features", (60, true)),
        ("x86_Thread_features_locked", (61, true)),
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
