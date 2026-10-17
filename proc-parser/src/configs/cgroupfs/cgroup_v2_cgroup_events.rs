use super::super::config::ProcFSConfig;
use super::super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize)]
pub struct CgroupFSV2CgroupEventConfigFields {
    pub populated: bool,
    pub frozen: bool,
}

pub type CgroupV2CgroupEventsConfig = ProcFSConfig<CgroupFSV2CgroupEventConfigFields>;

impl ProcFSConfigTrait for CgroupV2CgroupEventsConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([(1, self.fields.populated), (2, self.fields.frozen)])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow & self.fields.populated & self.fields.frozen
    }
}
