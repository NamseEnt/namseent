use super::Tile;
use namui::prelude::*;

pub type Position = Xy<Tile>;

pub trait PositionExt {
    fn as_px(self, px_per_tile: Per<Px, Tile>) -> Xy<Px>;
}

impl PositionExt for Position {
    fn as_px(self, px_per_tile: Per<Px, Tile>) -> Xy<Px> {
        Xy {
            x: px_per_tile * self.x,
            y: px_per_tile * self.y,
        }
    }
}
