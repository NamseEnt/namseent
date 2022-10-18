use crate::{
    app::game::{
        known_id::object::PLAYER_CHARACTER, simplify_collision_box_list, zero_speed, Collider,
        CollisionBox, MovementPath, PlayerCharacter, Positioner, TileExt, Velocity,
    },
    ecs,
};
use float_cmp::{ApproxEq, F32Margin};
use namui::prelude::*;

const COLLISION_MARGIN: F32Margin = F32Margin {
    ulps: 1,
    epsilon: 0.0001,
};

pub fn calculate_next_movement_of_character(ecs_app: &mut ecs::App, max_duration: Time) {
    let target_collider_list = ecs_app.query_entities::<(&Collider, &Positioner)>();
    let target_collision_box_list = simplify_collision_box_list(
        target_collider_list
            .iter()
            .filter_map(|(target_entity, (target_collider, target_positioner))| {
                if target_entity.id() == PLAYER_CHARACTER {
                    None
                } else {
                    Some(target_collider.get_collision_box(target_positioner.xy(0.ms())))
                }
            })
            .collect(),
    );

    if let Some((_character_entity, (character, character_collider, character_positioner))) =
        ecs_app
            .query_entities_mut::<(&PlayerCharacter, &Collider, &mut Positioner)>()
            .first_mut()
    {
        let current_time = character_positioner.movement_end_time();
        let character_xy = character_positioner.xy(current_time);
        let mut velocity = character.velocity();
        let mut end_time = current_time + max_duration;
        let character_collision_box = character_collider.get_collision_box(character_xy);

        resolve_collision(
            &mut velocity,
            &mut end_time,
            current_time,
            character_collision_box,
            &target_collision_box_list,
        );

        let end_time = match predict_remaining_collision_start_time(
            velocity,
            character_collision_box,
            &target_collision_box_list,
        ) {
            Some(remaining_collision_start_time) => {
                Time::min(end_time, current_time + remaining_collision_start_time)
            }
            None => end_time,
        };
        let delta_time = end_time - current_time;
        let delta_xy = Xy {
            x: velocity.x * delta_time,
            y: velocity.y * delta_time,
        };
        let end_xy = character_xy + delta_xy;
        let next_movement_path = MovementPath::new(current_time, end_time, character_xy, end_xy);

        character_positioner.push_movement_path(next_movement_path);
    }
}

fn resolve_collision(
    velocity: &mut Velocity,
    end_time: &mut Time,
    current_time: Time,
    character_collision_box: CollisionBox,
    target_collision_box_list: &Vec<CollisionBox>,
) {
    for target_collision_box in target_collision_box_list {
        if !detect_collision(character_collision_box, *target_collision_box) {
            continue;
        }
        if let Some(collision_normal) =
            get_collision_normal(character_collision_box, *target_collision_box)
        {
            if will_character_no_longer_collide(*velocity, collision_normal) {
                continue;
            }
            restrict_velocity(velocity, collision_normal);
            restrict_end_time(
                end_time,
                current_time,
                *velocity,
                character_collision_box,
                *target_collision_box,
            );
        }
    }
}

fn detect_collision(character: CollisionBox, target: CollisionBox) -> bool {
    character.left().approx_le(target.right(), COLLISION_MARGIN)
        && character.right().approx_ge(target.left(), COLLISION_MARGIN)
        && character.top().approx_le(target.bottom(), COLLISION_MARGIN)
        && character.bottom().approx_ge(target.top(), COLLISION_MARGIN)
}

#[derive(Debug, Clone, Copy)]
enum CollisionNormal {
    Left,
    Up,
    Right,
    Down,
}
fn get_collision_normal(character: CollisionBox, target: CollisionBox) -> Option<CollisionNormal> {
    let vector_character_to_target = target.center() - character.center();
    let overlap_x = match vector_character_to_target.x.is_sign_positive() {
        true => character.right() - target.left(),
        false => target.right() - character.left(),
    };
    let overlap_y = match vector_character_to_target.y.is_sign_positive() {
        true => character.bottom() - target.top(),
        false => target.bottom() - character.top(),
    };
    let collision_can_be_ignored = overlap_x.approx_eq(0.tile(), COLLISION_MARGIN)
        && overlap_y.approx_eq(0.tile(), COLLISION_MARGIN);
    if collision_can_be_ignored {
        return None;
    }
    if overlap_x < overlap_y {
        if vector_character_to_target.x.is_sign_positive() {
            Some(CollisionNormal::Right)
        } else {
            Some(CollisionNormal::Left)
        }
    } else {
        if vector_character_to_target.y.is_sign_positive() {
            Some(CollisionNormal::Down)
        } else {
            Some(CollisionNormal::Up)
        }
    }
}

