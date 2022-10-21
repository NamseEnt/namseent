use crate::app::game::{Game, PlayerCharacter, Positioner, TICK_INTERVAL};

impl Game {
    pub fn move_character(&mut self) {
        if let Some((_entity, (_player_character, positioner))) = self
            .ecs_app
            .query_entities_mut::<(&mut PlayerCharacter, &mut Positioner)>()
            .first_mut()
        {
            positioner.apply_movement(TICK_INTERVAL);
        }
    }
}
