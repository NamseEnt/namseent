use crate::app::game::Tile;
use namui::prelude::*;

#[derive(ecs_macro::Component, Debug)]
pub struct Positioner {
    movement: Movement,
    current_xy: Xy<Tile>,
    previous_xy: Xy<Tile>,
}

#[derive(Clone, Debug)]
pub enum Movement {
    Fixed,
    Moving(Velocity),
}

pub type Velocity = Xy<Per<Tile, Time>>;

impl Positioner {
    pub fn new() -> Self {
        Self::new_with_xy(Xy::zero())
    }
    pub fn new_with_xy(xy: Xy<Tile>) -> Self {
        Self {
            movement: Movement::Fixed,
            current_xy: xy,
            previous_xy: xy,
        }
    }

    pub fn xy(&self) -> Xy<Tile> {
        self.current_xy
    }
    pub fn xy_with_interpolation(&self, interpolation_progress: f32) -> Xy<Tile> {
        self.previous_xy * (1.0 - interpolation_progress) + self.current_xy * interpolation_progress
    }

    pub fn set_xy(&mut self, xy: Xy<Tile>) {
        self.current_xy = xy;
    }

    pub fn set_movement(&mut self, movement: Movement) {
        self.movement = movement;
    }

    pub fn apply_movement(&mut self, duration: Time) {
        if let Movement::Moving(velocity) = self.movement {
            self.current_xy.x += velocity.x * duration;
            self.current_xy.y += velocity.y * duration;
        }
    }

    pub fn save_xy_for_interpolation(&mut self) {
        self.previous_xy = self.current_xy;
    }
}
