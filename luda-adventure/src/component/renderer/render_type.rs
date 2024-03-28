use super::*;
use crate::app::game::{GameState, RenderingContext, Tile};
use namui::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum RenderType {
    SpriteBatch(SpriteBatch),
    Sprite(Sprite),
    SpriteAnimation(SpriteAnimation),
}

impl RenderType {
    pub fn visual_rect(&self) -> Rect<Tile> {
        match self {
            RenderType::SpriteBatch(sprite_batch) => sprite_batch.visual_rect,
            RenderType::Sprite(sprite) => sprite.visual_rect,
            RenderType::SpriteAnimation(sprite_animation) => sprite_animation.visual_rect,
        }
    }

    pub fn render(
        &self,
        rendering_context: &RenderingContext,
        game_state: &GameState,
    ) -> RenderingTree {
        match self {
            RenderType::SpriteBatch(sprite_batch) => sprite_batch.render(rendering_context),
            RenderType::Sprite(sprite) => sprite.render(rendering_context),
            RenderType::SpriteAnimation(sprite_animation) => {
                sprite_animation.render(rendering_context, game_state)
            }
        }
    }
}
