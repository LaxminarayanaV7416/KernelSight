use kinsight_common::ProcParserError;
use kinsight_common::Result;
use kinsight_common::config::Config;
use kinsight_common::utils::initiate_mpsc_channel;
use kinsight_common::utils::signal_handler;
use kinsight_common::{CPU_CORES, CPU_HZ};
use kinsight_parser::procfs_metrics_collector;
use kinsight_writer::MessagePool;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;

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
    println!(
        "Available CPU Cores are {} and runnin at clock speed {} per second",
        *CPU_CORES, *CPU_HZ
    );
    // println!("{:?}", *config);
    let threads = procfs_metrics_collector(&CONFIG.heart_beat, signaller, &CONFIG);
    threads?.join().unwrap();
    Ok(())
}

// #[derive(Debug, Default)]
// struct Message {
//     id: u64,
// }
//
// fn main() -> Result<()> {
//     let signaller: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
//     signal_handler(&signaller);
//     let (transmitter_writer, receiver_writer): (
//         Sender<Arc<RwLock<Message>>>,
//         Receiver<Arc<RwLock<Message>>>,
//     ) = initiate_mpsc_channel::<Arc<RwLock<Message>>>();
//     let message_pool: MessagePool<Message> = MessagePool::new(10);
//     let producer_signaller = Arc::clone(&signaller);
//     let producer = thread::spawn(move || {
//         let mut count = 0u64;
//         while !producer_signaller.load(Ordering::Relaxed) {
//             let msg = Message { id: count };
//             message_pool.send_message(transmitter_writer.clone(), msg);
//             count += 1;
//             message_pool.debug_count_free_and_busy();
//             thread::sleep(Duration::from_millis(1000));
//         }
//     });
//     let consumer_signaller = Arc::clone(&signaller);
//     let consumer = thread::spawn(move || {
//         while !consumer_signaller.load(Ordering::Relaxed) {
//             match receiver_writer.recv_timeout(Duration::from_secs(1)) {
//                 Ok(data) => {
//                     let guard = data.read();
//                     println!("Message Recieved = {:?}", guard);
//                     thread::sleep(Duration::from_millis(1001));
//                 }
//                 Err(RecvTimeoutError::Timeout) => {
//                     // Normal: no message arrived during this interval.
//                 }
//                 Err(RecvTimeoutError::Disconnected) => {
//                     break;
//                 }
//             };
//         }
//     });
//     producer.join();
//     consumer.join();
//     Ok(())
// }
