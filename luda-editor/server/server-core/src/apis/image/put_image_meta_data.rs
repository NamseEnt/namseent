use crate::documents::*;
use rpc::put_image_meta_data::{Error, Request, Response};

pub async fn put_image_meta_data(
    session: Option<SessionDocument>,
    Request {
        project_id,
        image_id,
        labels,
    }: Request,
) -> rpc::put_image_meta_data::Result {
    if session.is_none() {
        return Err(Error::Unauthorized);
    }
    let session = session.unwrap();
    let is_project_editor =
        crate::apis::project::shared::is_project_editor(session.user_id, project_id)
            .await
            .map_err(|error| {
                println!("error on is_project_editor: {:?}", error);
                Error::Unknown(error.to_string())
            })?;

    if !is_project_editor {
        return Err(Error::Unauthorized);
    }

    crate::dynamo_db()
        .put_item(ProjectImageDocument {
            project_id,
            image_id,
            labels,
        })
        .await
        .map_err(|error| {
            println!("error on put_item: {:?}", error);
            Error::Unknown(error.to_string())
        })?;
    Ok(Response {})
}
