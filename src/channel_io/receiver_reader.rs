use crate::io::AsyncReadItems;
use futures::channel::mpsc::{Receiver, TryRecvError};
use futures::stream::Stream;
use std::io::Result;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct ReceiverReader<T> {
    pub(super) receiver: Receiver<T>,
}

impl<T: Unpin> AsyncReadItems<T> for ReceiverReader<T> {
    fn poll_read_items(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        items: &mut [T],
    ) -> Poll<Result<usize>> {
        let mut items_iter = items.iter_mut();

        let first_slot = match items_iter.next() {
            None => return Poll::Ready(Ok(0)),
            Some(slot) => slot,
        };

        // For the first value we do a proper poll.
        match Pin::new(&mut self.receiver).poll_next(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(None) => return Poll::Ready(Ok(0)),
            Poll::Ready(Some(item)) => *first_slot = item,
        };

        // For the rest - we just try to get the values, and if nothing is there
        // we just short the write.
        let mut filled: usize = 1;
        for item_slot in items_iter {
            match self.receiver.try_next() {
                Err(TryRecvError { .. }) | Ok(None) => break,
                Ok(Some(item)) => {
                    *item_slot = item;
                    filled += 1;
                }
            }
        }
        Poll::Ready(Ok(filled))
    }
}
