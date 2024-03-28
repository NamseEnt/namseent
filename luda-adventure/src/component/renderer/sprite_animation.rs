use super::{minimum_visual_rect_containing_sprites, Sprite};
use crate::app::game::{GameState, RenderingContext, Tile};
use namui::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpriteAnimation {
    pub visual_rect: Rect<Tile>,
    pub sprites: Vec<Sprite>,
    pub frame_time: Time,
    pub started_at: Time,
}

impl SpriteAnimation {
    pub fn new(sprites: Vec<Sprite>, frame_time: Time, started_at: Time) -> Self {
        let visual_rect =
            minimum_visual_rect_containing_sprites(&sprites).unwrap_or(Rect::default());
        Self {
            visual_rect,
            sprites,
            frame_time,
            started_at,
        }
    }

    pub fn render(
        &self,
        rendering_context: &RenderingContext,
        game_state: &GameState,
    ) -> RenderingTree {
        let elapsed_time = game_state.tick.current_time - self.started_at;
        let frame_index = (elapsed_time / self.frame_time) as usize % self.sprites.len();
        self.sprites[frame_index].render(rendering_context)
    }
}
