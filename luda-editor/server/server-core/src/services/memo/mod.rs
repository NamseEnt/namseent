pub mod documents;

use super::{auth::documents::UserDocumentGet, sequence::documents::SequenceDocumentGet};
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
        Box::pin(async move {
            let memo_documents = MemoDocumentQuery {
                pk_sequence_id: req.sequence_id,
                last_sk: None, // TODO
            }
            .run()
            .await
            .map_err(|error| rpc::list_sequence_memos::Error::Unknown(error.to_string()))?;

            let memos = memo_documents
                .into_iter()
                .map(|memo_document| memo_document.into())
                .collect();

            Ok(rpc::list_sequence_memos::Response { memos })
        })
    }

    fn create_memo<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::create_memo::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::create_memo::Result> + Send>>
    {
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::create_memo::Error::Unauthorized);
            }
            let session = session.unwrap();

            let sequence_document = SequenceDocumentGet {
                pk_id: req.sequence_id,
            }
            .run()
            .await
            .map_err(|error| rpc::create_memo::Error::Unknown(error.to_string()))?;

            let is_project_editor = crate::services()
                .project_service
                .is_project_editor(session.user_id, sequence_document.project_id)
                .await
                .map_err(|error| rpc::create_memo::Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(rpc::create_memo::Error::Unauthorized);
            }

            let user_document = UserDocumentGet {
                pk_id: session.user_id,
            }
            .run()
            .await
            .map_err(|error| rpc::create_memo::Error::Unknown(error.to_string()))?;

            let memo_document = MemoDocument {
                sequence_id: sequence_document.id,
                cut_id: req.cut_id,
                memo_id: rpc::Uuid::new_v4(),
                content: req.content,
                user_id: session.user_id,
                user_name: user_document.name,
            };

            crate::dynamo_db()
                .create_item(memo_document.clone())
                .await
                .map_err(|error| rpc::create_memo::Error::Unknown(error.to_string()))?;

            Ok(rpc::create_memo::Response {
                memo: memo_document.into(),
            })
        })
    }

    fn delete_memo<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::delete_memo::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::delete_memo::Result> + Send>>
    {
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::delete_memo::Error::Unauthorized);
            }
            let session = session.unwrap();

            let sequence_document = SequenceDocumentGet {
                pk_id: req.sequence_id,
            }
            .run()
            .await
            .map_err(|error| rpc::delete_memo::Error::Unknown(error.to_string()))?;

            let is_project_editor = crate::services()
                .project_service
                .is_project_editor(session.user_id, sequence_document.project_id)
                .await
                .map_err(|error| rpc::delete_memo::Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(rpc::delete_memo::Error::Unauthorized);
            }

            let memo_document = MemoDocumentGet {
                pk_sequence_id: sequence_document.id,
                sk_memo_id: req.memo_id,
            }
            .run()
            .await
            .map_err(|error| rpc::delete_memo::Error::Unknown(error.to_string()))?;

            if session.user_id != memo_document.user_id {
                return Err(rpc::delete_memo::Error::Forbidden);
            }

            MemoDocumentDelete {
                pk_sequence_id: sequence_document.id,
                sk_memo_id: req.memo_id,
            }
            .run()
            .await
            .map_err(|error| rpc::delete_memo::Error::Unknown(error.to_string()))?;

            Ok(rpc::delete_memo::Response {})
        })
    }
}
