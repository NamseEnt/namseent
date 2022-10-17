use super::{Movement, MovementPath};
use crate::app::game::Tile;
use namui::prelude::*;

#[derive(ecs_macro::Component, Debug)]
pub struct Positioner {
    movement: Movement,
}

impl Positioner {
    pub fn new() -> Self {
        Self::new_with_xy(Xy::zero())
    }
    pub fn new_with_xy(xy: Xy<Tile>) -> Self {
        Self {
            movement: Movement::stay_forever(xy),
        }
    }

    pub fn xy(&self, time: Time) -> Xy<Tile> {
        self.movement.xy(time)
    }

    pub fn movement_end_time(&self) -> Time {
        self.movement.end_time()
    }

    pub fn push_movement_path(&mut self, movement_path: MovementPath) {
        self.movement.push_movement_path(movement_path)
    }

    pub fn stay_forever(&mut self, xy: Xy<Tile>) {
        self.movement = Movement::stay_forever(xy);
    }

    pub fn move_from(&mut self, xy: Xy<Tile>, time: Time) {
        self.movement = Movement::move_from(xy, time);
    }
}
