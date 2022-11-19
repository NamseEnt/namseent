use super::Sprite;
use crate::app::game::{RenderingContext, Tile};
use namui::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpriteBatch {
    pub visual_rect: Rect<Tile>,
    pub sprites: Vec<Sprite>,
}

impl SpriteBatch {
    pub fn new(sprites: Vec<Sprite>) -> Self {
        let visual_rect = sprites.iter().fold(Rect::default(), |visual_rect, sprite| {
            visual_rect.get_minimum_rectangle_containing(sprite.visual_rect)
        });
        Self {
            visual_rect,
            sprites,
        }
    }

    pub fn render(&self, rendering_context: &RenderingContext) -> RenderingTree {
        render(
            self.sprites
                .iter()
                .map(|sprite| sprite.render(rendering_context)),
        )
    }

    pub fn translate(&mut self, xy: Xy<Tile>) {
        self.visual_rect = self.visual_rect + xy;
        self.sprites
            .iter_mut()
            .for_each(|sprite| sprite.translate(xy));
    }
}
