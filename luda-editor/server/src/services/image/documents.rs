use crate::storage::dynamo_db::Document;
use rpc::Uuid;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectImageDocument {
    pub project_id: rpc::Uuid,
    pub image_id: rpc::Uuid,
    pub labels: Vec<rpc::data::Label>,
}

impl Document for ProjectImageDocument {
    fn partition_key_prefix() -> &'static str {
        "project_image"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.project_id.to_string()
    }

    fn sort_key(&self) -> Option<String> {
        Some(self.image_id.to_string())
    }
}

impl ProjectImageDocument {
    fn s3_key(&self) -> String {
        image_s3_key(self.project_id, self.image_id)
    }
    pub fn get_image_url(&self) -> String {
        let s3_key = self.s3_key();
        crate::s3().get_url(s3_key)
    }

    pub async fn request_put_presigned_url(
        &self,
    ) -> Result<String, crate::storage::s3::RequestPresignedUrlError> {
        let s3_key = self.s3_key();
        crate::s3()
            .request_presigned_url(s3_key, crate::storage::s3::PresignedMethod::Put)
            .await
    }
}

pub fn image_s3_key(project_id: Uuid, image_id: Uuid) -> String {
    crate::append_slash!["projects", project_id, "images", image_id,]
}
