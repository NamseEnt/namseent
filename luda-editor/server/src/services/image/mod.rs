mod documents;

use crate::session::SessionDocument;
use documents::ProjectImageDocument;
use rpc::data::ImageWithLabels;

#[derive(Debug)]
pub struct ImageService {}

impl ImageService {
    pub fn new() -> Self {
        ImageService {}
    }
}

impl rpc::ImageService<SessionDocument> for ImageService {
    fn put_image_meta_data<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::put_image_meta_data::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::put_image_meta_data::Result> + Send>,
    > {
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::put_image_meta_data::Error::Unauthorized);
            }
            let session = session.unwrap();
            let is_project_editor = crate::services()
                .project_service
                .is_project_editor(session.user_id, req.project_id)
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
                    project_id: req.project_id,
                    image_id: req.image_id,
                    labels: req.labels,
                })
                .await
                .map_err(|error| {
                    println!("error on put_item: {:?}", error);
                    rpc::put_image_meta_data::Error::Unknown(error.to_string())
                })?;
            Ok(rpc::put_image_meta_data::Response {})
        })
    }

    fn prepare_upload_image<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::prepare_upload_image::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::prepare_upload_image::Result> + Send>,
    > {
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::prepare_upload_image::Error::Unauthorized);
            }
            let session = session.unwrap();
            let is_project_editor = crate::services()
                .project_service
                .is_project_editor(session.user_id, req.project_id)
                .await
                .map_err(|error| rpc::prepare_upload_image::Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(rpc::prepare_upload_image::Error::Unauthorized);
            }
            let document = crate::dynamo_db()
                .get_item::<ProjectImageDocument>(req.project_id, Some(req.image_id))
                .await
                .map_err(|error| rpc::prepare_upload_image::Error::Unknown(error.to_string()))?;

            let upload_url = document
                .request_put_presigned_url()
                .await
                .map_err(|error| rpc::prepare_upload_image::Error::Unknown(error.to_string()))?;

            Ok(rpc::prepare_upload_image::Response { upload_url })
        })
    }

    fn list_images<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        req: rpc::list_images::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::list_images::Result> + Send>>
    {
        Box::pin(async move {
            let documents = crate::dynamo_db()
                .query::<ProjectImageDocument>(req.project_id)
                .await
                .map_err(|error| rpc::list_images::Error::Unknown(error.to_string()))?;

            Ok(rpc::list_images::Response {
                images: documents
                    .into_iter()
                    .map(|document| ImageWithLabels {
                        id: document.image_id,
                        url: document.get_image_url(),
                        labels: document.labels,
                    })
                    .collect(),
            })
        })
    }
}
