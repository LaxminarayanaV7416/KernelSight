use crate::common::file_reader::ProcFileReader;
use crate::common::parser_error::Result;
use crate::common::parser_utils::line_tracker;
use std::collections::HashMap;
use std::path::PathBuf;

/*
populated
    1 if the cgroup or its descendants contains any live processes; otherwise, 0.
frozen
    1 if the cgroup is frozen; otherwise, 0.
 */

pub struct CgroupFSV2MemoryStatReader {
    reader: ProcFileReader<256>,
    values: Box<CgroupFSV2MemoryStatFields>,
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
    pub fn new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            reader: ProcFileReader::new(path)?,
            values: Box::new(CgroupFSV2MemoryStatFields::default()),
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

    #[inline]
    fn set_field(&mut self, field: usize, value: u64) {
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
        if field_filter.is_empty() {
            return;
        }
        // Fields vary between kernel versions. Build a name-based line map
        // once, then reuse it for subsequent samples.
        if self.lines_map.is_none() {
            self.lines_map = Some(line_tracker(
                &self.reader.buffer[..self.reader.buffer_len],
                field_filter,
                b' ',
            ));
        }
        let mut line_number = 1usize;
        let mut line_started = false;
        let mut field = self
            .lines_map
            .as_ref()
            .and_then(|lines| lines.get(&line_number))
            .copied()
            .unwrap_or(0);
        let mut key_finished = false;
        let mut value_started = false;
        let mut value_finished = false;
        let mut value = 0u64;
        let mut digit_count = 0usize;
        let mut valid = true;
        for index in 0..self.reader.buffer_len {
            // Copy the byte so set_field() can mutably borrow self.
            let byte = self.reader.buffer[index];
            if matches!(byte, b'\n' | b'\0') {
                if line_started {
                    if field > 0 && value_started {
                        let parsed_value = if valid && digit_count > 0 { value } else { 0 };

                        self.set_field(field, parsed_value);
                    }
                    line_number += 1;
                }
                line_started = false;
                field = self
                    .lines_map
                    .as_ref()
                    .and_then(|lines| lines.get(&line_number))
                    .copied()
                    .unwrap_or(0);
                key_finished = false;
                value_started = false;
                value_finished = false;
                value = 0;
                digit_count = 0;
                valid = true;
                continue;
            }
            line_started = true;
            if !key_finished {
                if matches!(byte, b' ' | b'\t') {
                    key_finished = true;
                }
                continue;
            }
            // Unknown and disabled fields are represented by field zero.
            if field == 0 || value_finished {
                continue;
            }
            match byte {
                b'0'..=b'9' => {
                    value_started = true;
                    digit_count += 1;
                    // Preserve parse_u64_swar's 20-digit limit.
                    if digit_count > 20 {
                        valid = false;
                        continue;
                    }
                    let digit = u64::from(byte - b'0');
                    value = match value
                        .checked_mul(10)
                        .and_then(|current| current.checked_add(digit))
                    {
                        Some(value) => value,
                        None => {
                            valid = false;
                            continue;
                        }
                    };
                }
                byte if byte.is_ascii_whitespace() => {
                    if value_started {
                        value_finished = true;
                    }
                }
                _ if value_started => {
                    // Only the first contiguous decimal value is relevant.
                    value_finished = true;
                }
                _ => {
                    valid = false;
                    value_finished = true;
                }
            }
        }
        // Handle a final line without trailing newline/NUL.
        if line_started && field > 0 && value_started {
            let parsed_value = if valid && digit_count > 0 { value } else { 0 };

            self.set_field(field, parsed_value);
        }
    }
}
