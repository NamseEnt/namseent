use crate::{system::InitResult, *};

pub(super) async fn init() -> InitResult {
    Ok(())
}

pub async fn load_image(url: impl AsRef<str>) -> Result<Image> {
    system::skia::load_image_from_url(url).await
}
