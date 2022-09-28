use super::*;
use crate::app::game::{
    predict_collision,
    types::game_object::player_character::types::{
        get_heading_from_velocity, CollisionDirection, Heading,
    },
    TileExt, Velocity,
};
use namui::prelude::*;

#[derive(ecs_macro::Component)]
pub struct Mover {
    movement_plan: MovementPlan,
    pub heading: Heading,
}

impl Mover {
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
        collision_box_list: &[crate::app::game::CollisionBox],
    ) {
        let prediction_start_time = self.get_predicted_movement_end_time();
        let prediction_start_position = self.get_position(prediction_start_time);
        let character_collision_box = collider.get_collision_box(prediction_start_position);

        let directed_movement_ended =
            prediction_start_time == self.movement_plan.directed_movement.end_time;
        if directed_movement_ended {
            self.movement_plan
                .predicted_movement_list
                .push(Movement::stay_forever(
                    prediction_start_position,
                    prediction_start_time,
                ));
            return;
        }

        let first_collision = predict_collision(
            self.movement_plan.directed_movement.velocity,
            &character_collision_box,
            &collision_box_list,
            prediction_start_time,
            self.movement_plan.directed_movement.end_time,
            prediction_start_position,
        );
        match first_collision {
            Some(first_collision) => {
                let movement_until_first_collision = Movement {
                    start_time: prediction_start_time,
                    end_time: first_collision.start_time,
                    start_position: prediction_start_position,
                    end_position: first_collision.start_position,
                    velocity: self.movement_plan.directed_movement.velocity,
                };
                self.movement_plan
                    .predicted_movement_list
                    .push(movement_until_first_collision);

                let character_collision_box_at_first_collision =
                    collider.get_collision_box(self.get_position(first_collision.start_time));
                let velocity_after_first_collision = match first_collision.direction {
                    CollisionDirection::Vertical => Xy {
                        x: self.movement_plan.directed_movement.velocity.x,
                        y: Per::new(0.tile(), 1.ms()),
                    },
                    CollisionDirection::Horizontal => Xy {
                        x: Per::new(0.tile(), 1.ms()),
                        y: self.movement_plan.directed_movement.velocity.y,
                    },
                };
                let second_collision = predict_collision(
                    velocity_after_first_collision,
                    &character_collision_box_at_first_collision,
                    &collision_box_list,
                    first_collision.start_time,
                    first_collision.end_time,
                    first_collision.start_position,
                );
                match second_collision {
                    Some(second_collision) => {
                        let movement_until_second_collision = Movement {
                            start_time: first_collision.start_time,
                            end_time: second_collision.start_time,
                            start_position: first_collision.start_position,
                            end_position: second_collision.start_position,
                            velocity: velocity_after_first_collision,
                        };
                        let movement_after_second_collision = Movement::stay_forever(
                            movement_until_second_collision
                                .get_position(movement_until_second_collision.end_time)
                                .unwrap(),
                            movement_until_second_collision.end_time,
                        );
                        self.movement_plan.predicted_movement_list.extend([
                            movement_until_second_collision,
                            movement_after_second_collision,
                        ]);
                    }
                    None => {
                        let movement_after_first_collision_delta_time =
                            first_collision.end_time - first_collision.start_time;
                        let movement_after_first_collision = Movement {
                            start_time: first_collision.start_time,
                            end_time: first_collision.end_time,
                            start_position: first_collision.start_position,
                            velocity: velocity_after_first_collision,
                            end_position: first_collision.start_position
                                + Xy {
                                    x: velocity_after_first_collision.x
                                        * movement_after_first_collision_delta_time,
                                    y: velocity_after_first_collision.y
                                        * movement_after_first_collision_delta_time,
                                },
                        };
                        self.movement_plan
                            .predicted_movement_list
                            .push(movement_after_first_collision);
                    }
                }
            }
            None => {
                let movement_until_end = Movement {
                    start_time: prediction_start_time,
                    end_time: self.movement_plan.directed_movement.end_time,
                    start_position: prediction_start_position,
                    velocity: self.movement_plan.directed_movement.velocity,
                    end_position: self.movement_plan.directed_movement.end_position,
                };
                self.movement_plan
                    .predicted_movement_list
                    .push(movement_until_end);
            }
        };
    }

    pub fn get_position(&self, current_time: Time) -> Xy<crate::app::game::Tile> {
        self.movement_plan
            .get_position(current_time)
            .expect("get_position() of PlayerCharacter called before prediction")
    }
}
