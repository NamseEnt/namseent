use crate::documents::*;
use rpc::update_client_project_shared_data::{Error, Request, Response};

pub async fn update_client_project_shared_data(
    _session: Option<SessionDocument>,
    Request {
        project_id,
        project_shared_data_json: _,
    }: Request,
) -> rpc::update_client_project_shared_data::Result {
    let project = ProjectDocumentGet { pk_id: project_id }
        .run()
        .await
        .map_err(|error| Error::Unknown(error.to_string()))?;

    let project_shared_data_json =
        serde_json::from_str::<serde_json::Value>(&project.shared_data_json)
            .map_err(|err| Error::Unknown(err.to_string()))?;
    let patch = rpc::json_patch::diff(&project_shared_data_json, &project_shared_data_json);

    Ok(Response { patch })
}
