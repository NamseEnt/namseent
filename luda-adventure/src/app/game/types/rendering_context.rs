use super::Tile;
use namui::prelude::*;

pub struct RenderingContext {
    pub current_time: Time,
    pub px_per_tile: Per<Px, Tile>,
    pub screen_rect: Rect<Tile>,
    pub interpolation_progress: f32,
}
