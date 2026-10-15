use std::sync::mpsc::{Receiver, Sender, channel};

pub fn initiate_mpsc_channel<T>() -> (Sender<T>, Receiver<T>) {
    let (transmitter, receiver): (Sender<T>, Receiver<T>) = channel::<T>();
    (transmitter, receiver)
}
