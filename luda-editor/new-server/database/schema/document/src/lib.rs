mod heap_archived;
mod transact;
mod value_buffer;

pub use anyhow::Result;
pub use heap_archived::*;
use std::borrow::Cow;
pub use transact::*;
pub use value_buffer::ValueBuffer;

pub trait Document {
    fn name() -> &'static str;
    fn heap_archived(value_buffer: ValueBuffer) -> HeapArchived<Self>
    where
        Self: Sized;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self>
    where
        Self: Sized;
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

pub trait DocumentGet {
    type Output;

    fn name() -> &'static str;
    fn pk(&self) -> Cow<'_, [u8]>;
    fn sk(&self) -> Option<Cow<'_, [u8]>>;
}
