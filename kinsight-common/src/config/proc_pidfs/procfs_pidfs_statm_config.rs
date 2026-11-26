use super::super::config::ProcFSConfig;
use super::super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

// its the PID statm config
// its from the /proc/<PID>/statm

#[derive(Debug, Deserialize)]
pub struct ProcPidfsStatmConfigFields {
    pub size: bool,
    pub resident: bool,
    pub shared: bool,
    pub text: bool,
    pub lib: bool,
    pub data: bool,
    pub dirty: bool,
}

impl Default for ProcPidfsStatmConfigFields {
    fn default() -> Self {
        Self {
            size: true,
            resident: true,
            shared: true,
            text: true,
            lib: true,
            data: true,
            dirty: true,
        }
    }
}

pub type ProcPidfsStatmConfig = ProcFSConfig<ProcPidfsStatmConfigFields>;

impl ProcFSConfigTrait for ProcPidfsStatmConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.size),
            (2, self.fields.resident),
            (3, self.fields.shared),
            (4, self.fields.text),
            (5, self.fields.lib),
            (6, self.fields.data),
            (7, self.fields.dirty),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow
    }
}
