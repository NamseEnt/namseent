use crate::documents::*;
use rpc::put_image_meta_data::{Error, Request, Response};

pub async fn put_image_meta_data(
    session: Option<SessionDocument>,
    rpc::put_image_meta_data::Request {
        project_id,
        image_id,
        labels,
    }: rpc::put_image_meta_data::Request,
) -> rpc::put_image_meta_data::Result {
    if session.is_none() {
        return Err(rpc::put_image_meta_data::Error::Unauthorized);
    }
    let session = session.unwrap();
    let is_project_editor =
        crate::apis::project::shared::is_project_editor(session.user_id, project_id)
            .await
            .map_err(|error| {
                println!("error on is_project_editor: {:?}", error);
                rpc::put_image_meta_data::Error::Unknown(error.to_string())
            })?;

    if !is_project_editor {
        return Err(rpc::put_image_meta_data::Error::Unauthorized);
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
            rpc::put_image_meta_data::Error::Unknown(error.to_string())
        })?;
    Ok(rpc::put_image_meta_data::Response {})
}
