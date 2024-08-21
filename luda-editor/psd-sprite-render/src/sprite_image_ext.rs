use ::image::ImageReader;
use namui::*;
use psd_sprite::SpriteImage;
use skia_safe::Data;
use std::io::Cursor;

pub(crate) trait SpriteImageExt {
    fn to_namui_image(&self) -> anyhow::Result<Image>;
    fn to_namui_image_a8(&self) -> anyhow::Result<Image>;
}
impl SpriteImageExt for SpriteImage {
    fn to_namui_image(&self) -> anyhow::Result<Image> {
        let image = ImageReader::new(Cursor::new(&self.webp))
            .with_guessed_format()?
            .decode()?;
        let rgba = image.to_rgba8().into_vec();
        skia_safe::image::images::raster_from_data(
            &skia_safe::ImageInfo::new_n32(
                (image.width() as i32, image.height() as i32),
                skia_safe::AlphaType::Unpremul,
                None,
            ),
            Data::new_copy(&rgba),
            image.width() as usize * 4,
        )
        .map(|sk_image| {
            Image::new(
                ImageInfo {
                    alpha_type: namui::AlphaType::Unpremul,
                    color_type: namui::ColorType::Rgba8888,
                    height: (image.height() as f32).px(),
                    width: (image.width() as f32).px(),
                },
                sk_image,
            )
        })
        .ok_or(anyhow::anyhow!("Failed to create image from SpriteImage"))
    }

    fn to_namui_image_a8(&self) -> anyhow::Result<Image> {
        let image = ImageReader::new(Cursor::new(&self.webp))
            .with_guessed_format()?
            .decode()?;
        let rgba = image.to_luma8().into_vec();
        skia_safe::image::images::raster_from_data(
            &skia_safe::ImageInfo::new_a8((image.width() as i32, image.height() as i32)),
            Data::new_copy(&rgba),
            image.width() as usize,
        )
        .map(|sk_image| {
            Image::new(
                ImageInfo {
                    alpha_type: namui::AlphaType::Unpremul,
                    color_type: namui::ColorType::Alpha8,
                    height: (image.height() as f32).px(),
                    width: (image.width() as f32).px(),
                },
                sk_image,
            )
        })
        .ok_or(anyhow::anyhow!(
            "Failed to create a8 image from SpriteImage"
        ))
    }
}
