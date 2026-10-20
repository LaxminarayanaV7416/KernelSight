use crate::cgroupfs::v2::cgroup_events::CgroupFSV2CgroupEventReader;
use crate::cgroupfs::v2::cgroup_procs::CgroupFSV2CgroupProcsReader;
use crate::cgroupfs::v2::cpu_stat::CgroupFSV2CPUStatReader;
use crate::cgroupfs::v2::memory_current::CgroupFSV2MemoryCurrentReader;
use crate::cgroupfs::v2::memory_events::CgroupFSV2MemoryEventReader;
use crate::cgroupfs::v2::memory_peak::CgroupFSV2MemoryPeakReader;
use crate::cgroupfs::v2::memory_stat::CgroupFSV2MemoryStatReader;
use crate::cgroupfs::v2::pids_current::CgroupFSV2PidsCurrentReader;
use crate::cgroupfs::v2::pids_events::CgroupFSV2PidsEventReader;
use crate::cgroupfs::v2::pids_peak::CgroupFSV2PidsPeakReader;
use crate::common::cgroup_constants::{
    CGROUP_CPU_PRESSURE_FILE_SLUG, CGROUP_CPU_STAT_FILE_SLUG, CGROUP_EVENTS_FILE_SLUG,
    CGROUP_IO_PRESSURE_FILE_SLUG, CGROUP_MEMORY_CURRENT_FILE_SLUG, CGROUP_MEMORY_EVENTS_FILE_SLUG,
    CGROUP_MEMORY_PEAK_FILE_SLUG, CGROUP_MEMORY_PRESSURE_FILE_SLUG, CGROUP_MEMORY_STAT_FILE_SLUG,
    CGROUP_PIDS_CURRENT_FILE_SLUG, CGROUP_PIDS_EVENTS_FILE_SLUG, CGROUP_PIDS_PEAK_FILE_SLUG,
    CGROUP_PROCS_FILE_SLUG, CGROUP_V2_MOUNT_PATH,
};
use crate::common::parser_error::ProcParserError;
use crate::common::parser_error::Result;
use crate::common::system_call_utils::monotonic;
use crate::configs::config::{Config, ProcFSConfigTrait};
use crate::procfs::pressure::pressure_util::ProcProcessGeneralReader;
use crate::writer::writer_utils::{WriteCgroupsData, WriteCgroupsDataMsg};
use aho_corasick::AhoCorasick;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub enum CgroupsV2Actions {
    Add(String),
    Remove(String),
}

pub struct CgroupsV2Readers {
    pub cgroup_events: Option<CgroupFSV2CgroupEventReader>,
    pub cgroup_procs: CgroupFSV2CgroupProcsReader,
    pub cpu_pressure: ProcProcessGeneralReader,
    pub cpu_stat: CgroupFSV2CPUStatReader,
    pub io_pressure: ProcProcessGeneralReader,
    pub memory_current: Option<CgroupFSV2MemoryCurrentReader>,
    pub memory_events: Option<CgroupFSV2MemoryEventReader>,
    pub memory_pressure: ProcProcessGeneralReader,
    pub memory_peak: Option<CgroupFSV2MemoryPeakReader>,
    pub memory_stat: Option<CgroupFSV2MemoryStatReader>,
    pub pids_current: Option<CgroupFSV2PidsCurrentReader>,
    pub pids_event: Option<CgroupFSV2PidsEventReader>,
    pub pids_peak: Option<CgroupFSV2PidsPeakReader>,
    pub cgroup_path: String,
}

