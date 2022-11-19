use super::*;
use crate::app::game::{RenderingContext, Tile};
use namui::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum RenderType {
    SpriteBatch(SpriteBatch),
    Sprite(Sprite),
}

impl RenderType {
    pub fn visual_rect(&self) -> Rect<Tile> {
        match self {
            Self::SpriteBatch(sprite_batch) => sprite_batch.visual_rect,
            Self::Sprite(sprite) => sprite.visual_rect,
        }
    }

    pub fn render(&self, rendering_context: &RenderingContext) -> RenderingTree {
        match self {
            RenderType::SpriteBatch(sprite_batch) => sprite_batch.render(rendering_context),
            RenderType::Sprite(sprite) => sprite.render(rendering_context),
        }
    }
}
