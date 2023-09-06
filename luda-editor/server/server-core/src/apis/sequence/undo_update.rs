use crate::documents::*;
use rpc::undo_update::{Error, Request, Response};

pub async fn undo_update(
    session: Option<SessionDocument>,
    Request { sequence_id }: Request,
) -> rpc::undo_update::Result {
    let Some(session) = session else {
        return Err(Error::Unauthorized);
    };

    SequenceIndexDocumentUpdate {
        pk_id: sequence_id,
        update: move |mut document| async move {
            let is_project_editor = crate::apis::project::shared::is_project_editor(
                session.user_id,
                document.project_id,
            )
            .await
            .map_err(|error| Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(Error::Forbidden);
            }

            if *document.undoable_count == 0 {
                return Err(Error::NoMoreUndo);
            }
            document.undoable_count.decrease();
            document.redoable_count.increase();
            document.index.decrease();

            Ok(document)
        },
    }
    .run()
    .await
    .map_err(|error| match error {
        crate::storage::dynamo_db::UpdateItemError::NotFound => Error::NotFound,
        _ => Error::Unknown(error.to_string()),
    })?;

    Ok(Response {})
}
