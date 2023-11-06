use crate::documents::*;
use rpc::delete_sequence::{Error, Request, Response};

pub async fn delete_sequence(
    session: Option<SessionDocument>,
    Request { sequence_id }: Request,
) -> rpc::delete_sequence::Result {
    let Some(session) = session else {
        return Err(Error::Unauthorized);
    };

    // TODO: Remove all using queue

    let sequence_index_document = SequenceIndexDocumentGet { pk_id: sequence_id }
        .run()
        .await
        .map_err(|error| match error {
            crate::storage::dynamo_db::GetItemError::NotFound => Error::SequenceNotFound,
            _ => Error::Unknown(error.to_string()),
        })?;
    let is_project_editor = crate::apis::project::shared::is_project_editor(
        session.user_id,
        sequence_index_document.project_id,
    )
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    if !is_project_editor {
        return Err(Error::Unauthorized);
    }

    crate::dynamo_db()
        .transact()
        .delete_item(SequenceIndexDocumentDelete { pk_id: sequence_id })
        .delete_item(ProjectSequenceDocumentDelete {
            pk_project_id: sequence_index_document.project_id,
            sk_sequence_id: sequence_id,
        })
        .send()
        .await
        .map_err(|error| Error::Unknown(error.to_string()))?;

    Ok(Response {})
}
