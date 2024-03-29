use super::{InteractionKind, MAX_INTERACTION_DISTANCE};
use crate::{
    app::game::Tile,
    component::{Interactor, Positioner, Renderer},
    ecs::Entity,
};
use namui::*;

pub fn nearest_entity(
    interactive_object_list: &Vec<((&Entity, (&Interactor, &Positioner, &Renderer)), Tile)>,
) -> Option<(Uuid, InteractionKind)> {
    let mut nearest_entity_id_and_interaction_kind = None;
    let mut nearest_entity_distance = MAX_INTERACTION_DISTANCE;
    for ((entity, (interactor, _, _)), distance) in interactive_object_list {
        if distance < &nearest_entity_distance {
            nearest_entity_id_and_interaction_kind = Some((entity.id(), interactor.kind.clone()));
            nearest_entity_distance = *distance;
        }
    }
    nearest_entity_id_and_interaction_kind
}
