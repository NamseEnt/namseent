use crate::app::game::{TileExt, Velocity};
use namui::prelude::*;

pub struct CharacterState {
    pub last_velocity: Velocity,
}

impl CharacterState {
    pub fn new() -> Self {
        Self {
            last_velocity: Velocity::single(Per::new(0.tile(), 1.ms())),
        }
    }
}
