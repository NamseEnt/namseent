use super::{player_character::PlayerCharacter, *};
use namui::prelude::*;

impl Game {
    pub fn predict_character_movement_if_needed(&mut self, current_time: Time) {
        let collision_box_list =
            self.get_collision_box_list_without_character_collision_box(current_time);

        if let Some((_entity, (_character, collider, mover))) = self
            .ecs_app
            .query_entities_mut::<(&PlayerCharacter, &Collider, &mut Mover)>()
            .first_mut()
        {
            while mover.get_predicted_movement_end_time() - current_time < 1.sec() {
                mover.predict_movement(collider, collision_box_list.as_ref());
            }
        }
    }
}
