use crate::{session::SessionDocument, storage::s3::*};
use rpc::base64;

#[derive(Debug)]
pub struct ResourceService {}

impl ResourceService {
    pub fn new() -> Self {
        ResourceService {}
    }
}

impl rpc::ResourceService<SessionDocument> for ResourceService {
    fn get_resource<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::get_resource::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::get_resource::Result> + Send>>
    {
        Box::pin(async move {
            let result = crate::s3().get_object(req.resource_id).await;

            match result {
                Ok(bytes) => Ok(rpc::get_resource::Response {
                    base64: base64::encode(bytes),
                }),
                Err(error) => match error {
                    GetObjectError::NotFound => Err(rpc::get_resource::Error::NotFound),
                    GetObjectError::Unknown(error) => Err(rpc::get_resource::Error::Unknown(error)),
                },
            }
        })
    }

    fn list_resources<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::list_resources::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::list_resources::Result> + Send>>
    {
        Box::pin(async move {
            let prefix = "".to_string(); // TODO
            let result = crate::s3().list_objects(prefix, req.start_after).await;

            match result {
                Ok(resource_keys) => Ok(rpc::list_resources::Response { resource_keys }),
                Err(error) => match error {
                    ListObjectsError::Unknown(error) => {
                        Err(rpc::list_resources::Error::Unknown(error))
                    }
                },
            }
        })
    }

    fn put_resource<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::put_resource::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<
                    Output = Result<rpc::put_resource::Response, rpc::put_resource::Error>,
                >
                + Send,
        >,
    > {
        Box::pin(async move {
            let result = crate::s3()
                .put_object(req.resource_id, base64::decode(req.base64).unwrap().into())
                .await;

            match result {
                Ok(_) => Ok(rpc::put_resource::Response {}),
                Err(error) => match error {
                    PutObjectError::Unknown(error) => Err(rpc::put_resource::Error::Unknown(error)),
                },
            }
        })
    }
}
