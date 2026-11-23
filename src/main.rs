use kinsight_common::CPU_CORES;
use kinsight_common::ProcParserError;
use kinsight_common::Result;
use kinsight_common::config::Config;
use kinsight_common::utils::signal_handler;
use kinsight_parser::procfs_root_scanner;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::atomic::AtomicBool;

const PATH: &str = "/home/lax/Documents/perf_insight_lite/proc-parser/src/config.toml";
static CONFIG: LazyLock<Box<Config>> = LazyLock::new(|| {
    let content = std::fs::read_to_string(PATH)
        .map_err(|err| ProcParserError::FileReadError(err.to_string()))
        .unwrap_or_else(|e| panic!("Config error: {e:?}"));

    let parsed: Config = basic_toml::from_str(&content)
        .map_err(|err| ProcParserError::FileDoesntExistError(err.to_string())) // or your variant
        .unwrap_or_else(|e| panic!("Config error: {e:?}"));

    Box::new(parsed)
});

fn main() -> Result<()> {
    let signaller: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    signal_handler(&signaller);
    println!("Available CPU Cores are {}", *CPU_CORES);
    // println!("{:?}", *config);
    let threads = procfs_root_scanner(&CONFIG.heart_beat, signaller, &CONFIG);
    threads?.join().unwrap();
    Ok(())
}
