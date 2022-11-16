use crate::{
    app::game::{Collider, CollisionInfo, Game, PlayerCharacter, Positioner, RigidBody, Tile},
    ecs,
};
use namui::prelude::*;

const MAX_COLLISION_RESOLVE_COUNT: i32 = 6;

impl Game {
    pub fn resolve_collision_about_character(&mut self) {
        let rigid_body_list_except_character = get_rigid_body_list_except_character(&self.ecs_app);
        if let Some((_, character_collider, character_positioner)) = self
            .ecs_app
            .query_component_mut::<(PlayerCharacter, Collider, Positioner)>()
            .first_mut()
        {
            let mut character_rigid_body =
                character_collider.get_rigid_body(character_positioner.xy);
            let mut collision_resolve_count = 0;
            while collision_resolve_count < MAX_COLLISION_RESOLVE_COUNT {
                let mut no_collision_detected = true;
                for other_rigid_body in rigid_body_list_except_character.iter() {
                    if let CollisionInfo::Collided {
                        penetration_depth,
                        counter_penetration_vector,
                    } = CollisionInfo::min_by_penetration_depth(
                        character_rigid_body.collide(&other_rigid_body),
                        other_rigid_body
                            .collide(&character_rigid_body)
                            .reverse_collision_normal(),
                    ) {
                        no_collision_detected = false;
                        collision_resolve_count += 1;
                        move_back_by_penetration_depth(
                            penetration_depth,
                            counter_penetration_vector,
                            character_positioner,
                        );
                        character_rigid_body =
                            character_collider.get_rigid_body(character_positioner.xy);
                    }
                }
                if no_collision_detected {
                    break;
                }
            }
        }
    }
}

fn get_rigid_body_list_except_character(ecs_app: &ecs::App) -> Vec<RigidBody> {
    ecs_app
        .query_component::<(Collider, Positioner, Option<PlayerCharacter>)>()
        .into_iter()
        .filter(|(_, _, player_character)| player_character.is_none())
        .map(|(collider, positioner, _)| collider.get_rigid_body(positioner.xy))
        .collect::<Vec<_>>()
}

fn move_back_by_penetration_depth(
    penetration_depth: Tile,
    counter_penetration_vector: Xy<Tile>,
    character_positioner: &mut Positioner,
) {
    let counter_penetration_xy = counter_penetration_vector * penetration_depth.as_f32();
    character_positioner.xy = character_positioner.xy + counter_penetration_xy
}
