use crate::{psd_sprite::SpriteImage, skia_util::encode_image};
use anyhow::Result;
use namui_type::*;
use skia_safe::{Image, ImageInfo, Paint};

#[derive(Debug, Clone)]
pub(crate) struct SkPositionImage {
    pub dest_rect: Rect<i32>,
    pub sk_image: Image,
}
impl SkPositionImage {
    pub fn intersect_as_mask(&self, other: &SkPositionImage) -> Option<Self> {
        let merged_rect = self.dest_rect.intersect(other.dest_rect);
        let merged_rect = merged_rect?;

        let mut surface = skia_safe::surfaces::raster(
            &ImageInfo::new_a8((merged_rect.width(), merged_rect.height())),
            None,
            None,
        )?;
        let canvas = surface.canvas();
        canvas.translate((-merged_rect.left(), -merged_rect.top()));
        let mut paint = Paint::default();
        paint.set_blend_mode(skia_safe::BlendMode::SrcIn);
        canvas.draw_image(&self.sk_image, self.left_top(), Some(&paint));
        canvas.draw_image(&other.sk_image, other.left_top(), Some(&paint));
        let sk_image = surface.image_snapshot();

        Some(Self {
            dest_rect: merged_rect,
            sk_image,
        })
    }

    fn left_top(&self) -> (i32, i32) {
        (self.dest_rect.left(), self.dest_rect.top())
    }

    pub fn to_sprite_image(&self) -> Result<SpriteImage> {
        Ok(SpriteImage {
            dest_rect: self.dest_rect.map(|x| x.px()),
            encoded: encode_image(&self.sk_image)?,
        })
    }
}
impl AsRef<SkPositionImage> for &SkPositionImage {
    fn as_ref(&self) -> &SkPositionImage {
        self
    }
}
