use namui::*;
use psd_sprite::SpriteImage;
use skia_safe::Data;

pub(crate) trait SpriteImageExt {
    fn to_namui_image(&self) -> anyhow::Result<Image>;
}
impl SpriteImageExt for SpriteImage {
    fn to_namui_image(&self) -> anyhow::Result<Image> {
        let (pixels, color_type) = self.decode()?;

        let width = self.dest_rect.width().as_f32() as i32;
        let height = self.dest_rect.height().as_f32() as i32;

        let image_info = match color_type {
            psd_sprite::ColorType::Rgba8888 => {
                skia_safe::ImageInfo::new_n32((width, height), skia_safe::AlphaType::Unpremul, None)
            }
            psd_sprite::ColorType::A8 { .. } => skia_safe::ImageInfo::new_a8((width, height)),
        };

        let row_bytes = match color_type {
            psd_sprite::ColorType::Rgba8888 => width * 4,
            psd_sprite::ColorType::A8 { .. } => width,
        } as usize;

        skia_safe::image::images::raster_from_data(&image_info, Data::new_copy(&pixels), row_bytes)
            .map(|sk_image| {
                Image::new(
                    ImageInfo {
                        alpha_type: namui::AlphaType::Unpremul,
                        color_type: match color_type {
                            psd_sprite::ColorType::Rgba8888 => namui::ColorType::Rgba8888,
                            psd_sprite::ColorType::A8 { .. } => namui::ColorType::Alpha8,
                        },
                        height: (height as f32).px(),
                        width: (width as f32).px(),
                    },
                    sk_image,
                )
            })
            .ok_or(anyhow::anyhow!("Failed to create image from SpriteImage"))
    }
}
