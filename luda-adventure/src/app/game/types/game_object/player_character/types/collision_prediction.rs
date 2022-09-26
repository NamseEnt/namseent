use namui::prelude::*;

use crate::app::game::Position;

#[derive(Debug)]
pub struct CollisionPrediction {
    pub start_time: Time,
    pub end_time: Time,
    pub direction: CollisionDirection,
    pub start_position: Position,
}

#[derive(Debug)]
pub enum CollisionDirection {
    Vertical,
    Horizontal,
}
