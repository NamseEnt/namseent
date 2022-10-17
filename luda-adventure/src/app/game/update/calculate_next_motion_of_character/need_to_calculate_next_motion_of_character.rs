use crate::{
    app::game::{PlayerCharacter, Positioner},
    ecs,
};
use namui::prelude::*;

pub fn need_to_calculate_next_motion_of_character(
    ecs_app: &ecs::App,
    current_time: Time,
    near_future: Time,
) -> bool {
    if let Some((_entity, (_character, positioner))) = ecs_app
        .query_entities::<(&PlayerCharacter, &Positioner)>()
        .into_iter()
        .next()
    {
        let movement_end_time = positioner.movement_end_time();
        let need_to_calculate_next_movement = movement_end_time - current_time < near_future;
        need_to_calculate_next_movement
    } else {
        false
    }
}
