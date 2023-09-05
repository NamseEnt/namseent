use crate::documents::*;
use rpc::delete_memo::{Error, Request, Response};

pub async fn delete_memo(
    session: Option<SessionDocument>,
    Request {
        sequence_id,
        memo_id,
    }: Request,
) -> rpc::delete_memo::Result {
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

    let memo_document = MemoDocumentGet {
        pk_sequence_id: sequence_document.id,
        sk_memo_id: memo_id,
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    if session.user_id != memo_document.user_id {
        return Err(Error::Forbidden);
    }

    MemoDocumentDelete {
        pk_sequence_id: sequence_document.id,
        sk_memo_id: memo_id,
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    Ok(Response {})
}
