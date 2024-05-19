use crate::{system::InitResult, *};

pub(super) async fn init() -> InitResult {
    Ok(())
}

pub async fn load_image(url: impl AsRef<str>) -> Result<Image> {
    let image_handle = system::skia::load_image_from_url(url).await?;

    let wh = Wh::new(image_handle.width, image_handle.height);

    Ok(Image {
        src: ImageSource::ImageHandle { image_handle },
        wh,
    })
}
