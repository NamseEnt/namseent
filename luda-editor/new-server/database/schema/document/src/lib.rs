mod heap_archived;
mod transact;

pub use bytes::Bytes;
pub use heap_archived::*;
pub use inventory;
pub use schema_macro::*;
pub use serializer;
pub use serializer::*;
pub use transact::*;

pub trait Document {
    fn name() -> &'static str;
    fn heap_archived(bytes: Bytes) -> HeapArchived<Self>
    where
        Self: Sized;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self>
    where
        Self: Sized;
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

pub trait DocumentGet {
    type Output;

    fn pk(&self) -> Result<u128>;
    fn sk(&self) -> Result<Option<u128>>;
}

pub trait DocumentQuery {
    type Output;

    fn pk(&self) -> Result<u128>;
}

pub struct DocumentLogPlugin {
    pub name: &'static str,
    pub debug_log_value: fn(&[u8]),
}

impl DocumentLogPlugin {
    pub const fn new(name: &'static str, debug_log_value: fn(&[u8])) -> Self {
        DocumentLogPlugin {
            name,
            debug_log_value,
        }
    }
}

inventory::collect!(DocumentLogPlugin);
