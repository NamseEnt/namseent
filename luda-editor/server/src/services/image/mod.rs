use crate::{session::SessionDocument, storage::s3::*};
use rpc::data::{Label, UrlWithLabels};

#[derive(Debug)]
pub struct ImageService {}

impl ImageService {
    pub fn new() -> Self {
        ImageService {}
    }
}

impl rpc::ImageService<SessionDocument> for ImageService {
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
                .is_project_editor(&session.user_id, &req.project_id)
                .await
                .map_err(|error| rpc::prepare_upload_image::Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(rpc::prepare_upload_image::Error::Unauthorized);
            }

            let key =
                label_list_to_s3_key(&req.project_id, &req.label_list).map_err(
                    |error| match error {
                        LabelError::InvalidCharacter(string) => {
                            rpc::prepare_upload_image::Error::InvalidCharacter(string)
                        }
                        LabelError::TooLong => rpc::prepare_upload_image::Error::TooLong,
                    },
                )?;

            let url = crate::s3()
                .request_presigned_url(key, PresignedMethod::Put)
                .await
                .map_err(|error| rpc::prepare_upload_image::Error::Unknown(error.to_string()))?;

            Ok(rpc::prepare_upload_image::Response { upload_url: url })
        })
    }

    fn list_images<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        req: rpc::list_images::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::list_images::Result> + Send>>
    {
        Box::pin(async move {
            let prefix = images_prefix(&req.project_id);

            let objects = crate::s3()
                .list_objects(&prefix, None)
                .await
                .map_err(|error| rpc::list_images::Error::Unknown(error.to_string()))?;

            Ok(rpc::list_images::Response {
                images: objects
                    .into_iter()
                    .map(|object| UrlWithLabels {
                        url: object.url,
                        labels: s3_key_to_label_list(&object.key, &prefix),
                    })
                    .collect(),
            })
        })
    }
}

fn s3_key_to_label_list(key: &str, prefix: &str) -> Vec<Label> {
    let key_without_prefix = key.strip_prefix(prefix).unwrap();
    key_without_prefix
        .split('/')
        .map(|label_string| {
            let mut splitted = label_string.split(LABEL_CONNECTOR);
            let key = splitted.next().unwrap().to_string();
            let value = splitted.next().unwrap().to_string();

            Label { key, value }
        })
        .collect()
}

fn images_prefix(project_id: &str) -> String {
    format!("projects/{}/images/", project_id)
}

fn label_list_to_s3_key(
    project_id: &String,
    label_list: &Vec<Label>,
) -> Result<String, LabelError> {
    for label in label_list {
        check_label_component_validity(&label.key)?;
        check_label_component_validity(&label.value)?;
    }

    let key = label_list
        .iter()
        .map(|label| format!("{}{LABEL_CONNECTOR}{}", label.key, label.value))
        .collect::<Vec<_>>()
        .join(LABEL_JOINER);

    // by s3 spec, the key must be less than 1024 bytes
    if key.len() > 1024 {
        return Err(LabelError::TooLong);
    }

    Ok(format!("{prefix}{key}", prefix = images_prefix(project_id)))
}

const LABEL_CONNECTOR: &str = ":";
const LABEL_JOINER: &str = "∷";

fn check_label_component_validity(component: &String) -> Result<(), LabelError> {
    // # Not available by s3
    // Backslash ("\")
    // Caret ("^")
    // Grave accent / back tick ("`")
    // 'Greater Than' symbol (">")
    // 'Less Than' symbol ("<")
    // Left curly brace ("{")
    // Right curly brace ("}")
    // Right square bracket ("]")
    // Left square bracket ("[")
    // 'Pound' character ("#")
    // Non-printable ASCII characters (128–255 decimal characters)
    // Percent character ("%")
    // Quotation marks
    // Tilde ("~")
    // Vertical bar / pipe ("|")

    // # Not available by us
    // Colon (":")
    // Proportion ("∷")

    [
        "\\", "^", "`", ">", "<", "{", "}", "]", "[", "#", "%", "\"", "~", "|", ":", "∷",
    ]
    .into_iter()
    .find(|c| component.contains(c))
    .map(|c| Err(LabelError::InvalidCharacter(c.to_string())))
    .unwrap_or(Ok(()))
}

#[derive(Debug)]
enum LabelError {
    InvalidCharacter(String),
    TooLong,
}
crate::simple_error_impl!(LabelError);
