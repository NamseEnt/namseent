use crate::{
    app::game::{
        known_id::object::PLAYER_CHARACTER, Collider, CollisionInfo, Game, PlayerCharacter,
        Positioner, RigidBody, Tile,
    },
    ecs,
};
use namui::prelude::*;

impl Game {
    pub fn resolve_collision_about_character(&mut self) {
        let rigid_body_list_except_character = get_rigid_body_list_except_character(&self.ecs_app);
        if let Some((_, (_, character_collider, character_positioner))) = self
            .ecs_app
            .query_entities_mut::<(&PlayerCharacter, &Collider, &mut Positioner)>()
            .first_mut()
        {
            let mut character_rigid_body =
                character_collider.get_rigid_body(character_positioner.xy());
            for other_rigid_body in rigid_body_list_except_character {
                if let CollisionInfo::Collided {
                    penetration_depth,
                    collision_normal,
                } = CollisionInfo::min_by_penetration_depth(
                    character_rigid_body.collide(&other_rigid_body),
                    other_rigid_body.collide(&character_rigid_body),
                ) {
                    move_back_by_penetration_depth(
                        penetration_depth,
                        collision_normal,
                        character_positioner,
                    );
                    character_rigid_body =
                        character_collider.get_rigid_body(character_positioner.xy());
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
                false => Some(collider.get_rigid_body(positioner.xy())),
            },
        )
        .collect::<Vec<_>>()
}

fn move_back_by_penetration_depth(
    penetration_depth: Tile,
    collision_normal: Xy<Tile>,
    character_positioner: &mut Positioner,
) {
    let counter_penetration_xy = collision_normal * penetration_depth.as_f32();
    character_positioner.set_xy(character_positioner.xy() + counter_penetration_xy)
}
