use aws_config::SdkConfig;
use aws_sdk_s3::{types::ByteStream, Client};
use lambda_web::is_running_on_lambda;

#[derive(Debug)]
pub struct S3 {
    client: Client,
    bucket_name: String,
    key_prefix: String,
}

impl S3 {
    pub fn new(config: &SdkConfig) -> Self {
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
        }
    }
    fn key_with_prefix(&self, key: String) -> String {
        format!("{}{}", self.key_prefix, key)
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
        prefix: String,
        start_after: Option<String>,
    ) -> Result<Vec<String>, ListObjectsError> {
        let mut builder = self
            .client
            .list_objects()
            .bucket(&self.bucket_name)
            .prefix(self.key_with_prefix(prefix));
        if let Some(start_after) = start_after {
            builder = builder.marker(start_after);
        }

        let result = builder.send().await;

        if let Err(error) = result {
            eprintln!("error on list_objects: {:?}", error);
            return Err(ListObjectsError::Unknown(error.to_string()));
        }
        let output = result.unwrap();
        let contents = output.contents;
        let keys = contents
            .map(|contents| {
                contents
                    .into_iter()
                    .map(|content| content.key.unwrap())
                    .collect()
            })
            .unwrap_or(vec![]);
        Ok(keys)
    }
    // TODO: Change it with presigned url.
    // TODO: Tag the user to determine who uploaded too much data.
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
