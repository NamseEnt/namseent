pub mod documents;

use super::{auth::documents::UserDocumentGet, sequence::documents::SequenceIndexDocumentGet};
use crate::session::SessionDocument;
use documents::*;

#[derive(Debug)]
pub struct MemoService {}

impl MemoService {
    pub fn new() -> Self {
        MemoService {}
    }
}

impl rpc::MemoService<SessionDocument> for MemoService {
    fn list_sequence_memos<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        req: rpc::list_sequence_memos::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::list_sequence_memos::Result> + Send>,
    > {
        Box::pin(async move {})
    }

    fn create_memo<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::create_memo::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::create_memo::Result> + Send>>
    {
        Box::pin(async move {})
    }

    fn delete_memo<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::delete_memo::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::delete_memo::Result> + Send>>
    {
        Box::pin(async move {})
    }
}
