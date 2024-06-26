mod presigned_url;

use aws_config::SdkConfig;
use aws_sdk_s3::{types::ByteStream, Client};
use lambda_web::is_running_on_lambda;
pub use presigned_url::*;

#[derive(Debug)]
pub struct S3 {
    client: Client,
    bucket_name: String,
    key_prefix: String,
    rest_api_endpoint: String,
}

impl S3 {
    pub fn new(config: &SdkConfig, rest_api_endpoint: String) -> Self {
        let bucket_name = {
            if is_running_on_lambda() {
                std::env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME is not set")
            } else {
                "one-for-all".to_string()
            }
        };
        let key_prefix = {
            if is_running_on_lambda() {
                std::env::var("S3_KEY_PREFIX").expect("S3_KEY_PREFIX is not set")
            } else {
                "".to_string()
            }
        };
        S3 {
            client: Client::new(config),
            bucket_name,
            key_prefix,
            rest_api_endpoint,
        }
    }
    fn key_with_prefix(&self, key: impl AsRef<str>) -> String {
        crate::append_slash![self.key_prefix, key.as_ref()]
    }
    fn remove_key_prefix(&self, key: impl AsRef<str>) -> String {
        let key = key.as_ref();
        if key.starts_with(&self.key_prefix) {
            key[self.key_prefix.len()..].to_string()
        } else {
            unreachable!()
        }
    }
    pub async fn get_object(&self, key: String) -> Result<Vec<u8>, GetObjectError> {
        let result = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(self.key_with_prefix(key))
            .send()
            .await;

        if let Err(error) = result {
            let error: aws_sdk_s3::Error = error.into();
            if let aws_sdk_s3::Error::NoSuchKey(_) = error {
                return Err(GetObjectError::NotFound);
            } else {
                eprintln!("error on get_object: {:?}", error);
                //TODO: Hide reason and just show error id to client and log it with error id.
                return Err(GetObjectError::Unknown(error.to_string()));
            }
        }
        let output = result.unwrap();

        let body = output.body.collect().await;
        match body {
            Ok(bytes) => Ok(bytes.into_bytes().to_vec()),
            Err(error) => {
                eprintln!("error on get_object body.collect: {:?}", error);
                Err(GetObjectError::Unknown(error.to_string()))
            }
        }
    }
    pub async fn list_objects(
        &self,
        prefix: impl AsRef<str>,
        start_after: Option<String>,
    ) -> Result<Vec<ListedObject>, ListObjectsError> {
        let mut builder = self
            .client
            .list_objects()
            .bucket(&self.bucket_name)
            .prefix(self.key_with_prefix(prefix));
        if let Some(start_after) = start_after {
            builder = builder.marker(self.key_with_prefix(start_after));
        }

        let output = builder.send().await.map_err(|error| {
            eprintln!("error on list_objects: {:?}", error);
            ListObjectsError::Unknown(error.to_string())
        })?;

        let objects = output
            .contents
            .map(|contents| {
                contents
                    .into_iter()
                    .map(|object| {
                        let key = object.key.unwrap();
                        ListedObject {
                            key: self.remove_key_prefix(&key),
                            url: self.prefixed_key_to_url(&key),
                        }
                    })
                    .collect()
            })
            .unwrap_or(vec![]);
        Ok(objects)
    }
    // TODO: Change it with presigned url.
    // TODO: Tag the user to determine who uploaded too much data.
    // 이미지 public 읽기 가능하도록 해야할 것 같아. 해줘.
    pub async fn put_object(&self, key: String, bytes: Vec<u8>) -> Result<(), PutObjectError> {
        let result = self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(self.key_with_prefix(key))
            .body(ByteStream::from(bytes.to_vec()))
            .send()
            .await;

        if let Err(error) = result {
            eprintln!("error on put_object: {:?}", error);
            return Err(PutObjectError::Unknown(error.to_string()));
        }

        Ok(())
    }

    fn prefixed_key_to_url(&self, prefixed_key: &str) -> String {
        crate::append_slash![self.rest_api_endpoint, self.bucket_name, prefixed_key,]
    }

    pub fn get_url(&self, key: impl AsRef<str>) -> String {
        crate::append_slash![
            self.rest_api_endpoint,
            self.bucket_name,
            self.key_prefix,
            key.as_ref(),
        ]
    }

    pub(crate) async fn delete_object(
        &self,
        key: impl AsRef<str>,
    ) -> Result<(), DeleteObjectError> {
        let result = self
            .client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(self.key_with_prefix(key))
            .send()
            .await;

        if let Err(error) = result {
            eprintln!("error on delete_object: {:?}", error);
            return Err(DeleteObjectError::Unknown(error.to_string()));
        }

        Ok(())
    }
}

pub struct ListedObject {
    pub key: String,
    pub url: String,
}

#[derive(Debug)]
pub enum GetObjectError {
    NotFound,
    Unknown(String),
}
crate::simple_error_impl!(GetObjectError);

#[derive(Debug)]
pub enum ListObjectsError {
    Unknown(String),
}
crate::simple_error_impl!(ListObjectsError);

#[derive(Debug)]
pub enum PutObjectError {
    Unknown(String),
}
crate::simple_error_impl!(PutObjectError);

#[derive(Debug)]
pub enum DeleteObjectError {
    Unknown(String),
}
crate::simple_error_impl!(DeleteObjectError);
