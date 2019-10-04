use futures::channel::mpsc;

mod receiver_reader;
mod sender_writer;

pub use receiver_reader::*;
pub use sender_writer::*;

pub fn channel<T>(buffer: usize) -> (SenderWriter<T>, ReceiverReader<T>) {
    let (sender, receiver) = mpsc::channel(buffer);
    (SenderWriter { sender }, ReceiverReader { receiver })
}
