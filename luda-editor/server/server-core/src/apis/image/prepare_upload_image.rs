use crate::documents::*;
use rpc::prepare_upload_image::{Error, Request, Response};

pub async fn prepare_upload_image(
    session: Option<SessionDocument>,
    rpc::prepare_upload_image::Request {
        project_id,
        image_id,
    }: rpc::prepare_upload_image::Request,
) -> rpc::prepare_upload_image::Result {
    if session.is_none() {
        return Err(rpc::prepare_upload_image::Error::Unauthorized);
    }
    let session = session.unwrap();
    let is_project_editor =
        crate::apis::project::shared::is_project_editor(session.user_id, project_id)
            .await
            .map_err(|error| rpc::prepare_upload_image::Error::Unknown(error.to_string()))?;

    if !is_project_editor {
        return Err(rpc::prepare_upload_image::Error::Unauthorized);
    }
    let document = ProjectImageDocumentGet {
        pk_project_id: project_id,
        sk_image_id: image_id,
    }
    .run()
    .await
    .map_err(|error| rpc::prepare_upload_image::Error::Unknown(error.to_string()))?;

    let upload_url = document
        .request_put_presigned_url()
        .await
        .map_err(|error| rpc::prepare_upload_image::Error::Unknown(error.to_string()))?;

    Ok(rpc::prepare_upload_image::Response { upload_url })
}
