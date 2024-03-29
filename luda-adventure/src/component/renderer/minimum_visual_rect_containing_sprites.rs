use super::Sprite;
use crate::app::game::Tile;
use namui::*;

pub fn minimum_visual_rect_containing_sprites(sprites: &Vec<Sprite>) -> Option<Rect<Tile>> {
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
