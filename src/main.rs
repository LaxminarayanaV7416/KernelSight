use kinsight_common::CPU_CORES;
use kinsight_common::Result;

fn main() -> Result<()> {
    println!("Available CPU Cores are {}", *CPU_CORES);
    Ok(())
}
