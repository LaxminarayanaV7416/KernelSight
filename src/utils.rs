use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::flag;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub fn signal_handler(signaller: &Arc<AtomicBool>) {
    // register for SIGTERM and SIGINT
    flag::register(SIGTERM, Arc::clone(signaller)).unwrap();
    flag::register(SIGINT, Arc::clone(signaller)).unwrap();
}
