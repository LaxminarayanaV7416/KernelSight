use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::flag;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub fn signal_handler() -> Arc<AtomicBool> {
    let term = Arc::new(AtomicBool::new(false));

    // register for SIGTERM and SIGINT
    flag::register(SIGTERM, Arc::clone(&term)).unwrap();
    flag::register(SIGINT, Arc::clone(&term)).unwrap();

    term
}


pub fn xor_bytes(a: &[u8], b: &[u8], dest: &mut [u8]) {
    assert!(a.len() == b.len() && b.len() == dest.len());
    for ((out_byte, &a_byte), &b_byte) in dest.iter_mut().zip(a.iter()).zip(b.iter()) {
        *out_byte = a_byte ^ b_byte;
    }
}
