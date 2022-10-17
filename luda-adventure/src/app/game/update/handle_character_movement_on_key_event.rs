use crate::app::game::*;
use namui::prelude::*;

impl Game {
    pub fn handle_user_input_for_character_movement(&mut self, current_time: Time) {
        if let Some((_entity, (character, positioner))) = self
            .ecs_app
            .query_entities_mut::<(&mut PlayerCharacter, &mut Positioner)>()
            .first_mut()
        {
            let user_input = get_user_input_from_key_state();
            let user_input_not_changed = user_input == character.user_input();
            if user_input_not_changed {
                return;
            }
            character.set_user_input(user_input);
            let current_xy = positioner.xy(current_time);
            positioner.move_from(current_xy, current_time)
        }
    }
}

fn get_user_input_from_key_state() -> Xy<f32> {
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
    normalized_direction
}
