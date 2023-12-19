use anyhow::Result;
use namui_type::*;

pub struct ImageBitmap {
    image_info: ImageInfo,
}

impl ImageBitmap {
    pub async fn from_u8(image_source: &ImageSource, data: &[u8]) -> Result<ImageBitmap> {
        let image_info =
            tokio::task::block_in_place(|| crate::system::skia::load_image(image_source, data));

        Ok(ImageBitmap { image_info })
    }

    pub fn width(&self) -> Px {
        self.image_info.width
    }

    pub fn height(&self) -> Px {
        self.image_info.height
    }
}
