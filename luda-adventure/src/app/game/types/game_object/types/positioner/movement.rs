use super::MovementPath;
use crate::app::game::Tile;
use namui::prelude::*;
use std::f32::{INFINITY, NEG_INFINITY};

#[derive(Clone, Debug)]
pub enum Movement {
    Fixed(Xy<Tile>),
    Moving(Vec<MovementPath>),
}
impl Movement {
    pub fn xy(&self, current_time: Time) -> Xy<Tile> {
        match self {
            Movement::Fixed(xy) => *xy,
            Movement::Moving(movement_path_list) => {
                for movement_path in movement_path_list {
                    if let Some(xy) = movement_path.xy(current_time) {
                        return xy;
                    }
                }
                unreachable!("MovementPath did not calculated");
            }
        }
    }

    pub fn stay_forever(xy: Xy<Tile>) -> Self {
        Self::Fixed(xy)
    }

    pub fn move_from(xy: Xy<Tile>, time: Time) -> Self {
        Self::Moving(vec![MovementPath::new(time, time, xy, xy)])
    }

    pub fn end_time(&self) -> Time {
        match self {
            Movement::Fixed(_) => INFINITY.ms(),
            Movement::Moving(movement_path_list) => match movement_path_list.last() {
                Some(movement_path) => movement_path.end_time(),
                None => NEG_INFINITY.ms(),
            },
        }
    }

    pub fn push_movement_path(&mut self, movement_path: MovementPath) {
        match self {
            Movement::Fixed(_) => *self = Movement::Moving(vec![movement_path]),
            Movement::Moving(movement_path_list) => {
                movement_path_list.push(movement_path);
            }
        }
    }
}
