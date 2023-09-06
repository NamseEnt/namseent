use super::shared::psd_to_cg_file::{psd_to_webps_and_cg_file, PsdParsingResult};
use crate::documents::*;
use futures::FutureExt;
use rpc::complete_put_psd::{Error, Request, Response};

pub async fn complete_put_psd(
    session: Option<SessionDocument>,
    Request {
        project_id,
        psd_file_name,
        psd_id,
    }: Request,
) -> rpc::complete_put_psd::Result {
    crate::apis::project::shared::check_session_project_editor(
        session,
        project_id,
        || Error::Unauthorized,
        Error::Unknown,
    )
    .await?;

    if namui_type::uuid_from_hash(&psd_file_name) != psd_id {
        return Err(Error::WrongPsdFileName);
    };

    let psd_s3_key = format!("{project_id}/psd/{psd_id}");

    let psd_bytes = crate::s3()
        .get_object(psd_s3_key)
        .await
        .map_err(|err| match err {
            crate::storage::s3::GetObjectError::NotFound => Error::PsdFileNotFound,
            crate::storage::s3::GetObjectError::Unknown(err) => Error::Unknown(err.to_string()),
        })?;

    let PsdParsingResult {
        variants_webps,
        cg_file,
        cg_thumbnail_webp,
    } = psd_to_webps_and_cg_file(&psd_bytes, &psd_file_name)
        .map_err(|e| Error::WrongPsdFile(e.to_string()))?;

    let cg_file_id = cg_file.id;

    let futures = variants_webps
        .into_iter()
        .map(|(variant_id, variant_webp_bytes)| {
            async move {
                rpc::utils::retry_on_error(
                    || async {
                        let cg_key = format!("{project_id}/cg/{cg_file_id}/{variant_id}.webp");

                        crate::s3()
                            .put_object(cg_key, variant_webp_bytes.clone())
                            .await
                            .map_err(|err| Error::Unknown(err.to_string()))?;

                        Ok(())
                    },
                    3,
                )
                .await
            }
            .boxed()
        })
        .chain(std::iter::once(
            async move {
                rpc::utils::retry_on_error(
                    || async {
                        let cg_key = format!("{project_id}/cg/{cg_file_id}/thumbnail.webp");

                        crate::s3()
                            .put_object(cg_key, cg_thumbnail_webp.clone())
                            .await
                            .map_err(|err| Error::Unknown(err.to_string()))?;

                        Ok(())
                    },
                    3,
                )
                .await
            }
            .boxed(),
        ));

    futures::future::try_join_all(futures).await?;

    let cg_id = psd_id;
    crate::dynamo_db()
        .put_item(CgDocument {
            project_id,
            cg_id,
            cg_file,
        })
        .await
        .map_err(|err| Error::Unknown(err.to_string()))?;

    Ok(Response { cg_id })
}
