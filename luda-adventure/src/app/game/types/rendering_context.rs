use super::Tile;
use namui::*;

pub struct RenderingContext {
    pub px_per_tile: Per<Px, Tile>,
    pub screen_rect: Rect<Tile>,
}
