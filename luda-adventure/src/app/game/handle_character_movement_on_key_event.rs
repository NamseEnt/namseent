use super::{get_character_velocity_from_key_state, known_id, Game};
use namui::prelude::*;

impl Game {
    pub fn handle_character_movement_on_key_event(&mut self, current_time: Time) {
        if let Some(character) = self
            .object_list
            .iter_mut()
            .find(|game_object| game_object.get_id() == known_id::object::PLAYER_CHARACTER_OBJECT)
        {
            let character_velocity = get_character_velocity_from_key_state();
            let character_velocity_has_not_changed = character_velocity.x * 1.ms()
                == self.state.character.last_velocity.x * 1.ms()
                && character_velocity.y * 1.ms() == self.state.character.last_velocity.y * 1.ms();
            if character_velocity_has_not_changed {
                return;
            }
            if let Some(character) = character.get_mover() {
                self.state.character.last_velocity = character_velocity;
                character.set_velocity(current_time, character_velocity, f32::INFINITY.ms())
            }
        }
    }
}
