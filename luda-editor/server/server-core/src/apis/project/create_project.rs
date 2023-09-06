use crate::documents::*;
use rpc::create_project::{Error, Request, Response};

pub async fn create_project(
    session: Option<SessionDocument>,
    Request { name }: Request,
) -> rpc::create_project::Result {
    // TODO: Prevent duplicate project names.
    if session.is_none() {
        return Err(Error::Unauthorized);
    }
    let session = session.unwrap();

    let project_id = rpc::Uuid::new_v4();
    let owner_id = session.user_id;

    let project_document = ProjectDocument {
        id: project_id,
        owner_id,
        name,
        shared_data_json: serde_json::to_string(&rpc::data::ProjectSharedData::new(project_id))
            .unwrap(),
    };

    let owner_project_document = OwnerProjectDocument {
        owner_id,
        project_id,
    };

    let result = crate::dynamo_db()
        .transact()
        .create_item(project_document)
        .create_item(owner_project_document)
        .send()
        .await;
    match result {
        Ok(_) => Ok(Response {}),
        Err(error) => Err(Error::Unknown(error.to_string())),
    }
}
