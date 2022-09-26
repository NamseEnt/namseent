use super::{
    get_heading_from_velocity, predict_collision, CollisionDirection, Heading, Movement,
    MovementPlan,
};
use crate::app::game::{known_id, Collider, GameObject, Mover, Position, Tile, TileExt, Velocity};
use namui::prelude::*;
use namui_prebuilt::simple_rect;

const COLLISION_WIDTH: Tile = Tile(3.0);
const COLLISION_HEIGHT: Tile = Tile(3.0);
const COLLISION_OFFSET_X: Tile = Tile(-1.5);
const COLLISION_OFFSET_Y: Tile = Tile(-1.5);
const VISUAL_WIDTH: Tile = Tile(3.0);
const VISUAL_HEIGHT: Tile = Tile(4.0);
const VISUAL_OFFSET_X: Tile = Tile(-1.5);
const VISUAL_OFFSET_Y: Tile = Tile(-2.5);

pub struct PlayerCharacter {
    movement_plan: MovementPlan,
    heading: Heading,
}
impl PlayerCharacter {
    pub fn new(position: Position, current_time: Time) -> Self {
        Self {
            movement_plan: MovementPlan::stay_forever(position, current_time),
            heading: Heading::Right,
        }
    }
}
impl GameObject for PlayerCharacter {
    fn get_id(&self) -> Uuid {
        known_id::object::PLAYER_CHARACTER_OBJECT
    }

    fn render(
        &self,
        _game_context: &crate::app::game::GameState,
        rendering_context: &crate::app::game::RenderingContext,
    ) -> namui::RenderingTree {
        let position = self.get_position(rendering_context.current_time);
        translate(
            rendering_context.px_per_tile * (position.x + VISUAL_OFFSET_X),
            rendering_context.px_per_tile * (position.y + VISUAL_OFFSET_Y),
            render([
                simple_rect(
                    Wh {
                        width: rendering_context.px_per_tile * VISUAL_WIDTH,
                        height: rendering_context.px_per_tile * VISUAL_HEIGHT,
                    },
                    Color::TRANSPARENT,
                    0.px(),
                    Color::from_f01(0.5, 0.5, 1.0, 0.5),
                ),
                namui_prebuilt::typography::center_text(
                    Wh {
                        width: rendering_context.px_per_tile * VISUAL_WIDTH,
                        height: rendering_context.px_per_tile * VISUAL_HEIGHT,
                    },
                    match self.heading {
                        Heading::Left => "L",
                        Heading::Right => "R",
                    },
                    Color::WHITE,
                ),
            ]),
        )
    }

    fn get_position(&self, current_time: Time) -> Position {
        self.movement_plan
            .get_position(current_time)
            .expect("get_position() of PlayerCharacter called before prediction")
    }

    fn get_z_index(&self) -> i32 {
        0
    }

    fn get_visual_area(&self, current_time: Time) -> crate::app::game::VisualArea {
        let position = self.get_position(current_time);
        Rect::Xywh {
            x: position.x + VISUAL_OFFSET_X,
            y: position.y + VISUAL_OFFSET_Y,
            width: VISUAL_WIDTH,
            height: VISUAL_HEIGHT,
        }
    }

    fn get_mover(&mut self) -> Option<&mut dyn crate::app::game::Mover> {
        Some(self)
    }

    fn get_collider(&mut self) -> Option<&mut dyn crate::app::game::Collider> {
        Some(self)
    }
}
impl Collider for PlayerCharacter {
    fn get_collision_box(&self, curent_time: Time) -> crate::app::game::CollisionBox {
        let position = self.get_position(curent_time);
        Rect::Xywh {
            x: position.x + COLLISION_OFFSET_X,
            y: position.y + COLLISION_OFFSET_Y,
            width: COLLISION_WIDTH,
            height: COLLISION_HEIGHT,
        }
        .into()
    }
}
impl Mover for PlayerCharacter {
    fn set_velocity(&mut self, current_time: Time, velocity: Velocity, duration: Time) {
        let current_position = self.get_position(current_time);
        self.movement_plan =
            MovementPlan::move_now(current_position, current_time, duration, velocity);
        if let Some(heading) = get_heading_from_velocity(velocity) {
            self.heading = heading;
        }
    }

    fn get_predicted_movement_end_time(&self) -> Time {
        self.movement_plan
            .predicted_movement_list
            .last()
            .map(|last_predicated_movement| last_predicated_movement.end_time)
            .unwrap_or(self.movement_plan.directed_movement.start_time)
    }

    fn predict_movement(&mut self, collision_box_list: &[crate::app::game::CollisionBox]) {
        let prediction_start_time = self.get_predicted_movement_end_time();
        let prediction_start_position = self.get_position(prediction_start_time);
        let character_collision_box = self.get_collision_box(prediction_start_time);

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
                    self.get_collision_box(first_collision.start_time);
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
}