impl CgroupsV2Readers {
    pub fn new(path: &str) -> Self {
        let base_path = Path::new(CGROUP_V2_MOUNT_PATH).join(path);

        let cgroup_events = match CgroupFSV2CgroupEventReader::new(
            &base_path.join(CGROUP_EVENTS_FILE_SLUG).to_str().unwrap(),
            false,
            false,
        ) {
            Ok(reader) => Some(reader),
            Err(_) => None,
        };
        let cgroup_procs = match CgroupFSV2CgroupProcsReader::new(
            &base_path.join(CGROUP_PROCS_FILE_SLUG).to_str().unwrap(),
            false,
            false,
        ) {
            Ok(reader) => reader,
            Err(_) => panic!("Somethign is wrong in Cgroups!"),
        };
        let cpu_pressure = match ProcProcessGeneralReader::new(
            &base_path
                .join(CGROUP_CPU_PRESSURE_FILE_SLUG)
                .to_str()
                .unwrap(),
            false,
            false,
        ) {
            Ok(reader) => reader,
            Err(_) => panic!("Somethign is wrong in Cgroups!"),
        };
        let cpu_stat = match CgroupFSV2CPUStatReader::new(
            &base_path.join(CGROUP_CPU_STAT_FILE_SLUG).to_str().unwrap(),
            false,
            false,
        ) {
            Ok(reader) => reader,
            Err(_) => panic!("Somethign is wrong in Cgroups!"),
        };
        let io_pressure = match ProcProcessGeneralReader::new(
            &base_path
                .join(CGROUP_IO_PRESSURE_FILE_SLUG)
                .to_str()
                .unwrap(),
            false,
            false,
        ) {
            Ok(reader) => reader,
            Err(_) => panic!("Somethign is wrong in Cgroups!"),
        };
        let memory_current = match CgroupFSV2MemoryCurrentReader::new(
            &base_path
                .join(CGROUP_MEMORY_CURRENT_FILE_SLUG)
                .to_str()
                .unwrap(),
            false,
            false,
        ) {
            Ok(reader) => Some(reader),
            Err(_) => None,
        };
        let memory_event = match CgroupFSV2MemoryEventReader::new(
            &base_path
                .join(CGROUP_MEMORY_EVENTS_FILE_SLUG)
                .to_str()
                .unwrap(),
            false,
            false,
        ) {
            Ok(reader) => Some(reader),
            Err(_) => None,
        };
        let memory_pressure = match ProcProcessGeneralReader::new(
            &base_path
                .join(CGROUP_MEMORY_PRESSURE_FILE_SLUG)
                .to_str()
                .unwrap(),
            false,
            false,
        ) {
            Ok(reader) => reader,
            Err(err) => panic!("Somethign is wrong in Cgroups!"),
        };
        let memory_peak = match CgroupFSV2MemoryPeakReader::new(
            &base_path
                .join(CGROUP_MEMORY_PEAK_FILE_SLUG)
                .to_str()
                .unwrap(),
            false,
            false,
        ) {
            Ok(reader) => Some(reader),
            Err(_) => None,
        };
        let memory_stat = match CgroupFSV2MemoryStatReader::new(
            &base_path
                .join(CGROUP_MEMORY_STAT_FILE_SLUG)
                .to_str()
                .unwrap(),
            false,
            false,
        ) {
            Ok(reader) => Some(reader),
            Err(_) => None,
        };
        let pids_current = match CgroupFSV2PidsCurrentReader::new(
            &base_path
                .join(CGROUP_PIDS_CURRENT_FILE_SLUG)
                .to_str()
                .unwrap(),
            false,
            false,
        ) {
            Ok(reader) => Some(reader),
            Err(_) => None,
        };
        let pids_event = match CgroupFSV2PidsEventReader::new(
            &base_path
                .join(CGROUP_PIDS_EVENTS_FILE_SLUG)
                .to_str()
                .unwrap(),
            false,
            false,
        ) {
            Ok(reader) => Some(reader),
            Err(_) => None,
        };
        let pids_peak = match CgroupFSV2PidsPeakReader::new(
            &base_path.join(CGROUP_PIDS_PEAK_FILE_SLUG).to_str().unwrap(),
            false,
            false,
        ) {
            Ok(reader) => Some(reader),
            Err(_) => None,
        };

        Self {
            cgroup_events: cgroup_events,
            cgroup_procs: cgroup_procs,
            cpu_pressure: cpu_pressure,
            cpu_stat: cpu_stat,
            io_pressure: io_pressure,
            memory_current: memory_current,
            memory_events: memory_event,
            memory_pressure: memory_pressure,
            memory_peak: memory_peak,
            memory_stat: memory_stat,
            pids_current: pids_current,
            pids_event: pids_event,
            pids_peak: pids_peak,
            cgroup_path: path.to_string(),
        }
    }

