use crate::app::game::Tile;
use namui::prelude::*;

#[derive(ecs_macro::Component, Debug)]
pub struct Mover {
    movement: Movement,
}

#[derive(Copy, Clone, Debug)]
pub enum Movement {
    Fixed,
    Moving(Velocity),
}

pub type Velocity = Xy<Per<Tile, Time>>;

impl Mover {
    pub fn new() -> Self {
        Self {
            movement: Movement::Fixed,
        }
    }

    pub fn movement(&self) -> Movement {
        self.movement
    }

    pub fn set_movement(&mut self, movement: Movement) {
        self.movement = movement;
    }
}
