use crate::{
    app::game::{tile, Game, GameState, Tile},
    component::{Interactor, PlayerCharacter, Positioner, Renderer},
    ecs::Entity,
};

pub const MAX_INTERACTION_DISTANCE: Tile = tile(4.0);

impl Game {
    pub fn get_interactive_object_with_distance(
        &self,
        game_state: &GameState,
    ) -> Vec<((&Entity, (&Positioner, &Renderer)), Tile)> {
        let Some((_, (_, character_positioner))) = self
            .ecs_app
            .query_entities::<(&PlayerCharacter, &Positioner)>()
            .into_iter()
            .next() else {
            return vec![];
        };
        let interpolation_progress = game_state.tick.interpolation_progress();
        let character_position = character_positioner.xy_with_interpolation(interpolation_progress);

        let interactive_objects = self
            .ecs_app
            .query_entities::<(&Interactor, &Positioner, &Renderer)>();
        interactive_objects
            .into_iter()
            .map(|(entity, (_, positioner, renderer))| {
                let distance = (character_position
                    - positioner.xy_with_interpolation(interpolation_progress))
                .length();
                ((entity, (positioner, renderer)), distance)
            })
            .collect()
    }
}