    pub fn read_cgroups(
        &mut self,
        transmit_cgroup_writer: &Sender<WriteCgroupsDataMsg>,
        is_cgroup_events_allowed: &bool,
        is_cpu_pressure_allowed: &bool,
        is_cpu_stat_allowed: &bool,
        is_io_pressure_allowed: &bool,
        is_memory_current_allowed: &bool,
        is_memory_events_allowed: &bool,
        is_memory_peak_allowed: &bool,
        is_memory_pressure_allowed: &bool,
        is_memory_stat_allowed: &bool,
        is_pids_current_allowed: &bool,
        is_pids_events_allowed: &bool,
        is_pids_peak_allowed: &bool,
        cgroup_events_filter_map: &HashMap<usize, bool>,
        cpu_pressure_filter_map: &HashMap<usize, bool>,
        cpu_stat_filter_map: &HashMap<&'static str, (usize, bool)>,
        io_pressure_filter_map: &HashMap<usize, bool>,
        memory_events_filter_map: &HashMap<&'static str, (usize, bool)>,
        memory_pressure_filter_map: &HashMap<usize, bool>,
        memory_stat_filter_map: &HashMap<&'static str, (usize, bool)>,
        pids_events_filter_map: &HashMap<&'static str, (usize, bool)>,
    ) -> bool {
        let time = monotonic();
        // here we loop the cgroup_procs and send the PIDS to ProcfsPIDS monitoring thread such that
        // it starts monitoring those specific pids inner files and our target is to monitor the pids
        // that we are targetting not the whole system, but we can get around this by writing the
        // match as empty in config.toml such that it picks the defualt pattern and matches all of
        // them so we basically get all the pids!!
        if self.cgroup_procs.read_and_parse() {
            let values = self.cgroup_procs.values();
            for pid in values.pids.iter() {
                println!("PID found >>> {}", pid);
            }
        }

        if *is_cgroup_events_allowed {
            let Some(reader) = self.cgroup_events.as_mut() else {
                return false;
            };
            if !reader.read_and_parse(cgroup_events_filter_map) {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::CgroupEvent(WriteCgroupsData::new(
                    time,
                    *reader.values(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        if *is_cpu_pressure_allowed {
            let reader = &mut self.cpu_pressure;
            if !reader.read_and_parse(cpu_pressure_filter_map) {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::CpuPressure(WriteCgroupsData::new(
                    time,
                    *reader.values(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        if *is_cpu_stat_allowed {
            let reader = &mut self.cpu_stat;
            if !reader.read_and_parse(cpu_stat_filter_map) {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::CpuStat(WriteCgroupsData::new(
                    time,
                    *reader.values(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        if *is_io_pressure_allowed {
            let reader = &mut self.io_pressure;
            if !reader.read_and_parse(io_pressure_filter_map) {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::IoPressure(WriteCgroupsData::new(
                    time,
                    *reader.values(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        if *is_memory_current_allowed {
            let Some(reader) = self.memory_current.as_mut() else {
                return false;
            };
            if !reader.read_and_parse() {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::MemoryCurrent(WriteCgroupsData::new(
                    time,
                    reader.values().clone(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        if *is_memory_events_allowed {
            let Some(reader) = self.memory_events.as_mut() else {
                return false;
            };
            if !reader.read_and_parse(memory_events_filter_map) {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::MemoryEvent(WriteCgroupsData::new(
                    time,
                    *reader.values(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        if *is_memory_pressure_allowed {
            let reader = &mut self.memory_pressure;
            if !reader.read_and_parse(memory_pressure_filter_map) {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::MemoryPressure(WriteCgroupsData::new(
                    time,
                    *reader.values(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        if *is_memory_peak_allowed {
            let Some(reader) = self.memory_peak.as_mut() else {
                return false;
            };
            if !reader.read_and_parse() {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::MemoryPeak(WriteCgroupsData::new(
                    time,
                    reader.values().clone(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        if *is_memory_stat_allowed {
            let Some(reader) = self.memory_stat.as_mut() else {
                return false;
            };
            if !reader.read_and_parse(memory_stat_filter_map) {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::MemoryStat(WriteCgroupsData::new(
                    time,
                    *reader.values(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        if *is_pids_current_allowed {
            let Some(reader) = self.pids_current.as_mut() else {
                return false;
            };
            if !reader.read_and_parse() {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::PidsCurrent(WriteCgroupsData::new(
                    time,
                    reader.values().clone(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        if *is_pids_events_allowed {
            let Some(reader) = self.pids_event.as_mut() else {
                return false;
            };
            if !reader.read_and_parse(pids_events_filter_map) {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::PidsEvent(WriteCgroupsData::new(
                    time,
                    *reader.values(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        if *is_pids_peak_allowed {
            let Some(reader) = self.pids_peak.as_mut() else {
                return false;
            };
            if !reader.read_and_parse() {
                return false;
            }
            transmit_cgroup_writer
                .send(WriteCgroupsDataMsg::PidsPeak(WriteCgroupsData::new(
                    time,
                    reader.values().clone(),
                    self.cgroup_path.clone(),
                )))
                .unwrap();
        }
        true
    }
}

pub fn cgroup_initiater(
    config: &'static Config,
    signaller: Arc<AtomicBool>, // dont clone this we should clone and send this
    receiver: Receiver<CgroupsV2Actions>,
    transmit_cgroup_writer: Sender<WriteCgroupsDataMsg>,
) -> Result<(JoinHandle<()>, JoinHandle<()>)> {
    let cgroups_data_set: Arc<RwLock<HashSet<String>>> = Arc::new(RwLock::new(HashSet::new()));
    let pattern_searcher = Box::new(
        AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .build(config.cgroup_patterns.clone())
            .unwrap(),
    );
    let cgroups_data_clone = Arc::clone(&cgroups_data_set);
    let parser_signaller = Arc::clone(&signaller);
    let parser_worker = thread::spawn(move || {
        while !parser_signaller.load(Ordering::Relaxed) {
            match receiver.recv_timeout(Duration::from_secs(1)) {
                Ok(CgroupsV2Actions::Add(cgroup)) => {
                    if pattern_searcher.is_match(cgroup.as_str()) {
                        let cgroup_stripped: &str = cgroup[4..].trim();
                        let mut data = cgroups_data_clone.write().unwrap();
                        if !data.contains(cgroup_stripped) {
                            data.insert(cgroup_stripped.to_string());
                        }
                    }
                }

                Ok(CgroupsV2Actions::Remove(cgroup)) => {
                    if pattern_searcher.is_match(cgroup.as_str()) {
                        let cgroup_stripped: &str = cgroup[4..].trim();
                        let mut data = cgroups_data_clone.write().unwrap();
                        data.remove(cgroup_stripped);
                    }
                }

                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }
    });
    let global_heart_beat = Box::new(config.heart_beat);
    let mut cgroup_map: HashMap<String, CgroupsV2Readers> = HashMap::new();
    // get all configs values
    let is_cgroup_events_allowed = Box::new(config.cgroupfs.cgroup_events.is_enabled());
    let is_cpu_pressure_allowed = Box::new(config.cgroupfs.cpu_pressure.is_enabled());
    let is_cpu_stat_allowed = Box::new(config.cgroupfs.cpu_stat.is_enabled());
    let is_io_pressure_allowed = Box::new(config.cgroupfs.io_pressure.is_enabled());
    let is_memory_current_allowed = Box::new(config.cgroupfs.memory_current.is_enabled());
    let is_memory_events_allowed = Box::new(config.cgroupfs.memory_events.is_enabled());
    let is_memory_peak_allowed = Box::new(config.cgroupfs.memory_peak.is_enabled());
    let is_memory_pressure_allowed = Box::new(config.cgroupfs.memory_pressure.is_enabled());
    let is_memory_stat_allowed = Box::new(config.cgroupfs.memory_stat.is_enabled());
    let is_pids_current_allowed = Box::new(config.cgroupfs.pids_current.is_enabled());
    let is_pids_events_allowed = Box::new(config.cgroupfs.pids_events.is_enabled());
    let is_pids_peak_allowed = Box::new(config.cgroupfs.pids_peak.is_enabled());

    let cgroup_events_filter_map = Box::new(config.cgroupfs.cgroup_events.get_hashmap());
    let cpu_pressure_filter_map = Box::new(config.cgroupfs.cpu_pressure.get_hashmap());
    let cpu_stat_filter_map = Box::new(config.cgroupfs.cpu_stat.get_field_string_to_struct_ids());
    let io_pressure_filter_map = Box::new(config.cgroupfs.io_pressure.get_hashmap());
    let memory_events_filter_map = Box::new(
        config
            .cgroupfs
            .memory_events
            .get_field_string_to_struct_ids(),
    );
    let memory_pressure_filter_map = Box::new(config.cgroupfs.memory_pressure.get_hashmap());
    let memory_stat_filter_map =
        Box::new(config.cgroupfs.memory_stat.get_field_string_to_struct_ids());
    let pids_events_filter_map =
        Box::new(config.cgroupfs.pids_events.get_field_string_to_struct_ids());

    let reader_worker = thread::spawn(move || {
        while !signaller.load(Ordering::Relaxed) {
            let cgroup_records: HashSet<String> = {
                let gaurd = match cgroups_data_set.read() {
                    Ok(records) => records,
                    Err(err) => {
                        eprintln!("Soemthign is wrong in cgroup fetcher!! {}", err);
                        return;
                    }
                };
                gaurd.clone()
            };

            // remove the cgroups that dont exist anymore in the procfs
            cgroup_map.retain(|pid, _| cgroup_records.contains(pid));

            for cgroup_path_slug in cgroup_records {
                // here handle the insertion or deletion of the new cgroup records
                if !cgroup_map.contains_key(&cgroup_path_slug) {
                    // add the new record
                    cgroup_map.insert(
                        cgroup_path_slug.clone(),
                        CgroupsV2Readers::new(cgroup_path_slug.as_str()),
                    );
                }
                let mut cgroup_map_record = match cgroup_map.get_mut(&cgroup_path_slug) {
                    Some(record) => record,
                    None => continue,
                };
                cgroup_map_record.read_cgroups(
                    &transmit_cgroup_writer,
                    &is_cgroup_events_allowed,
                    &is_cpu_pressure_allowed,
                    &is_cpu_stat_allowed,
                    &is_io_pressure_allowed,
                    &is_memory_current_allowed,
                    &is_memory_events_allowed,
                    &is_memory_peak_allowed,
                    &is_memory_pressure_allowed,
                    &is_memory_stat_allowed,
                    &is_pids_current_allowed,
                    &is_pids_events_allowed,
                    &is_pids_peak_allowed,
                    &cgroup_events_filter_map,
                    &cpu_pressure_filter_map,
                    &cpu_stat_filter_map,
                    &io_pressure_filter_map,
                    &memory_events_filter_map,
                    &memory_pressure_filter_map,
                    &memory_stat_filter_map,
                    &pids_events_filter_map,
                );
            }

            thread::sleep(Duration::from_millis(*global_heart_beat));
        }
    });

    Ok((parser_worker, reader_worker))
}
