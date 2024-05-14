mod heap_archived;
mod in_memory;
mod sqlite;

use anyhow::Result;
pub use heap_archived::*;
pub use in_memory::*;
use rkyv::ser::serializers::AllocSerializer;
pub use sqlite::*;
use std::future::Future;

pub trait KvStore {
    fn get<T: rkyv::Archive>(&self, key: impl AsRef<str>) -> Result<Option<HeapArchived<T>>>;
    fn put<T: rkyv::Serialize<AllocSerializer<0>>>(
        &self,
        key: impl AsRef<str>,
        value: &T,
    ) -> Result<()>;
    fn delete(&self, key: impl AsRef<str>) -> Result<()>;
    /// optimistic locking update
    /// Return value: true if the update was successful, false if conflict
    async fn update<T, Fut>(
        &self,
        key: impl AsRef<str>,
        update: impl FnOnce(Option<HeapArchived<T>>) -> Fut,
    ) -> Result<bool>
    where
        T: rkyv::Archive + rkyv::Serialize<AllocSerializer<0>>,
        Fut: Future<Output = Option<Option<T>>>;
}

// #[document]
// struct GoogleIdentity {
//     #[pk]
//     google_id: String,
//     user_id: String,
// }

// #[document]
// struct User {
//     #[pk]
//     id: String,
//     name: String,
//     works: Many<Work>,
// }

// #[document]
// struct Work {
//     #[pk]
//     id: String,
//     owner: One<User>,
//     title: String,
// }

// struct Many<T> {
//     _phantom: std::marker::PhantomData<T>,
// }
// struct One<T> {
//     _phantom: std::marker::PhantomData<T>,
// }
