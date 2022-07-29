mod editor_history_system;
mod system_tree_0;
mod system_tree_1;
mod system_tree_2;
mod system_tree_3;
mod system_tree_4;
mod system_tree_5;
pub mod system_tree_6;

use editor_core::storage::SyncStatus;
pub use editor_history_system::*;
use std::sync::{Arc, Mutex};
pub use system_tree_6 as system_tree;
pub use system_tree_6::*;

#[derive(Clone)]
pub struct Storage(Arc<dyn editor_core::storage::Storage<system_tree::SystemTree>>);

impl Storage {
    pub fn new(storage: Arc<dyn editor_core::storage::Storage<system_tree::SystemTree>>) -> Self {
        Self(storage)
    }
    pub fn get<'a>(
        &'a self,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<
                    Output = Result<
                        crdt::HistorySystem<system_tree::SystemTree>,
                        editor_core::storage::GetError,
                    >,
                >,
        >,
    > {
        self.0.get()
    }

    pub fn start_sync<'a>(
        &'a self,
        update_queue: Arc<Mutex<Vec<crdt::yrs::Update>>>,
        update_sync_status: Arc<Mutex<SyncStatus>>,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = ()>>> {
        self.0.start_sync(update_queue, update_sync_status)
    }

    pub fn upload_resource<'a>(
        &'a self,
        path: impl AsRef<str>,
        data: &'a [u8],
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<Output = Result<(), editor_core::storage::UploadResourceError>>,
        >,
    > {
        self.0.upload_resource(path.as_ref().to_string(), data)
    }

    pub fn list_resources<'a>(
        &'a self,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<
                    Output = Result<Box<[String]>, editor_core::storage::ListResourceError>,
                >,
        >,
    > {
        self.0.list_resources()
    }

    pub fn get_resource<'a>(
        &'a self,
        path: impl AsRef<str>,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<
                    Output = Result<Box<[u8]>, editor_core::storage::GetResourceError>,
                >,
        >,
    > {
        self.0.get_resource(path.as_ref().to_string())
    }
}

impl std::fmt::Debug for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Storage")
    }
}
