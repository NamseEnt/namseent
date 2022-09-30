use crate::app::game::Tile;
use namui::prelude::*;

#[derive(Debug)]
pub struct CollisionPrediction {
    pub start_time: Time,
    pub end_time: Time,
    pub direction: CollisionDirection,
    pub start_position: Xy<Tile>,
}

#[derive(Debug)]
pub enum CollisionDirection {
    Vertical,
    Horizontal,
}
