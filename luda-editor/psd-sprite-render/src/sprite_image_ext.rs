use namui::*;
use psd_sprite::SpriteImage;
use skia_safe::Data;

pub(crate) trait SpriteImageExt {
    fn to_namui_image(&self) -> anyhow::Result<Image>;
    fn to_namui_image_a8(&self) -> anyhow::Result<Image>;
}
impl SpriteImageExt for SpriteImage {
    fn to_namui_image(&self) -> anyhow::Result<Image> {
        let decoded = self.decode()?;

        skia_safe::image::images::raster_from_data(
            &skia_safe::ImageInfo::new_n32(
                (decoded.width as i32, decoded.height as i32),
                skia_safe::AlphaType::Unpremul,
                None,
            ),
            Data::new_copy(&decoded.pixels),
            decoded.width * 4,
        )
        .map(|sk_image| {
            Image::new(
                ImageInfo {
                    alpha_type: namui::AlphaType::Unpremul,
                    color_type: namui::ColorType::Rgba8888,
                    height: (decoded.height as f32).px(),
                    width: (decoded.width as f32).px(),
                },
                sk_image,
            )
        })
        .ok_or(anyhow::anyhow!("Failed to create image from SpriteImage"))
    }

    fn to_namui_image_a8(&self) -> anyhow::Result<Image> {
        let decoded = self.decode()?;

        skia_safe::image::images::raster_from_data(
            &skia_safe::ImageInfo::new_a8((decoded.width as i32, decoded.height as i32)),
            Data::new_copy(&decoded.pixels),
            decoded.width,
        )
        .map(|sk_image| {
            Image::new(
                ImageInfo {
                    alpha_type: namui::AlphaType::Unpremul,
                    color_type: namui::ColorType::Alpha8,
                    height: (decoded.height as f32).px(),
                    width: (decoded.width as f32).px(),
                },
                sk_image,
            )
        })
        .ok_or(anyhow::anyhow!(
            "Failed to create a8 image from SpriteImage"
        ))
    }
}
