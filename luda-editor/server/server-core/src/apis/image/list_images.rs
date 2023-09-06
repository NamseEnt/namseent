use crate::documents::*;
use rpc::data::ImageWithLabels;
use rpc::list_images::{Error, Request, Response};

pub async fn list_images(
    _session: Option<SessionDocument>,
    Request { project_id }: Request,
) -> rpc::list_images::Result {
    let query = ProjectImageDocumentQuery {
        pk_project_id: project_id,
        last_sk: None, // TODO
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    Ok(Response {
        images: query
            .documents
            .into_iter()
            .map(|document| ImageWithLabels {
                id: document.image_id,
                url: document.get_image_url(),
                labels: document.labels,
            })
            .collect(),
    })
}
