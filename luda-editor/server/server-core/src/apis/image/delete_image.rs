use crate::documents::*;
use rpc::delete_image::{Error, Request, Response};
use rpc::utils::retry_on_error;

pub async fn delete_image(
    session: Option<SessionDocument>,
    rpc::delete_image::Request {
        project_id,
        image_id,
    }: rpc::delete_image::Request,
) -> rpc::delete_image::Result {
    if session.is_none() {
        return Err(rpc::delete_image::Error::Unauthorized);
    }
    let session = session.unwrap();
    let is_project_editor =
        crate::apis::project::shared::is_project_editor(session.user_id, project_id)
            .await
            .map_err(|error| {
                println!("error on is_project_editor: {:?}", error);
                rpc::delete_image::Error::Unknown(error.to_string())
            })?;

    if !is_project_editor {
        return Err(rpc::delete_image::Error::Unauthorized);
    }

    retry_on_error(
        || {
            ProjectImageDocumentDelete {
                pk_project_id: project_id,
                sk_image_id: image_id,
            }
            .run()
        },
        5,
    )
    .await
    .map_err(|error| {
        println!("error on delete_item: {:?}", error);
        rpc::delete_image::Error::Unknown(error.to_string())
    })?;

    retry_on_error(
        || crate::s3().delete_object(image_s3_key(project_id, image_id)),
        5,
    )
    .await
    .map_err(|error| {
        println!("error on delete_object: {:?}", error);
        rpc::delete_image::Error::Unknown(error.to_string())
    })?;

    Ok(rpc::delete_image::Response {})
}
