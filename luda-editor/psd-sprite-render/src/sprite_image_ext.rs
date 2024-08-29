use namui::*;
use psd_sprite::SpriteImage;
use skia_safe::Data;

pub(crate) trait SpriteImageExt {
    fn to_namui_image(&self) -> anyhow::Result<Image>;
}
impl SpriteImageExt for SpriteImage {
    fn to_namui_image(&self) -> anyhow::Result<Image> {
        let decoded = self.decode()?;

        let width = self.dest_rect.width().as_f32() as i32;
        let height = self.dest_rect.height().as_f32() as i32;

        let image_info = match &self.encoded {
            psd_sprite::SpriteImageBuffer::Rgb8A8 { .. } => {
                skia_safe::ImageInfo::new_n32((width, height), skia_safe::AlphaType::Unpremul, None)
            }
            psd_sprite::SpriteImageBuffer::A8 { .. } => {
                skia_safe::ImageInfo::new_a8((width, height))
            }
        };

        let pixels = match decoded {
            psd_sprite::SpriteImageBuffer::Rgb8A8 { rgb, a } => {
                let mut pixels = Vec::with_capacity((width * height * 4) as usize);
                for (rgb, a) in rgb.chunks(3).zip(a.iter()) {
                    pixels.extend_from_slice(rgb);
                    pixels.push(*a);
                }
                Data::new_copy(&pixels)
            }
            psd_sprite::SpriteImageBuffer::A8 { a } => Data::new_copy(&a),
        };

        let row_bytes = match &self.encoded {
            psd_sprite::SpriteImageBuffer::Rgb8A8 { .. } => width * 4,
            psd_sprite::SpriteImageBuffer::A8 { .. } => width,
        } as usize;

        skia_safe::image::images::raster_from_data(&image_info, pixels, row_bytes)
            .map(|sk_image| {
                Image::new(
                    ImageInfo {
                        alpha_type: namui::AlphaType::Unpremul,
                        color_type: match &self.encoded {
                            psd_sprite::SpriteImageBuffer::Rgb8A8 { .. } => {
                                namui::ColorType::Rgba8888
                            }
                            psd_sprite::SpriteImageBuffer::A8 { .. } => namui::ColorType::Alpha8,
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
