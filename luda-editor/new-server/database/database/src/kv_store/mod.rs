mod heap_archived;
mod in_memory;
mod sqlite;

use anyhow::Result;
pub(crate) use heap_archived::*;
pub(crate) use in_memory::*;
pub(crate) use sqlite::*;
use std::sync::Arc;

pub(crate) trait KvStore {
    async fn get(&self, key: impl AsRef<str>) -> Result<Option<ValueBuffer>>;
    async fn put(&self, key: impl AsRef<str>, value: &impl AsRef<[u8]>) -> Result<()>;
    async fn delete(&self, key: impl AsRef<str>) -> Result<()>;
    // /// optimistic locking update
    // /// Return value: true if the update was successful, false if conflict
    // async fn update<T, Fut>(
    //     &self,
    //     key: impl AsRef<str>,
    //     update: impl FnOnce(Option<HeapArchived<T>>) -> Fut,
    // ) -> Result<bool>
    // where
    //     T: rkyv::Archive + rkyv::Serialize<AllocSerializer<64>>,
    //     Fut: Future<Output = Option<Option<T>>>;
    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        key: impl AsRef<str>,
        value_fn: impl FnOnce() -> Result<Bytes>,
    ) -> Result<()>;
}
pub enum ValueBuffer {
    Vec(Vec<u8>),
    Arc(Arc<Vec<u8>>),
}
impl ValueBuffer {
    pub fn get_arc_vec(&self) -> Arc<Vec<u8>> {
        match self {
            Self::Vec(vec) => Arc::new(vec.clone()),
            Self::Arc(arc) => arc.clone(),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::Vec(vec) => vec.as_slice(),
            Self::Arc(arc) => arc.as_slice(),
        }
    }
}
impl From<Vec<u8>> for ValueBuffer {
    fn from(vec: Vec<u8>) -> Self {
        Self::Vec(vec)
    }
}
impl From<Arc<Vec<u8>>> for ValueBuffer {
    fn from(arc: Arc<Vec<u8>>) -> Self {
        Self::Arc(arc)
    }
}
