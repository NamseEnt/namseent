use crate::{
    app::game::{
        known_id::object::PLAYER_CHARACTER, Collider, CollisionBox, Game, PlayerCharacter,
        Positioner, Tile, TileExt,
    },
    ecs,
};
use namui::prelude::*;

impl Game {
    pub fn resolve_collision_about_character(&mut self) {
        let collision_box_list_except_character =
            get_collision_box_list_except_character(&self.ecs_app);
        if let Some((_, (_, character_collider, character_positioner))) = self
            .ecs_app
            .query_entities_mut::<(&PlayerCharacter, &Collider, &mut Positioner)>()
            .first_mut()
        {
            for other_collision_box in collision_box_list_except_character {
                let character_collision_box =
                    character_collider.get_collision_box(character_positioner.xy());
                if !detect_collision(character_collision_box, other_collision_box) {
                    continue;
                }

                let (collision_normal, penetration_depth) =
                    get_collision_normal_and_penetration_depth(
                        character_collision_box,
                        other_collision_box,
                    );
                move_back_by_penetration_depth(
                    collision_normal,
                    penetration_depth,
                    character_positioner,
                );
            }
        }
    }
}

fn get_collision_box_list_except_character(ecs_app: &ecs::App) -> Vec<CollisionBox> {
    ecs_app
        .query_entities::<(&Collider, &Positioner)>()
        .into_iter()
        .filter_map(
            |(entity, (collider, positioner))| match entity.id() == PLAYER_CHARACTER {
                true => None,
                false => Some(collider.get_collision_box(positioner.xy())),
            },
        )
        .collect::<Vec<_>>()
}

fn detect_collision(
    character_collision_box: CollisionBox,
    other_collision_box: CollisionBox,
) -> bool {
    character_collision_box
        .left()
        .le(&other_collision_box.right())
        && character_collision_box
            .right()
            .ge(&other_collision_box.left())
        && character_collision_box
            .top()
            .le(&other_collision_box.bottom())
        && character_collision_box
            .bottom()
            .ge(&other_collision_box.top())
}

#[derive(Debug, Clone, Copy)]
enum CollisionNormal {
    Left,
    Up,
    Right,
    Down,
}
fn get_collision_normal_and_penetration_depth(
    character_collision_box: CollisionBox,
    other_collision_box: CollisionBox,
) -> (CollisionNormal, Tile) {
    let vector_character_to_target =
        other_collision_box.center() - character_collision_box.center();
    let overlap_x = match vector_character_to_target.x.is_sign_positive() {
        true => character_collision_box.right() - other_collision_box.left(),
        false => other_collision_box.right() - character_collision_box.left(),
    };
    let overlap_y = match vector_character_to_target.y.is_sign_positive() {
        true => character_collision_box.bottom() - other_collision_box.top(),
        false => other_collision_box.bottom() - character_collision_box.top(),
    };
    match overlap_x < overlap_y {
        true => match vector_character_to_target.x.is_sign_positive() {
            true => (CollisionNormal::Right, overlap_x),
            false => (CollisionNormal::Left, overlap_x),
        },
        false => match vector_character_to_target.y.is_sign_positive() {
            true => (CollisionNormal::Down, overlap_y),
            false => (CollisionNormal::Up, overlap_y),
        },
    }
}

fn move_back_by_penetration_depth(
    collision_normal: CollisionNormal,
    penetration_depth: Tile,
    character_positioner: &mut Positioner,
) {
    let counter_penetration_xy = match collision_normal {
        CollisionNormal::Left => Xy {
            x: penetration_depth,
            y: 0.tile(),
        },
        CollisionNormal::Up => Xy {
            x: 0.tile(),
            y: penetration_depth,
        },
        CollisionNormal::Right => Xy {
            x: -penetration_depth,
            y: 0.tile(),
        },
        CollisionNormal::Down => Xy {
            x: 0.tile(),
            y: -penetration_depth,
        },
    };
    character_positioner.set_xy(character_positioner.xy() + counter_penetration_xy)
}