fn restrict_velocity(velocity: &mut Velocity, collision_normal: CollisionNormal) {
    match collision_normal {
        CollisionNormal::Left => {
            if velocity.x.is_sign_negative() {
                velocity.x = zero_speed()
            }
        }
        CollisionNormal::Up => {
            if velocity.y.is_sign_negative() {
                velocity.y = zero_speed()
            }
        }
        CollisionNormal::Right => {
            if velocity.x.is_sign_positive() {
                velocity.x = zero_speed()
            }
        }
        CollisionNormal::Down => {
            if velocity.y.is_sign_positive() {
                velocity.y = zero_speed()
            }
        }
    }
}

fn will_character_no_longer_collide(velocity: Velocity, collision_normal: CollisionNormal) -> bool {
    match collision_normal {
        CollisionNormal::Left => {
            if velocity.x.is_sign_positive() {
                return true;
            }
        }
        CollisionNormal::Up => {
            if velocity.y.is_sign_positive() {
                return true;
            }
        }
        CollisionNormal::Right => {
            if velocity.x.is_sign_negative() {
                return true;
            }
        }
        CollisionNormal::Down => {
            if velocity.y.is_sign_negative() {
                return true;
            }
        }
    }
    false
}

fn restrict_end_time(
    end_time: &mut Time,
    current_time: Time,
    velocity: Velocity,
    character: CollisionBox,
    target: CollisionBox,
) {
    let collision_duration_of_x = velocity.x.invert()
        * match velocity.x.is_sign_positive() {
            true => target.right() - character.left(),
            false => target.left() - character.right(),
        };
    let collision_duration_of_y = velocity.y.invert()
        * match velocity.y.is_sign_positive() {
            true => target.bottom() - character.top(),
            false => target.left() - character.bottom(),
        };
    let collision_duration = Time::min(collision_duration_of_x, collision_duration_of_y);
    if collision_duration.as_millis() > 0.0 {
        *end_time = Time::min(*end_time, current_time + collision_duration);
    }
}

fn predict_remaining_collision_start_time(
    velocity: Velocity,
    character_collision_box: CollisionBox,
    target_collision_box_list: &Vec<CollisionBox>,
) -> Option<Time> {
    let remaining_collision_start_time_list =
        target_collision_box_list
            .iter()
            .filter_map(|target_collision_box| {
                if detect_collision(character_collision_box, *target_collision_box) {
                    return None;
                }
                let (remaining_collision_start_time_of_x, remaining_collision_end_time_of_x) =
                    match velocity.x.is_sign_positive() {
                        true => (
                            velocity.x.invert()
                                * (target_collision_box.left() - character_collision_box.right()),
                            velocity.x.invert()
                                * (target_collision_box.right() - character_collision_box.left()),
                        ),
                        false => (
                            velocity.x.invert()
                                * (target_collision_box.right() - character_collision_box.left()),
                            velocity.x.invert()
                                * (target_collision_box.left() - character_collision_box.right()),
                        ),
                    };
                let (remaining_collision_start_time_of_y, remaining_collision_end_time_of_y) =
                    match velocity.y.is_sign_positive() {
                        true => (
                            velocity.y.invert()
                                * (target_collision_box.top() - character_collision_box.bottom()),
                            velocity.y.invert()
                                * (target_collision_box.bottom() - character_collision_box.top()),
                        ),
                        false => (
                            velocity.y.invert()
                                * (target_collision_box.bottom() - character_collision_box.top()),
                            velocity.y.invert()
                                * (target_collision_box.top() - character_collision_box.bottom()),
                        ),
                    };
                let remaining_collision_start_time = Time::max(
                    remaining_collision_start_time_of_x,
                    remaining_collision_start_time_of_y,
                );
                let remaining_collision_end_time = Time::min(
                    remaining_collision_end_time_of_x,
                    remaining_collision_end_time_of_y,
                );
                let will_collide_at_future = remaining_collision_start_time.as_millis() > 0.0
                    && remaining_collision_start_time < remaining_collision_end_time;

                match will_collide_at_future {
                    true => Some(remaining_collision_start_time),
                    false => None,
                }
            });

    remaining_collision_start_time_list.min()
}
