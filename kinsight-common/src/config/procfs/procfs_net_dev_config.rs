use super::super::config::ProcFSConfig;
use super::super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

// its the Uptime config
// its from the /proc/net/dev

#[derive(Debug, Deserialize)]
pub struct ProcNetDevConfigFields {
    pub total_download_bytes: bool,
    pub total_donwload_packets: bool,
    pub bad_packets_downloaded: bool,
    pub recieved_packets_dropped: bool,
    pub total_upload_bytes: bool,
    pub total_upload_packets: bool,
    pub bad_packets_uploaded: bool,
    pub sent_packets_dropped: bool,
}

impl Default for ProcNetDevConfigFields {
    fn default() -> Self {
        Self {
            total_download_bytes: true,
            total_donwload_packets: true,
            bad_packets_downloaded: true,
            recieved_packets_dropped: true,
            total_upload_bytes: true,
            total_upload_packets: true,
            bad_packets_uploaded: true,
            sent_packets_dropped: true,
        }
    }
}

pub type ProcNetDevConfig = ProcFSConfig<ProcNetDevConfigFields>;

impl ProcFSConfigTrait for ProcNetDevConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.total_download_bytes),
            (2, self.fields.total_donwload_packets),
            (3, self.fields.bad_packets_downloaded),
            (4, self.fields.recieved_packets_dropped),
            (5, self.fields.total_upload_bytes),
            (6, self.fields.total_upload_packets),
            (7, self.fields.bad_packets_uploaded),
            (8, self.fields.sent_packets_dropped),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::new()
    }

    fn is_enabled(&self) -> bool {
        self.allow
    }
}
