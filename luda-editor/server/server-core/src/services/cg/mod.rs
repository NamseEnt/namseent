pub mod documents;
mod parse_psd_to_inter_cg_parts;
mod psd_to_cg_file;

use self::{
    documents::{CgDocument, CgInProject, CgInProjectQuery},
    psd_to_cg_file::{psd_to_webps_and_cg_file, PsdParsingResult},
};
use crate::{services::cg::documents::CgDocumentGet, session::SessionDocument};
use futures::FutureExt;

#[derive(Debug)]
pub struct CgService {}

impl CgService {
    pub fn new() -> Self {
        CgService {}
    }
}

impl rpc::CgService<SessionDocument> for CgService {
    fn request_put_psd_presigned_url<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::request_put_psd_presigned_url::Request {
            project_id,
            psd_file_name,
            psd_file_size,
        }: rpc::request_put_psd_presigned_url::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<Output = rpc::request_put_psd_presigned_url::Result>
                + Send,
        >,
    > {
        Box::pin(async move {
            crate::services()
                .project_service
                .check_session_project_editor(
                    session,
                    project_id,
                    || rpc::request_put_psd_presigned_url::Error::Unauthorized,
                    |err| rpc::request_put_psd_presigned_url::Error::Unknown(err),
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
                .map_err(|err| {
                    rpc::request_put_psd_presigned_url::Error::Unknown(err.to_string())
                })?;

            Ok(rpc::request_put_psd_presigned_url::Response {
                presigned_url,
                psd_id,
            })
        })
    }

    fn complete_put_psd<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::complete_put_psd::Request {
            project_id,
            psd_file_name,
            psd_id,
        }: rpc::complete_put_psd::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::complete_put_psd::Result> + Send>,
    > {
        Box::pin(async move {
            crate::services()
                .project_service
                .check_session_project_editor(
                    session,
                    project_id,
                    || rpc::complete_put_psd::Error::Unauthorized,
                    |err| rpc::complete_put_psd::Error::Unknown(err),
                )
                .await?;

            if namui_type::uuid_from_hash(&psd_file_name) != psd_id {
                return Err(rpc::complete_put_psd::Error::WrongPsdFileName);
            };

            let psd_s3_key = format!("{project_id}/psd/{psd_id}");

            let psd_bytes = crate::s3()
                .get_object(psd_s3_key)
                .await
                .map_err(|err| match err {
                    crate::storage::s3::GetObjectError::NotFound => {
                        rpc::complete_put_psd::Error::PsdFileNotFound
                    }
                    crate::storage::s3::GetObjectError::Unknown(err) => {
                        rpc::complete_put_psd::Error::Unknown(err.to_string())
                    }
                })?;

            let PsdParsingResult {
                variants_webps,
                cg_file,
                cg_thumbnail_webp,
            } = psd_to_webps_and_cg_file(&psd_bytes, &psd_file_name)
                .map_err(|e| rpc::complete_put_psd::Error::WrongPsdFile(e.to_string()))?;

            let cg_file_id = cg_file.id;

            let futures = variants_webps
                .into_iter()
                .map(|(variant_id, variant_webp_bytes)| {
                    async move {
                        rpc::utils::retry_on_error(
                            || async {
                                let cg_key =
                                    format!("{project_id}/cg/{cg_file_id}/{variant_id}.webp");

                                crate::s3()
                                    .put_object(cg_key, variant_webp_bytes.clone())
                                    .await
                                    .map_err(|err| {
                                        rpc::complete_put_psd::Error::Unknown(err.to_string())
                                    })?;

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
                                    .map_err(|err| {
                                        rpc::complete_put_psd::Error::Unknown(err.to_string())
                                    })?;

                                Ok(())
                            },
                            3,
                        )
                        .await
                    }
                    .boxed(),
                ));

            futures::future::try_join_all(futures).await?;

            crate::dynamo_db()
                .transact()
                .create_item(CgDocument {
                    id: psd_id,
                    cg_file,
                })
                .create_item(CgInProject {
                    cg_id: psd_id,
                    project_id,
                })
                .send()
                .await
                .map_err(|err| rpc::complete_put_psd::Error::Unknown(err.to_string()))?;

            Ok(rpc::complete_put_psd::Response {})
        })
    }

    fn list_cg_files<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::list_cg_files::Request { project_id }: rpc::list_cg_files::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::list_cg_files::Result> + Send>>
    {
        Box::pin(async move {
            crate::services()
                .project_service
                .check_session_project_editor(
                    session,
                    project_id,
                    || rpc::list_cg_files::Error::Unauthorized,
                    |err| rpc::list_cg_files::Error::Unknown(err),
                )
                .await?;

            let futures = CgInProjectQuery {
                pk_project_id: project_id,
                last_sk: None, // TODO
            }
            .run()
            .await
            .map_err(|err| rpc::list_cg_files::Error::Unknown(err.to_string()))?
            .documents
            .into_iter()
            .map(
                |CgInProject {
                     project_id: _,
                     cg_id,
                 }| { async move { CgDocumentGet { pk_id: cg_id }.run().await } },
            );

            let cg_files = futures::future::try_join_all(futures)
                .await
                .map_err(|err| rpc::list_cg_files::Error::Unknown(err.to_string()))?
                .into_iter()
                .map(|cg_document| cg_document.cg_file)
                .collect::<Vec<_>>();

            Ok(rpc::list_cg_files::Response { cg_files })
        })
    }
}
