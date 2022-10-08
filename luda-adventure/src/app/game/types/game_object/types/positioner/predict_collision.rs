use super::{CollisionDirection, CollisionPrediction, IntersectionPrediction};
use crate::app::game::{CollisionBox, Tile, TileExt, Velocity};
use namui::prelude::*;

pub fn predict_collision(
    velocity: Velocity,
    character_collision_box: &CollisionBox,
    target_collision_box_list: &[CollisionBox],
    current_time: Time,
    end_time: Time,
    start_position: Xy<Tile>,
) -> Option<CollisionPrediction> {
    let mut collision_prediction_list = target_collision_box_list
        .iter()
        .filter_map(|target_collision_box| {
            predict_collision_between_character_collision_box_and_others(
                &character_collision_box,
                target_collision_box,
                velocity,
                current_time,
                end_time,
                start_position,
            )
        })
        .filter(|collision_prediction| collision_prediction.start_time >= 0.0.ms())
        .collect::<Vec<_>>();
    collision_prediction_list.sort_by(|a, b| a.start_time.cmp(&b.start_time));
    collision_prediction_list.into_iter().next()
}

fn predict_collision_between_character_collision_box_and_others(
    character_collision_box: &CollisionBox,
    target_collision_box: &CollisionBox,
    velocity: Velocity,
    start_time: Time,
    end_time: Time,
    start_position: Xy<Tile>,
) -> Option<CollisionPrediction> {
    let intersection_prediction_of_x = predict_intersection_on_one_axis(
        velocity.x,
        character_collision_box.left(),
        character_collision_box.right(),
        target_collision_box.left(),
        target_collision_box.right(),
    );
    let intersection_prediction_of_y = predict_intersection_on_one_axis(
        velocity.y,
        character_collision_box.top(),
        character_collision_box.bottom(),
        target_collision_box.top(),
        target_collision_box.bottom(),
    );
    match (intersection_prediction_of_x, intersection_prediction_of_y) {
        (
            IntersectionPrediction::WillIntersect {
                start_remaining_time: intersection_start_remaining_time_of_x,
                end_remaining_time: intersection_end_remaining_time_of_x,
                start_remaining_distance: intersection_start_remaining_distance_of_x,
            },
            IntersectionPrediction::WillIntersect {
                start_remaining_time: intersection_start_remaining_time_of_y,
                end_remaining_time: intersection_end_remaining_time_of_y,
                start_remaining_distance: intersection_start_remaining_distance_of_y,
            },
        ) => {
            let collision_direction = match intersection_start_remaining_time_of_x
                > intersection_start_remaining_time_of_y
            {
                true => CollisionDirection::Horizontal,
                false => CollisionDirection::Vertical,
            };
            let (collision_start_remaining_time, collision_end_remaining_time) =
                match collision_direction {
                    CollisionDirection::Horizontal => (
                        intersection_start_remaining_time_of_x,
                        intersection_end_remaining_time_of_y,
                    ),
                    CollisionDirection::Vertical => (
                        intersection_start_remaining_time_of_y,
                        intersection_end_remaining_time_of_x,
                    ),
                };
            let collision_remaining_distance = match collision_direction {
                CollisionDirection::Horizontal => Xy {
                    x: intersection_start_remaining_distance_of_x,
                    y: velocity.y * collision_start_remaining_time,
                },
                CollisionDirection::Vertical => Xy {
                    x: velocity.x * collision_start_remaining_time,
                    y: intersection_start_remaining_distance_of_y,
                },
            };
            let collision_prediction = CollisionPrediction {
                start_time: collision_start_remaining_time + start_time,
                end_time: collision_end_remaining_time + start_time,
                direction: collision_direction,
                start_position: start_position + collision_remaining_distance,
            };
            let collision_reversed =
                collision_prediction.start_time >= collision_prediction.end_time;
            let collision_will_happen_after_end_time = collision_prediction.start_time >= end_time;
            let collision_was_happened_before_start_time =
                collision_prediction.start_time < start_time;
            if collision_reversed
                || collision_will_happen_after_end_time
                || collision_was_happened_before_start_time
            {
                return None;
            }
            Some(collision_prediction)
        }
        (IntersectionPrediction::WillNotIntersect, _)
        | (_, IntersectionPrediction::WillNotIntersect) => None,
    }
}

fn predict_intersection_on_one_axis(
    speed: Per<Tile, Time>,
    character_less_side: Tile,
    character_greater_side: Tile,
    target_less_side: Tile,
    target_greater_side: Tile,
) -> IntersectionPrediction {
    enum MovementDirection {
        Positive,
        Zero,
        Negative,
    }
    let movement_direction = match speed
        .partial_cmp(&Per::new(0.tile(), 1.ms()))
        .expect("Invalid speed")
    {
        std::cmp::Ordering::Less => MovementDirection::Negative,
        std::cmp::Ordering::Equal => MovementDirection::Zero,
        std::cmp::Ordering::Greater => MovementDirection::Positive,
    };
    let target_greater_side_minus_character_less_side = target_greater_side - character_less_side;
    let target_greater_side_minus_character_less_side_divided_by_speed =
        speed.invert() * target_greater_side_minus_character_less_side;
    let target_less_side_minus_character_greater_side = target_less_side - character_greater_side;
    let target_less_side_minus_character_greater_side_divided_by_speed =
        speed.invert() * target_less_side_minus_character_greater_side;
    let intersection_prediction = match movement_direction {
        MovementDirection::Positive => {
            let start_remaining_time =
                target_less_side_minus_character_greater_side_divided_by_speed;
            let end_remaining_time = target_greater_side_minus_character_less_side_divided_by_speed;
            let start_remaining_distance = target_less_side_minus_character_greater_side;
            let will_intersect = start_remaining_time <= end_remaining_time;
            if will_intersect {
                IntersectionPrediction::WillIntersect {
                    start_remaining_time,
                    end_remaining_time,
                    start_remaining_distance,
                }
            } else {
                IntersectionPrediction::WillNotIntersect
            }
        }
        MovementDirection::Zero => {
            let already_intersecting = character_less_side < target_greater_side
                && character_greater_side > target_less_side;
            if already_intersecting {
                IntersectionPrediction::WillIntersect {
                    start_remaining_time: f32::NEG_INFINITY.ms(),
                    end_remaining_time: f32::INFINITY.ms(),
                    start_remaining_distance: 0.0.tile(),
                }
            } else {
                IntersectionPrediction::WillNotIntersect
            }
        }
        MovementDirection::Negative => {
            let start_remaining_time =
                target_greater_side_minus_character_less_side_divided_by_speed;
            let end_remaining_time = target_less_side_minus_character_greater_side_divided_by_speed;
            let start_remaining_distance = target_greater_side_minus_character_less_side;
            let will_intersect = start_remaining_time <= end_remaining_time;
            if will_intersect {
                IntersectionPrediction::WillIntersect {
                    start_remaining_time,
                    end_remaining_time,
                    start_remaining_distance,
                }
            } else {
                IntersectionPrediction::WillNotIntersect
            }
        }
    };
    if let IntersectionPrediction::WillIntersect {
        end_remaining_time, ..
    } = intersection_prediction
    {
        let already_passed = end_remaining_time <= 0.0.ms();
        if already_passed {
            return IntersectionPrediction::WillNotIntersect;
        }
    }
    intersection_prediction
}
