use super::*;
use aws_sdk_s3::presigning::config::PresigningConfig;

impl S3 {
    pub async fn request_put_presigned_url(
        &self,
        key: String,
        options: PutPresignedUrlOptions,
    ) -> Result<String, RequestPresignedUrlError> {
        let presining_config = PresigningConfig::builder()
            .expires_in(options.expires_in)
            .build()
            .map_err(|error| RequestPresignedUrlError::WrongExpiresIn(error.to_string()))?;

        Ok(self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(self.key_with_prefix(&key))
            .set_content_type(options.content_type)
            .set_content_length(options.content_length.map(|length| length as i64))
            .presigned(presining_config)
            .await
            .map_err(|error| RequestPresignedUrlError::Unknown(error.to_string()))?
            .uri()
            .to_string())
    }
}

pub struct PutPresignedUrlOptions {
    pub expires_in: std::time::Duration,
    pub content_type: Option<String>,
    pub content_length: Option<usize>,
}

#[derive(Debug)]
pub enum RequestPresignedUrlError {
    WrongExpiresIn(String),
    Unknown(String),
}
crate::simple_error_impl!(RequestPresignedUrlError);
