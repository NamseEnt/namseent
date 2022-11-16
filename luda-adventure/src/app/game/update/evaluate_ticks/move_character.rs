use crate::app::game::{Game, Movement, Mover, PlayerCharacter, Positioner, TICK_INTERVAL};
use namui::prelude::*;

impl Game {
    pub fn move_character(&mut self) {
        if let Some((_player_character, positioner, mover)) = self
            .ecs_app
            .query_component_mut::<(PlayerCharacter, Positioner, Mover)>()
            .first_mut()
        {
            if let Movement::Moving(velocity) = mover.movement {
                let delta_xy = velocity * Xy::single(TICK_INTERVAL);
                positioner.xy = positioner.xy + delta_xy;
            }
        }
    }
}
