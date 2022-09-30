use crate::app::game::Tile;
use namui::prelude::*;

#[derive(Debug)]
pub enum IntersectionPrediction {
    WillIntersect {
        start_remaining_time: Time,
        end_remaining_time: Time,
        start_remaining_distance: Tile,
    },
    WillNotIntersect,
}
