use super::{Game};
use namui::prelude::*;

impl Game<'_> {
    pub fn predict_character_movement_if_needed(&mut self, _current_time: Time) {
        todo!()
        // let collision_box_list = get_collision_box_list_without_character_collision_box(
        //     &mut self.object_list,
        //     current_time,
        // );
        // if let Some(character) = self
        //     .object_list
        //     .iter_mut()
        //     .find(|game_object| game_object.get_id() == known_id::object::PLAYER_CHARACTER_OBJECT)
        // {
        //     if let Some(character) = character.get_mover() {
        //         while character.get_predicted_movement_end_time() - current_time < 1.sec() {
        //             character.predict_movement(collision_box_list.as_ref());
        //         }
        //     }
        // }
    }
}
