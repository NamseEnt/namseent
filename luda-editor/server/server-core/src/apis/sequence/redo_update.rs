use crate::documents::*;
use rpc::redo_update::{Error, Request, Response};

pub async fn redo_update(
    session: Option<SessionDocument>,
    Request { sequence_id }: Request,
) -> rpc::redo_update::Result {
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

            if *document.redoable_count == 0 {
                return Err(Error::NoMoreRedo);
            }
            document.redoable_count.decrease();
            document.undoable_count.increase();
            document.index.increase();

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
