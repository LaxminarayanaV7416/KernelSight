use super::super::config::ProcFSConfig;
use super::super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize)]
pub struct CgroupV2PidsEventsConfigFields {
    pub max: bool,
}

pub type CgroupV2PidsEventsConfig = ProcFSConfig<CgroupV2PidsEventsConfigFields>;

impl ProcFSConfigTrait for CgroupV2PidsEventsConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([(1, self.fields.max)])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow & self.fields.max
    }
}
