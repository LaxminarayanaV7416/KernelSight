use serde::Deserialize;
// use std::collections::HashMap;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    heartbeat: u64,
}
