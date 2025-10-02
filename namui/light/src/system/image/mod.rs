use crate::*;

pub(super) async fn init() -> Result<()> {
    Ok(())
}

pub async fn load_image(resource_location: impl AsRef<ResourceLocation>) -> Result<Image> {
    system::skia::load_image_from_resource_location(resource_location).await
}
