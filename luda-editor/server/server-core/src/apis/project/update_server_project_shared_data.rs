use super::shared::*;
use crate::documents::*;
use rpc::update_server_project_shared_data::{Error, Request, Response};

pub async fn update_server_project_shared_data(
    session: Option<SessionDocument>,
    Request { project_id, patch }: Request,
) -> rpc::update_server_project_shared_data::Result {
    if session.is_none() {
        return Err(Error::Unauthorized);
    }
    let session = session.unwrap();

    let is_project_editor = is_project_editor(session.user_id, project_id)
        .await
        .map_err(|error| Error::Unknown(error.to_string()))?;

    if !is_project_editor {
        return Err(Error::Unauthorized);
    }

    ProjectDocumentUpdate {
        pk_id: project_id,
        update: move |mut project: ProjectDocument| async move {
            let mut project_shared_data_json =
                serde_json::from_str::<serde_json::Value>(&project.shared_data_json)
                    .map_err(|err| Error::Unknown(err.to_string()))?;
            rpc::json_patch::patch(&mut project_shared_data_json, &patch)
                .map_err(|err| Error::Unknown(err.to_string()))?;

            project.shared_data_json = serde_json::to_string(&project_shared_data_json)
                .map_err(|err| Error::Unknown(err.to_string()))?;
            Ok(project)
        },
    }
    .run()
    .await
    .map_err(|error| match error {
        crate::storage::dynamo_db::UpdateItemError::Canceled(error) => error,
        crate::storage::dynamo_db::UpdateItemError::NotFound
        | crate::storage::dynamo_db::UpdateItemError::SerializationFailed(_)
        | crate::storage::dynamo_db::UpdateItemError::Conflict
        | crate::storage::dynamo_db::UpdateItemError::Unknown(_) => {
            Error::Unknown(error.to_string())
        }
    })?;

    Ok(Response {})
}
