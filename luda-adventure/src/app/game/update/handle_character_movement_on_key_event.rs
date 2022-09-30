use crate::app::game::*;
use namui::prelude::*;

impl Game {
    pub fn handle_character_movement_on_key_event(&mut self, current_time: Time) {
        if let Some((_character, (_player_character, positioner))) = self
            .ecs_app
            .query_entities_mut::<(&PlayerCharacter, &mut Positioner)>()
            .first_mut()
        {
            let character_velocity = get_character_velocity_from_key_state();
            let character_velocity_has_not_changed = character_velocity.x * 1.ms()
                == self.state.character.last_velocity.x * 1.ms()
                && character_velocity.y * 1.ms() == self.state.character.last_velocity.y * 1.ms();
            if character_velocity_has_not_changed {
                return;
            }
            self.state.character.last_velocity = character_velocity;
            positioner.set_velocity(current_time, character_velocity, f32::INFINITY.ms())
        }
    }
}

fn get_character_velocity_from_key_state() -> Velocity {
    let mut direction = Xy::<f32>::zero();
    if namui::keyboard::any_code_press([Code::ArrowDown]) {
        direction.y += 1.0;
    }
    if namui::keyboard::any_code_press([Code::ArrowUp]) {
        direction.y -= 1.0;
    }
    if namui::keyboard::any_code_press([Code::ArrowRight]) {
        direction.x += 1.0;
    }
    if namui::keyboard::any_code_press([Code::ArrowLeft]) {
        direction.x -= 1.0;
    }
    let direction_length = direction.length();
    let normalized_direction = match direction_length == 0.0 {
        true => direction,
        false => direction / direction_length,
    };
    Xy {
        x: Per::new(10.tile() * normalized_direction.x, 1.sec()),
        y: Per::new(10.tile() * normalized_direction.y, 1.sec()),
    }
}
