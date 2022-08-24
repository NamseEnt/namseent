mod github_storage;
mod server_storage;

pub use github_storage::*;
pub use server_storage::*;
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

#[derive(Clone, Copy)]
pub enum SyncStatus {
    Idle,
    Sending(namui::Time),
    Sent(namui::Time),
}

pub trait Storage<T: crdt::History>: Send + Sync {
    fn get<'a>(
        &'a self,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<crdt::HistorySystem<T>, GetError>>>>;
    fn start_sync<'a>(
        &'a self,
        update_queue: Arc<Mutex<Vec<yrs::Update>>>,
        sync_status: Arc<Mutex<SyncStatus>>,
    ) -> Pin<Box<dyn 'a + Future<Output = ()>>>;
    fn upload_resource<'a>(
        &'a self,
        path: String,
        data: &'a [u8],
    ) -> Pin<Box<dyn 'a + Future<Output = Result<(), UploadResourceError>>>>;
    fn list_resources<'a>(
        &'a self,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<Box<[String]>, ListResourceError>>>>;
    fn get_resource<'a>(
        &'a self,
        path: String,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<Box<[u8]>, GetResourceError>>>>;
}

#[derive(Debug)]
pub enum GetError {
    NotExists,
    Unknown(Box<dyn std::error::Error>),
}
namui::simple_error_impl!(GetError);

#[derive(Debug)]
pub enum UploadResourceError {
    Conflict,
    Unknown(Box<dyn std::error::Error>),
}
namui::simple_error_impl!(UploadResourceError);

#[derive(Debug)]
pub enum ListResourceError {
    Unknown(Box<dyn std::error::Error>),
}
namui::simple_error_impl!(ListResourceError);

#[derive(Debug)]
pub enum GetResourceError {
    NotExists,
    Unknown(Box<dyn std::error::Error>),
}
namui::simple_error_impl!(GetResourceError);
