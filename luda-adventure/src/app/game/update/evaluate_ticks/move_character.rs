use crate::app::game::*;
use crate::component::*;
use namui::*;

impl Game {
    pub fn move_character(&mut self, delta_time: Time) {
        if let Some((_entity, (_player_character, positioner, mover))) = self
            .ecs_app
            .query_entities_mut::<(&PlayerCharacter, &mut Positioner, &Mover)>()
            .first_mut()
        {
            if let Movement::Moving(velocity) = mover.movement {
                let delta_xy = velocity * Xy::single(delta_time);
                positioner.xy = positioner.xy + delta_xy;
            }
        }
    }
}
