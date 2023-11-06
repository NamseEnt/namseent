#[document_macro::document]
pub struct ProjectImageDocument {
    #[pk]
    pub project_id: rpc::Uuid,
    #[sk]
    pub image_id: rpc::Uuid,
    pub labels: Vec<rpc::data::Label>,
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
            .request_put_presigned_url(
                s3_key,
                crate::storage::s3::PutPresignedUrlOptions {
                    expires_in: std::time::Duration::from_secs(60),
                    content_type: None,
                    content_length: None,
                },
            )
            .await
    }
}

pub fn image_s3_key(project_id: rpc::Uuid, image_id: rpc::Uuid) -> String {
    crate::append_slash!["projects", project_id, "images", image_id,]
}
