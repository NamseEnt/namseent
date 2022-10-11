use crate::app::game::Tile;
use namui::prelude::*;

#[derive(Clone, Debug, Copy)]
pub struct CollisionPrediction {
    pub start_time: Time,
    pub end_time: Time,
    pub direction: CollisionDirection,
    pub start_position: Xy<Tile>,
}

#[derive(Clone, Debug, Copy)]
pub enum CollisionDirection {
    Vertical,
    Horizontal,
}
