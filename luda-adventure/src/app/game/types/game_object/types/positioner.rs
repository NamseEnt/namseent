use crate::app::game::Tile;
use namui::prelude::*;

#[derive(ecs_macro::Component, Debug)]
pub struct Positioner {
    movement: Movement,
    xy: Xy<Tile>,
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
            xy,
        }
    }

    pub fn xy(&self) -> Xy<Tile> {
        self.xy
    }

    pub fn set_xy(&mut self, xy: Xy<Tile>) {
        self.xy = xy;
    }

    pub fn set_movement(&mut self, movement: Movement) {
        self.movement = movement;
    }

    pub fn apply_movement(&mut self, duration: Time) {
        if let Movement::Moving(velocity) = self.movement {
            self.xy.x += velocity.x * duration;
            self.xy.y += velocity.y * duration;
        }
    }
}
