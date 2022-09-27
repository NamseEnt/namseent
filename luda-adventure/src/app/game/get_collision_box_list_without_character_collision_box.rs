use super::{CollisionBox, GameObject};
use namui::prelude::*;

pub fn get_collision_box_list_without_character_collision_box(
    _object_list: &mut Vec<Box<dyn GameObject>>,
    _current_time: Time,
) -> Vec<CollisionBox> {
    todo!()
    // object_list
    //     .iter_mut()
    //     .filter_map(|game_object| {
    //         if game_object.get_id() == known_id::object::PLAYER_CHARACTER_OBJECT {
    //             None
    //         } else {
    //             game_object.get_collider()
    //         }
    //     })
    //     .map(|collider| collider.get_collision_box(current_time))
    //     .collect::<Vec<_>>()
}
