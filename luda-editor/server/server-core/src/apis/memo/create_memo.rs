use super::shared::*;
use crate::documents::*;
use rpc::create_memo::{Error, Request, Response};

pub async fn create_memo(
    session: Option<SessionDocument>,
    rpc::create_memo::Request {
        sequence_id,
        cut_id,
        content,
    }: rpc::create_memo::Request,
) -> rpc::create_memo::Result {
    if session.is_none() {
        return Err(rpc::create_memo::Error::Unauthorized);
    }
    let session = session.unwrap();

    let sequence_document = SequenceIndexDocumentGet { pk_id: sequence_id }
        .run()
        .await
        .map_err(|error| rpc::create_memo::Error::Unknown(error.to_string()))?;

    let is_project_editor = crate::apis::project::shared::is_project_editor(
        session.user_id,
        sequence_document.project_id,
    )
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
        cut_id: cut_id,
        memo_id: rpc::Uuid::new_v4(),
        content: content,
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
}
