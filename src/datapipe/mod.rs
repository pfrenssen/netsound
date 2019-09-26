use crate::io::{AsyncItemsAvailable, AsyncReadItems, AsyncWriteItems};

pub trait CompatibleItem: Unpin + Send + Copy {}
impl<T> CompatibleItem for T where T: Unpin + Send + Copy {}

pub trait Reader:
    AsyncReadItems<<Self as Reader>::Item> + AsyncItemsAvailable<<Self as Reader>::Item>
{
    type Item: CompatibleItem;
}

pub trait Writer: AsyncWriteItems<<Self as Writer>::Item> {
    type Item: CompatibleItem;
}
