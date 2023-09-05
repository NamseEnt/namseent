use crate::documents::*;
use rpc::create_memo::{Error, Request, Response};

pub async fn create_memo(
    session: Option<SessionDocument>,
    Request {
        sequence_id,
        cut_id,
        content,
    }: Request,
) -> rpc::create_memo::Result {
    if session.is_none() {
        return Err(Error::Unauthorized);
    }
    let session = session.unwrap();

    let sequence_document = SequenceIndexDocumentGet { pk_id: sequence_id }
        .run()
        .await
        .map_err(|error| Error::Unknown(error.to_string()))?;

    let is_project_editor = crate::apis::project::shared::is_project_editor(
        session.user_id,
        sequence_document.project_id,
    )
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    if !is_project_editor {
        return Err(Error::Unauthorized);
    }

    let user_document = UserDocumentGet {
        pk_id: session.user_id,
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

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
        .map_err(|error| Error::Unknown(error.to_string()))?;

    Ok(Response {
        memo: memo_document.into(),
    })
}
