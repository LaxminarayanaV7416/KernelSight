use crate::common::cgroup_constants::CGROUP_V2_MOUNT_PATH;
use crate::common::file_reader::ProcFileReader;
use crate::common::parser_utils::line_tracker;
use crate::common::parser_utils::parse_u64_swar;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

/*
populated
    1 if the cgroup or its descendants contains any live processes; otherwise, 0.
frozen
    1 if the cgroup is frozen; otherwise, 0.
 */

pub struct CgroupFSV2MemoryStatReader {
    reader: ProcFileReader<256>,
    values: CgroupFSV2MemoryStatFields,
    lines_map: Option<HashMap<usize, usize>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CgroupFSV2MemoryStatFields {
    pub anon: u64,
    pub file: u64,
    pub kernel: u64,
    pub kernel_stack: u64,
    pub pagetables: u64,
    pub sec_pagetables: u64,
    pub percpu: u64,
    pub sock: u64,
    pub vmalloc: u64,
    pub shmem: u64,
    pub zswap: u64,
    pub zswapped: u64,
    pub zswap_incomp: u64,
    pub file_mapped: u64,
    pub file_dirty: u64,
    pub file_writeback: u64,
    pub swapcached: u64,
    pub anon_thp: u64,
    pub file_thp: u64,
    pub shmem_thp: u64,
    pub inactive_anon: u64,
    pub active_anon: u64,
    pub inactive_file: u64,
    pub active_file: u64,
    pub unevictable: u64,
    pub slab_reclaimable: u64,
    pub slab_unreclaimable: u64,
    pub slab: u64,
    pub hugetlb: u64,
    pub workingset_refault_anon: u64,
    pub workingset_refault_file: u64,
    pub workingset_activate_anon: u64,
    pub workingset_activate_file: u64,
    pub workingset_restore_anon: u64,
    pub workingset_restore_file: u64,
    pub workingset_nodereclaim: u64,
    pub pgdemote_kswapd: u64,
    pub pgdemote_direct: u64,
    pub pgdemote_khugepaged: u64,
    pub pgdemote_proactive: u64,
    pub pgsteal_kswapd: u64,
    pub pgsteal_direct: u64,
    pub pgsteal_khugepaged: u64,
    pub pgsteal_proactive: u64,
    pub pgscan_kswapd: u64,
    pub pgscan_direct: u64,
    pub pgscan_khugepaged: u64,
    pub pgscan_proactive: u64,
    pub pgrefill: u64,
    pub pgpromote_success: u64,
    pub pgscan: u64,
    pub pgsteal: u64,
    pub pswpin: u64,
    pub pswpout: u64,
    pub pgfault: u64,
    pub pgmajfault: u64,
    pub pgactivate: u64,
    pub pgdeactivate: u64,
    pub pglazyfree: u64,
    pub pglazyfreed: u64,
    pub swpin_zero: u64,
    pub swpout_zero: u64,
    pub zswpin: u64,
    pub zswpout: u64,
    pub zswpwb: u64,
    pub thp_fault_alloc: u64,
    pub thp_collapse_alloc: u64,
    pub thp_swpout: u64,
    pub thp_swpout_fallback: u64,
    pub numa_pages_migrated: u64,
    pub numa_pte_updates: u64,
    pub numa_hint_faults: u64,
}

impl CgroupFSV2MemoryStatReader {
    pub fn new(
        path: &str,
        is_cachable: bool,
        is_root: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reader: ProcFileReader::new(path, is_cachable, is_root)?,
            values: CgroupFSV2MemoryStatFields::default(),
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

    pub fn values(&self) -> &CgroupFSV2MemoryStatFields {
        &self.values
    }

    fn set_field(&mut self, field: usize, start: usize, end: usize) {
        let bytes = &self.reader.buffer[start..end];
        let mut seen_colon = false;
        let mut digit_start: Option<usize> = None;
        let mut digit_end: usize = 0;
        for (i, &byte) in bytes.iter().enumerate() {
            if !seen_colon {
                if byte == b' ' {
                    seen_colon = true;
                }
                continue;
            }
            match byte {
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
        let Some(digit_start) = digit_start else {
            return;
        };
        let value_bytes = &bytes[digit_start..digit_end];
        let value = parse_u64_swar(value_bytes).unwrap_or_default();

        match field {
            1 => self.values.anon = value,
            2 => self.values.file = value,
            3 => self.values.kernel = value,
            4 => self.values.kernel_stack = value,
            5 => self.values.pagetables = value,
            6 => self.values.sec_pagetables = value,
            7 => self.values.percpu = value,
            8 => self.values.sock = value,
            9 => self.values.vmalloc = value,
            10 => self.values.shmem = value,
            11 => self.values.zswap = value,
            12 => self.values.zswapped = value,
            13 => self.values.zswap_incomp = value,
            14 => self.values.file_mapped = value,
            15 => self.values.file_dirty = value,
            16 => self.values.file_writeback = value,
            17 => self.values.swapcached = value,
            18 => self.values.anon_thp = value,
            19 => self.values.file_thp = value,
            20 => self.values.shmem_thp = value,
            21 => self.values.inactive_anon = value,
            22 => self.values.active_anon = value,
            23 => self.values.inactive_file = value,
            24 => self.values.active_file = value,
            25 => self.values.unevictable = value,
            26 => self.values.slab_reclaimable = value,
            27 => self.values.slab_unreclaimable = value,
            28 => self.values.slab = value,
            29 => self.values.hugetlb = value,
            30 => self.values.workingset_refault_anon = value,
            31 => self.values.workingset_refault_file = value,
            32 => self.values.workingset_activate_anon = value,
            33 => self.values.workingset_activate_file = value,
            34 => self.values.workingset_restore_anon = value,
            35 => self.values.workingset_restore_file = value,
            36 => self.values.workingset_nodereclaim = value,
            37 => self.values.pgdemote_kswapd = value,
            38 => self.values.pgdemote_direct = value,
            39 => self.values.pgdemote_khugepaged = value,
            40 => self.values.pgdemote_proactive = value,
            41 => self.values.pgsteal_kswapd = value,
            42 => self.values.pgsteal_direct = value,
            43 => self.values.pgsteal_khugepaged = value,
            44 => self.values.pgsteal_proactive = value,
            45 => self.values.pgscan_kswapd = value,
            46 => self.values.pgscan_direct = value,
            47 => self.values.pgscan_khugepaged = value,
            48 => self.values.pgscan_proactive = value,
            49 => self.values.pgrefill = value,
            50 => self.values.pgpromote_success = value,
            51 => self.values.pgscan = value,
            52 => self.values.pgsteal = value,
            53 => self.values.pswpin = value,
            54 => self.values.pswpout = value,
            55 => self.values.pgfault = value,
            56 => self.values.pgmajfault = value,
            57 => self.values.pgactivate = value,
            58 => self.values.pgdeactivate = value,
            59 => self.values.pglazyfree = value,
            60 => self.values.pglazyfreed = value,
            61 => self.values.swpin_zero = value,
            62 => self.values.swpout_zero = value,
            63 => self.values.zswpin = value,
            64 => self.values.zswpout = value,
            65 => self.values.zswpwb = value,
            66 => self.values.thp_fault_alloc = value,
            67 => self.values.thp_collapse_alloc = value,
            68 => self.values.thp_swpout = value,
            69 => self.values.thp_swpout_fallback = value,
            70 => self.values.numa_pages_migrated = value,
            71 => self.values.numa_pte_updates = value,
            72 => self.values.numa_hint_faults = value,
            _ => {}
        }
    }

    fn parse_buffer(&mut self, field_filter: &HashMap<&str, (usize, bool)>) {
        // first step is to track line numbers for the fields we care about
        // this is used to map field numbers to line numbers in the buffer
        // once this is done we only parse the fields we care about
        // the complicated logic is explained in the parser_utils line_tracker function
        if self.lines_map.is_none() {
            self.lines_map = Some(line_tracker(
                &self.reader.buffer[..self.reader.buffer_len],
                field_filter,
                b' ',
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

pub fn parse_cgroup_v2_memory_stat(
    signal_hook: Arc<AtomicBool>,
    monitoring_heartbeat: u64,
    is_cachable: bool,
    is_root: bool,
    cgroup_name: &str,
) -> JoinHandle<()> {
    let cgroup_event_path = Path::new(CGROUP_V2_MOUNT_PATH)
        .join("system.slice")
        .join(cgroup_name)
        .join("memory.stat");
    let cgroup_event_temp_reader =
        CgroupFSV2MemoryStatReader::new(&cgroup_event_path.to_str().unwrap(), is_cachable, is_root);
    let mut cgroup_event_reader = match cgroup_event_temp_reader {
        Ok(reader) => Box::new(reader),
        Err(e) => panic!("Failed to create loadavg reader: {}", e),
    };
    // TODO: from here get the config which have HashMap<&str, (usize, bool)>
    // Just use the method get_field_string_to_struct_ids
    // let filter_map = ProcMemInfoConfig.get_field_string_to_struct_ids()
    let filter_map = HashMap::from([
        ("anon", (1, true)),
        ("file", (2, true)),
        ("kernel", (3, true)),
        ("kernel_stack", (4, true)),
        ("pagetables", (5, true)),
        ("sec_pagetables", (6, true)),
        ("percpu", (7, true)),
        ("sock", (8, true)),
        ("vmalloc", (9, true)),
        ("shmem", (10, true)),
        ("zswap", (11, true)),
        ("zswapped", (12, true)),
        ("zswap_incomp", (13, true)),
        ("file_mapped", (14, true)),
        ("file_dirty", (15, true)),
        ("file_writeback", (16, true)),
        ("swapcached", (17, true)),
        ("anon_thp", (18, true)),
        ("file_thp", (19, true)),
        ("shmem_thp", (20, true)),
        ("inactive_anon", (21, true)),
        ("active_anon", (22, true)),
        ("inactive_file", (23, true)),
        ("active_file", (24, true)),
        ("unevictable", (25, true)),
        ("slab_reclaimable", (26, true)),
        ("slab_unreclaimable", (27, true)),
        ("slab", (28, true)),
        ("hugetlb", (29, true)),
        ("workingset_refault_anon", (30, true)),
        ("workingset_refault_file", (31, true)),
        ("workingset_activate_anon", (32, true)),
        ("workingset_activate_file", (33, true)),
        ("workingset_restore_anon", (34, true)),
        ("workingset_restore_file", (35, true)),
        ("workingset_nodereclaim", (36, true)),
        ("pgdemote_kswapd", (37, true)),
        ("pgdemote_direct", (38, true)),
        ("pgdemote_khugepaged", (39, true)),
        ("pgdemote_proactive", (40, true)),
        ("pgsteal_kswapd", (41, true)),
        ("pgsteal_direct", (42, true)),
        ("pgsteal_khugepaged", (43, true)),
        ("pgsteal_proactive", (44, true)),
        ("pgscan_kswapd", (45, true)),
        ("pgscan_direct", (46, true)),
        ("pgscan_khugepaged", (47, true)),
        ("pgscan_proactive", (48, true)),
        ("pgrefill", (49, true)),
        ("pgpromote_success", (50, true)),
        ("pgscan", (51, true)),
        ("pgsteal", (52, true)),
        ("pswpin", (53, true)),
        ("pswpout", (54, true)),
        ("pgfault", (55, true)),
        ("pgmajfault", (56, true)),
        ("pgactivate", (57, true)),
        ("pgdeactivate", (58, true)),
        ("pglazyfree", (59, true)),
        ("pglazyfreed", (60, true)),
        ("swpin_zero", (61, true)),
        ("swpout_zero", (62, true)),
        ("zswpin", (63, true)),
        ("zswpout", (64, true)),
        ("zswpwb", (65, true)),
        ("thp_fault_alloc", (66, true)),
        ("thp_collapse_alloc", (67, true)),
        ("thp_swpout", (68, true)),
        ("thp_swpout_fallback", (69, true)),
        ("numa_pages_migrated", (70, true)),
        ("numa_pte_updates", (71, true)),
        ("numa_hint_faults", (72, true)),
    ]);
    let thread_handle = thread::spawn(move || {
        while !signal_hook.load(Ordering::Relaxed) {
            let read_status = cgroup_event_reader.read_and_parse(&filter_map);
            if read_status {
                println!("Parsed: {:?}", cgroup_event_reader.values());
                println!("=====================================");
                thread::sleep(std::time::Duration::from_millis(monitoring_heartbeat));
            } else {
                println!("Failed to read meminfo, ProcFS File closed!");
                break;
            }
        }
    });
    println!("Killed LoadAvg thread!!");
    thread_handle
}
