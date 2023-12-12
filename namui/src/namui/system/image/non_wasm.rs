use anyhow::Result;

pub struct ImageBitmap {
    dyn_image: ::image::DynamicImage,
}

impl ImageBitmap {
    pub async fn from_u8(data: &[u8]) -> Result<ImageBitmap> {
        let dyn_image = tokio::task::block_in_place(|| ::image::load_from_memory(data))?;

        Ok(ImageBitmap { dyn_image })
    }

    pub fn width(&self) -> u32 {
        self.dyn_image.width()
    }

    pub fn height(&self) -> u32 {
        self.dyn_image.height()
    }
}
