use crate::component::*;
use crate::{
    app::game::{known_id::object::PLAYER_CHARACTER, Game, Tile},
    ecs,
};
use namui::*;

const MAX_COLLISION_RESOLVE_COUNT: i32 = 6;

impl Game {
    pub fn resolve_collision_about_character(&mut self) {
        let rigid_body_list_except_character = get_rigid_body_list_except_character(&self.ecs_app);
        if let Some((_, (_, character_collider, character_positioner))) = self
            .ecs_app
            .query_entities_mut::<(&PlayerCharacter, &Collider, &mut Positioner)>()
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
        .query_entities::<(&Collider, &Positioner)>()
        .into_iter()
        .filter_map(
            |(entity, (collider, positioner))| match entity.id() == PLAYER_CHARACTER {
                true => None,
                false => Some(collider.get_rigid_body(positioner.xy)),
            },
        )
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
