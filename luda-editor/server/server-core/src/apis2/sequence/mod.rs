pub mod documents;

use super::project::documents::*;
use crate::session::SessionDocument;
use documents::*;
use futures::future::try_join_all;

#[derive(Debug)]
pub struct SequenceService {}

impl SequenceService {
    pub fn new() -> Self {
        SequenceService {}
    }
}
impl Default for SequenceService {
    fn default() -> Self {
        Self::new()
    }
}

impl rpc::SequenceService<SessionDocument> for SequenceService {
    fn list_project_sequences<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        req: rpc::list_project_sequences::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::list_project_sequences::Result> + Send>,
    > {
        Box::pin(async move {})
    }

    fn create_sequence<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::create_sequence::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::create_sequence::Result> + Send>,
    > {
        Box::pin(async move {})
    }

    fn undo_update<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::undo_update::Request { sequence_id }: rpc::undo_update::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::undo_update::Result> + Send>>
    {
        Box::pin(async move {})
    }

    fn redo_update<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::redo_update::Request { sequence_id }: rpc::redo_update::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::redo_update::Result> + Send>>
    {
        Box::pin(async move {})
    }

    fn update_sequence<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::update_sequence::Request {
            sequence_id,
            action,
        }: rpc::update_sequence::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::update_sequence::Result> + Send>,
    > {
        Box::pin(async move {})
    }

    fn update_sequence_cut<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::update_sequence_cut::Request {
            sequence_id,
            cut_id,
            action,
        }: rpc::update_sequence_cut::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::update_sequence_cut::Result> + Send>,
    > {
        Box::pin(async move {})
    }

    fn get_sequence_and_project_shared_data<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        rpc::get_sequence_and_project_shared_data::Request { sequence_id }
        : rpc::get_sequence_and_project_shared_data::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<Output = rpc::get_sequence_and_project_shared_data::Result>
                + Send,
        >,
    > {
        Box::pin(async move {})
    }

    fn delete_sequence<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::delete_sequence::Request { sequence_id }: rpc::delete_sequence::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::delete_sequence::Result> + Send>,
    > {
        Box::pin(async move {})
    }
}
