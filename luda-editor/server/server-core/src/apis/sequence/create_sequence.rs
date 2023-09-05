use crate::documents::*;
use rpc::create_sequence::{Error, Request, Response};

pub async fn create_sequence(
    session: Option<SessionDocument>,
    Request { project_id, name }: Request,
) -> rpc::create_sequence::Result {
    if session.is_none() {
        return Err(Error::Unauthorized);
    }
    let session = session.unwrap();
    let is_project_editor =
        crate::apis::project::shared::is_project_editor(session.user_id, project_id)
            .await
            .map_err(|error| Error::Unknown(error.to_string()))?;

    if !is_project_editor {
        return Err(Error::Unauthorized);
    }

    let sequence_id = rpc::Uuid::new_v4();

    crate::dynamo_db()
        .transact()
        .create_item(SequenceIndexDocument {
            id: sequence_id,
            project_id,
            index: CircularIndex::new(),
            undoable_count: BoundedUsize::new(),
            redoable_count: BoundedUsize::new(),
        })
        .create_item(SequenceDocument {
            id: sequence_id,
            index: CircularIndex::new(),
            project_id,
            name,
            cuts: vec![],
        })
        .create_item(ProjectSequenceDocument {
            project_id,
            sequence_id,
        })
        .send()
        .await
        .map_err(|error| Error::Unknown(error.to_string()))?;

    Ok(Response {})
}
