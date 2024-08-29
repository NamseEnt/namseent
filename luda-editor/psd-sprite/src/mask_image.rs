use crate::psd_sprite::SpriteImage;
use anyhow::Result;
use namui_type::*;
use nimg::encode_a8;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub(crate) struct MaskImage<'a> {
    pub dest_rect: Rect<usize>,
    pub bytes: Cow<'a, [u8]>,
}
impl MaskImage<'_> {
    pub fn intersect(&self, other: &MaskImage) -> Option<Self> {
        let merged_rect = self.dest_rect.intersect(other.dest_rect)?;

        if merged_rect.width() == 0 || merged_rect.height() == 0 {
            return None;
        }

        let mut bytes = Vec::with_capacity(merged_rect.width() * merged_rect.height());

        for y in merged_rect.left()..merged_rect.right() {
            for x in merged_rect.top()..merged_rect.bottom() {
                let self_x = x - self.dest_rect.left();
                let self_y = y - self.dest_rect.top();
                let other_x = x - other.dest_rect.left();
                let other_y = y - other.dest_rect.top();

                let self_index = self_x + self_y * self.dest_rect.width();
                let other_index = other_x + other_y * other.dest_rect.width();

                let self_value = self.bytes.get(self_index).copied().unwrap_or(0);
                let other_value = other.bytes.get(other_index).copied().unwrap_or(0);

                bytes.push(((self_value as u16) * (other_value as u16) / 255) as u8);
            }
        }

        Some(Self {
            dest_rect: merged_rect,
            bytes: Cow::Owned(bytes),
        })
    }

    pub fn to_sprite_image(&self) -> Result<SpriteImage> {
        Ok(SpriteImage {
            dest_rect: self.dest_rect.map(|x| (*x as f32).px()),
            encoded: crate::SpriteImageBuffer::A8 {
                a: encode_a8(
                    self.dest_rect.width(),
                    self.dest_rect.height(),
                    self.bytes.as_ref(),
                )?,
            },
        })
    }
}
