use super::MAX_INTERACTION_DISTANCE;
use crate::{
    app::game::Tile,
    component::{Positioner, Renderer},
    ecs::Entity,
};
use namui::prelude::*;

pub fn nearest_entity_id(
    interactive_object_list: &Vec<((&Entity, (&Positioner, &Renderer)), Tile)>,
) -> Option<Uuid> {
    let mut nearest_entity_id = None;
    let mut nearest_entity_distance = MAX_INTERACTION_DISTANCE;
    for ((entity, _), distance) in interactive_object_list {
        if distance < &nearest_entity_distance {
            nearest_entity_id = Some(entity.id());
            nearest_entity_distance = *distance;
        }
    }
    nearest_entity_id
}
