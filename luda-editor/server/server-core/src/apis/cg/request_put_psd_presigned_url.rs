use crate::documents::*;
use rpc::request_put_psd_presigned_url::{Error, Request, Response};

pub async fn request_put_psd_presigned_url(
    session: Option<SessionDocument>,
    Request {
        project_id,
        psd_file_name,
        psd_file_size,
    }: Request,
) -> rpc::request_put_psd_presigned_url::Result {
    crate::apis::project::shared::check_session_project_editor(
        session,
        project_id,
        || Error::Unauthorized,
        Error::Unknown,
    )
    .await?;

    let psd_id = namui_type::uuid_from_hash(&psd_file_name);

    let psd_s3_key = format!("{project_id}/psd/{psd_id}");

    let presigned_url = crate::s3()
        .request_put_presigned_url(
            psd_s3_key,
            crate::storage::s3::PutPresignedUrlOptions {
                expires_in: std::time::Duration::from_secs(60),
                content_type: Some("image/vnd.adobe.photoshop".to_string()),
                content_length: Some(psd_file_size),
            },
        )
        .await
        .map_err(|err| Error::Unknown(err.to_string()))?;

    Ok(Response {
        presigned_url,
        psd_id,
    })
}
