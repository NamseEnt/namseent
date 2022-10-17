use crate::{
    app::game::{
        known_id::object::PLAYER_CHARACTER, zero_speed, Collider, CollisionBox, MovementPath,
        PlayerCharacter, Positioner, Velocity,
    },
    ecs,
};
use float_cmp::{ApproxEq, F32Margin};
use namui::prelude::*;

const COLLISION_MARGIN: F32Margin = F32Margin {
    ulps: 1,
    epsilon: 0.00001,
};
const TIME_MS_MARGIN: F32Margin = F32Margin {
    ulps: 1,
    epsilon: 0.01,
};

pub fn calculate_next_movement_of_character(ecs_app: &mut ecs::App, max_duration: Time) {
    let target_collider_list = ecs_app.query_entities::<(&Collider, &Positioner)>();
    let target_collision_box_list: Vec<_> = target_collider_list
        .iter()
        .filter_map(|(target_entity, (target_collider, target_positioner))| {
            if target_entity.id() == PLAYER_CHARACTER {
                None
            } else {
                Some(target_collider.get_collision_box(target_positioner.xy(0.ms())))
            }
        })
        .collect();

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
        let collision_normal = get_collision_normal(character_collision_box, *target_collision_box);

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
fn get_collision_normal(character: CollisionBox, target: CollisionBox) -> CollisionNormal {
    let vector_character_to_target = target.center() - character.center();
    let overlap_x = match vector_character_to_target.x.is_sign_positive() {
        true => character.right() - target.left(),
        false => target.right() - character.left(),
    };
    let overlap_y = match vector_character_to_target.y.is_sign_positive() {
        true => character.bottom() - target.top(),
        false => target.bottom() - character.top(),
    };
    if overlap_x < overlap_y {
        if vector_character_to_target.x.is_sign_positive() {
            CollisionNormal::Right
        } else {
            CollisionNormal::Left
        }
    } else {
        if vector_character_to_target.y.is_sign_positive() {
            CollisionNormal::Down
        } else {
            CollisionNormal::Up
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
            false => character.right() - target.left(),
        };
    let collision_duration_of_y = velocity.y.invert()
        * match velocity.y.is_sign_positive() {
            true => target.bottom() - character.top(),
            false => character.bottom() - target.top(),
        };
    let collision_duration = Time::min(collision_duration_of_x, collision_duration_of_y);
    let collision_duration_approx_less_or_equal_to_zero = collision_duration.as_millis() < 0.0
        || collision_duration
            .as_millis()
            .approx_eq(0.0, TIME_MS_MARGIN);
    if collision_duration_approx_less_or_equal_to_zero {
        return;
    }
    *end_time = Time::min(*end_time, current_time + collision_duration);
}

fn predict_remaining_collision_start_time(
    velocity: Velocity,
    character_collision_box: CollisionBox,
    target_collision_box_list: &Vec<CollisionBox>,
) -> Option<Time> {
    let collision_prediction_list =
        target_collision_box_list
            .iter()
            .filter_map(|target_collision_box| {
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
                let will_collide_at_future = remaining_collision_start_time
                    < remaining_collision_end_time
                    && remaining_collision_start_time.as_millis() > 0.0
                    && remaining_collision_start_time
                        .as_millis()
                        .approx_ne(0.0, TIME_MS_MARGIN);

                match will_collide_at_future {
                    true => Some(remaining_collision_start_time),
                    false => None,
                }
            });

    collision_prediction_list.min()
}
