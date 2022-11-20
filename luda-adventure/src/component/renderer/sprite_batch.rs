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
        let visual_rect =
            minimum_visual_rect_containing_sprites(&sprites).unwrap_or(Rect::default());
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

fn minimum_visual_rect_containing_sprites(sprites: &Vec<Sprite>) -> Option<Rect<Tile>> {
    let first_visual_rect = sprites.first().map(|sprite| sprite.visual_rect);
    match first_visual_rect {
        Some(first_visual_rect) => Some(
            sprites
                .iter()
                .fold(first_visual_rect, |visual_rect, sprite| {
                    visual_rect.get_minimum_rectangle_containing(sprite.visual_rect)
                }),
        ),
        None => None,
    }
}
