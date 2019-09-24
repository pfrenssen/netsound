use super::*;

mod write_items;
pub use write_items::*;

mod read_items;
pub use read_items::*;

mod items_available;
pub use items_available::*;

pub trait AsyncWriteItemsExt<T: Unpin>: AsyncWriteItems<T> {
    fn write_items<'a>(&'a mut self, buf: &'a [T]) -> WriteItems<'a, T, Self>
    where
        Self: Unpin,
    {
        WriteItems::new(self, buf)
    }
}

impl<T: Unpin, W: AsyncWriteItems<T> + ?Sized> AsyncWriteItemsExt<T> for W {}

pub trait AsyncReadItemsExt<T: Unpin>: AsyncReadItems<T> {
    fn read_items<'a>(&'a mut self, buf: &'a mut [T]) -> ReadItems<'a, T, Self>
    where
        Self: Unpin,
    {
        ReadItems::new(self, buf)
    }
}

impl<T: Unpin, R: AsyncReadItems<T> + ?Sized> AsyncReadItemsExt<T> for R {}

pub trait AsyncItemsAvailableExt<T: Unpin>: AsyncItemsAvailable<T> {
    fn items_available<'a>(&'a mut self) -> ItemsAvailable<'a, T, Self>
    where
        Self: Unpin,
    {
        ItemsAvailable::new(self)
    }
}

impl<T: Unpin, P: AsyncItemsAvailable<T> + ?Sized> AsyncItemsAvailableExt<T> for P {}
