use crate::app::game::{Tile, TileExt};
use namui::prelude::*;

pub type Velocity = Xy<Per<Tile, Time>>;

pub fn zero_velocity() -> Velocity {
    Xy::single(zero_speed())
}

pub fn zero_speed() -> Per<Tile, Time> {
    Per::new(0.tile(), 1.ms())
}
