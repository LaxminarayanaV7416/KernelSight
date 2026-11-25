use std::iter::repeat_with;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct MessagePool<T> {
    messages: Vec<Arc<RwLock<T>>>,
}

impl<T: Default> MessagePool<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            messages: repeat_with(|| Arc::new(RwLock::new(T::default())))
                .take(capacity)
                .collect(),
        }
    }

    // try to find the free messages slot usign the strong
    // reference counting of stomic referncing
    pub fn acquire(&self) -> Option<Arc<RwLock<T>>> {
        for slot in &self.messages {
            if Arc::strong_count(slot) == 1 {
                return Some(Arc::clone(slot));
            }
        }
        None
    }

    // debug trails for curiocity
    pub fn debug_count_free_and_busy(&self) {
        for (i, slot) in self.messages.iter().enumerate() {
            println!("Slot {} strong_count = {}", i, Arc::strong_count(slot));
        }
    }

    // use the below mesage to send the message through the thread messenger
    pub fn send_message(&self, transmitter: Sender<Arc<RwLock<T>>>, message: T) -> bool {
        let Some(slot) = self.acquire() else {
            return false;
        };
        if let Ok(mut guard) = slot.write() {
            *guard = message;
        } else {
            return false; // Lock was poisoned
        }
        transmitter.send(slot).is_ok()
    }
}
