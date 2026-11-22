use super::kinsight_utils::PROCFS_ROOT_PARSER_THREAD_NAME;
use kinsight_common::ProcParserError;
use kinsight_common::Result;
use papaya::HashMap;
use std::thread::JoinHandle;
use std::time::Duration;

fn procfs_root_scanner() -> Result<JoinHandle<()>> {
    // this doesnt contain any hashmap for the pid tracking
    
}
