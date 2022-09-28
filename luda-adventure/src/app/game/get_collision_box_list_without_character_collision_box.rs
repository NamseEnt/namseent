use super::*;
use namui::prelude::*;

impl Game {
    pub fn get_collision_box_list_without_character_collision_box(
        &self,
        current_time: Time,
    ) -> Vec<CollisionBox> {
        self.ecs_app
            .query_entities::<(&Collider, &Mover)>()
            .iter()
            .filter_map(|(entity, (collider, mover))| {
                if entity.id() == self.player_entity_id {
                    None
                } else {
                    let position = mover.get_position(current_time);
                    Some(collider.get_collision_box(position))
                }
            })
            .collect::<Vec<_>>()
    }
}
