mod documents;

use self::documents::image_s3_key;
use crate::session::SessionDocument;
use documents::*;
use rpc::{data::ImageWithLabels, utils::retry_on_error};

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
        Box::pin(async move {})
    }

    fn prepare_upload_image<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::prepare_upload_image::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::prepare_upload_image::Result> + Send>,
    > {
        Box::pin(async move {})
    }

    fn list_images<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        req: rpc::list_images::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::list_images::Result> + Send>>
    {
        Box::pin(async move {})
    }

    fn delete_image<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::delete_image::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::delete_image::Result> + Send>>
    {
        // TODO: Remove s3 image using queue or sweep using bot.
        Box::pin(async move {})
    }
}
