use super::config::ProcFSConfig;
use super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

// its the Uptime config
// its from the /proc/uptime

#[derive(Debug, Default, Deserialize)]
pub struct ProcUptimeConfigFields {
    pub suspended_time: bool,
    pub idle_time: bool,
}

pub type ProcUptimeConfig = ProcFSConfig<ProcUptimeConfigFields>;

impl ProcFSConfigTrait for ProcUptimeConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([(1, self.fields.suspended_time), (2, self.fields.idle_time)])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow & self.fields.suspended_time & self.fields.idle_time
    }
}

// #[derive(Debug, Default, Deserialize)]
// pub struct ProcUptimeConfig {
//     pub allow: bool,
//     pub heart_beat: u64,
//     pub fields:
// }
//
// impl ProcUptimeConfig {
//     pub fn get_hashmap(&self) -> HashMap<usize, bool> {
//         HashMap::from([(1, self.suspended_time), (2, self.idle_time)])
//     }
// }
