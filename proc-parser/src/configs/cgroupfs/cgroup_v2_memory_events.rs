use super::super::config::ProcFSConfig;
use super::super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize)]
pub struct CgroupV2MemoryEventConfigFields {
    pub low: bool,
    pub high: bool,
    pub max: bool,
    pub oom: bool,
    pub oom_kill: bool,
    pub oom_group_kill: bool,
    pub sock_throttled: bool,
}

pub type CgroupV2MemoryEventConfig = ProcFSConfig<CgroupV2MemoryEventConfigFields>;

impl ProcFSConfigTrait for CgroupV2MemoryEventConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.low),
            (2, self.fields.high),
            (3, self.fields.max),
            (4, self.fields.oom),
            (5, self.fields.oom_kill),
            (6, self.fields.oom_group_kill),
            (7, self.fields.sock_throttled),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow
            & self.fields.low
            & self.fields.high
            & self.fields.max
            & self.fields.oom
            & self.fields.oom_kill
            & self.fields.oom_group_kill
            & self.fields.sock_throttled
    }
}
