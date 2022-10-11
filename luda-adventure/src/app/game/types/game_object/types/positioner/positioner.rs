use super::{
    get_heading_from_velocity, predict_collision, CollisionDirection, CollisionState, Heading,
    Movement, MovementPlan, Velocity,
};
use crate::app::game::{Collider, TileExt};
use namui::prelude::*;

#[derive(ecs_macro::Component)]
pub struct Positioner {
    movement_plan: MovementPlan,
    pub heading: Heading,
}

impl Positioner {
    pub fn new(movement_plan: MovementPlan) -> Self {
        Self {
            movement_plan,
            heading: Heading::Right,
        }
    }
    pub fn set_velocity(&mut self, current_time: Time, velocity: Velocity, duration: Time) {
        let current_position = self.get_position(current_time);
        self.movement_plan =
            MovementPlan::move_now(current_position, current_time, duration, velocity);
        if let Some(heading) = get_heading_from_velocity(velocity) {
            self.heading = heading;
        }
    }
    pub fn get_predicted_movement_end_time(&self) -> Time {
        self.movement_plan.get_predicted_movement_end_time()
    }
    pub fn predict_movement(
        &mut self,
        collider: &Collider,
        target_collision_box_list: &[crate::app::game::CollisionBox],
    ) {
        let last_prediction = self.get_last_prediction();
        let prediction_start_time = last_prediction.end_time;
        let prediction_start_position = last_prediction.end_position;
        let character_collision_box = collider.get_collision_box(prediction_start_position);
        let original_movement = &self.movement_plan.original_movement;

        let original_movement_ended = prediction_start_time == original_movement.end_time;
        if original_movement_ended {
            self.movement_plan
                .predicted_movement_list
                .push(super::Movement::stay_forever(
                    prediction_start_position,
                    prediction_start_time,
                ));
            return;
        }

        match last_prediction.collision_state {
            super::CollisionState::MoveAlongAxis | super::CollisionState::FreeMove => {
                if let Some(collision) = predict_collision(
                    original_movement.velocity,
                    &character_collision_box,
                    &target_collision_box_list,
                    prediction_start_time,
                    original_movement.end_time,
                    prediction_start_position,
                ) {
                    let movement_until_collision = Movement {
                        start_time: prediction_start_time,
                        end_time: collision.start_time,
                        start_position: prediction_start_position,
                        end_position: collision.start_position,
                        velocity: original_movement.velocity,
                        collision_state: CollisionState::MoveToCollide(collision),
                    };
                    self.movement_plan
                        .predicted_movement_list
                        .push(movement_until_collision);
                } else {
                    let movement_until_original_movement_end = Movement {
                        start_time: prediction_start_time,
                        end_time: original_movement.end_time,
                        start_position: prediction_start_position,
                        velocity: original_movement.velocity,
                        end_position: original_movement.end_position,
                        collision_state: CollisionState::FreeMove,
                    };
                    self.movement_plan
                        .predicted_movement_list
                        .push(movement_until_original_movement_end);
                }
            }
            super::CollisionState::MoveToCollide(last_collision) => {
                let velocity_after_last_collision = match last_collision.direction {
                    CollisionDirection::Vertical => Xy {
                        x: original_movement.velocity.x,
                        y: Per::new(0.tile(), 1.ms()),
                    },
                    CollisionDirection::Horizontal => Xy {
                        x: Per::new(0.tile(), 1.ms()),
                        y: original_movement.velocity.y,
                    },
                };

                if let Some(collision) = predict_collision(
                    velocity_after_last_collision,
                    &character_collision_box,
                    &target_collision_box_list,
                    prediction_start_time,
                    original_movement.end_time,
                    prediction_start_position,
                ) {
                    let movement_until_collision = Movement {
                        start_time: prediction_start_time,
                        end_time: collision.start_time,
                        start_position: prediction_start_position,
                        end_position: collision.start_position,
                        velocity: velocity_after_last_collision,
                        collision_state: CollisionState::MoveAlongAxisToCollide,
                    };
                    self.movement_plan
                        .predicted_movement_list
                        .push(movement_until_collision);
                } else {
                    let delta_time = last_collision.end_time - last_collision.start_time;
                    let end_position = prediction_start_position
                        + Xy {
                            x: velocity_after_last_collision.x * delta_time,
                            y: velocity_after_last_collision.y * delta_time,
                        };
                    let movement_until_last_collision_end = Movement {
                        start_time: prediction_start_time,
                        end_time: last_collision.end_time,
                        start_position: prediction_start_position,
                        velocity: velocity_after_last_collision,
                        end_position,
                        collision_state: CollisionState::MoveAlongAxis,
                    };
                    self.movement_plan
                        .predicted_movement_list
                        .push(movement_until_last_collision_end);
                }
            }
            super::CollisionState::MoveAlongAxisToCollide | super::CollisionState::Stuck => {
                let stay_forever =
                    Movement::stay_forever(prediction_start_position, prediction_start_time);
                self.movement_plan
                    .predicted_movement_list
                    .push(stay_forever);
            }
        }
    }

    pub fn get_position(&self, current_time: Time) -> Xy<crate::app::game::Tile> {
        self.movement_plan
            .get_position(current_time)
            .expect("get_position() of PlayerCharacter called before prediction")
    }

    fn get_last_prediction(&self) -> Movement {
        match self.movement_plan.predicted_movement_list.last() {
            Some(movement) => *movement,
            None => Movement {
                start_time: (f32::NEG_INFINITY).ms(),
                end_time: self.movement_plan.original_movement.start_time,
                start_position: self.movement_plan.original_movement.start_position,
                end_position: self.movement_plan.original_movement.end_position,
                velocity: self.movement_plan.original_movement.velocity,
                collision_state: self.movement_plan.original_movement.collision_state.clone(),
            },
        }
    }
}
