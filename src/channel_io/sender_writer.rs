use crate::io::AsyncWriteItems;
use futures::channel::mpsc::{SendError, Sender, TrySendError};
use std::io::Result;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct SenderWriter<T> {
    pub(super) sender: Sender<T>,
}

impl<T: Unpin + Copy> AsyncWriteItems<T> for SenderWriter<T> {
    fn poll_write_items(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        items: &[T],
    ) -> Poll<Result<usize>> {
        let mut items_iter = items.iter();

        let first_item = match items_iter.next() {
            None => return Poll::Ready(Ok(0)),
            Some(item) => item,
        };

        match Pin::new(&mut self.sender).poll_ready(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Ok(())) => {}
            Poll::Ready(Err(SendError { .. })) => return Poll::Ready(Ok(0)),
        };

        match self.sender.start_send(*first_item) {
            Ok(()) => {}
            Err(err @ SendError { .. }) => {
                assert!(!err.is_full());
                if err.is_disconnected() {
                    return Poll::Ready(Ok(0));
                }
                unreachable!("unexpected error state");
            }
        }

        let mut filled: usize = 1;
        for item in items_iter {
            match self.sender.try_send(*item) {
                Ok(()) => {}
                Err(err @ TrySendError { .. }) => {
                    if err.is_full() || err.is_disconnected() {
                        break;
                    }
                    unreachable!("unexpected error state");
                }
            }
            filled += 1;
        }
        Poll::Ready(Ok(filled))
    }
}
