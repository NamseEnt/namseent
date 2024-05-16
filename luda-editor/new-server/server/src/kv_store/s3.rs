// use super::*;
// use anyhow::Result;
// use aws_sdk_s3::{primitives::ByteStream, Client};

// #[derive(Clone)]
// pub struct S3KsStore {
//     client: Client,
//     bucket_name: String,
// }
// impl S3KsStore {
//     pub(crate) fn new(client: Client, bucket_name: impl AsRef<str>) -> Self {
//         Self {
//             client,
//             bucket_name: bucket_name.as_ref().to_string(),
//         }
//     }
// }

// impl KvStore for S3KsStore {
//     async fn get(&self, key: impl AsRef<str>) -> Result<Option<ValueBuffer>> {
//         let result = self
//             .client
//             .get_object()
//             .bucket(&self.bucket_name)
//             .key(key.as_ref())
//             .send()
//             .await;

//         match result {
//             Ok(output) => {
//                 let buffer = output.body.collect().await?;
//                 Ok(Some(ValueBuffer::Vec(buffer.to_vec())))
//             }
//             Err(err) => match err.as_service_error() {
//                 Some(aws_sdk_s3::operation::get_object::GetObjectError::NoSuchKey(_)) => Ok(None),
//                 _ => Err(err.into()),
//             },
//         }
//     }

//     async fn put(&self, key: impl AsRef<str>, value: &impl AsRef<[u8]>) -> Result<()> {
//         // TODO: multipart

//         crate::s3()
//             .put_object()
//             .bucket(&self.bucket_name)
//             .key(key.as_ref())
//             .body(ByteStream::from(value.as_ref().to_vec()))
//             .send()
//             .await?;

//         Ok(())
//     }

//     async fn delete(&self, key: impl AsRef<str>) -> Result<()> {
//         crate::s3()
//             .delete_object()
//             .bucket(&self.bucket_name)
//             .key(key.as_ref())
//             .send()
//             .await?;

//         Ok(())
//     }
// }
