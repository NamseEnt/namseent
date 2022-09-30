use crate::app::game::*;
use namui::prelude::*;

impl Game {
    pub fn predict_character_movement_if_needed(&mut self, current_time: Time) {
        let collision_box_list =
            self.get_collision_box_list_without_character_collision_box(current_time);

        if let Some((_entity, (_character, collider, positioner))) = self
            .ecs_app
            .query_entities_mut::<(&PlayerCharacter, &Collider, &mut Positioner)>()
            .first_mut()
        {
            while positioner.get_predicted_movement_end_time() - current_time < 1.sec() {
                positioner.predict_movement(collider, collision_box_list.as_ref());
            }
        }
    }

    fn get_collision_box_list_without_character_collision_box(
        &self,
        current_time: Time,
    ) -> Vec<CollisionBox> {
        self.ecs_app
            .query_entities::<(&Collider, &Positioner)>()
            .iter()
            .filter_map(|(entity, (collider, positioner))| {
                if entity.id() == self.player_entity_id {
                    None
                } else {
                    let position = positioner.get_position(current_time);
                    Some(collider.get_collision_box(position))
                }
            })
            .collect::<Vec<_>>()
    }
}
