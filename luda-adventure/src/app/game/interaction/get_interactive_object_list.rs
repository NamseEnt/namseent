use crate::{
    app::game::{tile, Game, Tile},
    component::{Interactor, PlayerCharacter, Positioner, Renderer},
    ecs::Entity,
};

pub const MAX_INTERACTION_DISTANCE: Tile = tile(4.0);

impl Game {
    pub fn get_interactive_object_with_distance(
        &self,
    ) -> Vec<((&Entity, (&Interactor, &Positioner, &Renderer)), Tile)> {
        let Some((_, (_, character_positioner))) = self
            .ecs_app
            .query_entities::<(&PlayerCharacter, &Positioner)>()
            .into_iter()
            .next() else {
            return vec![];
        };
        let character_position = character_positioner.xy;

        let interactive_objects = self
            .ecs_app
            .query_entities::<(&Interactor, &Positioner, &Renderer)>();
        interactive_objects
            .into_iter()
            .map(|(entity, (interactor, positioner, renderer))| {
                let distance = (character_position - positioner.xy).length();
                ((entity, (interactor, positioner, renderer)), distance)
            })
            .collect()
    }
}
