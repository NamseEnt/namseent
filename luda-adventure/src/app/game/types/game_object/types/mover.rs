use super::CollisionBox;
use crate::app::game::{GameObject, Velocity};
use namui::prelude::*;

pub trait Mover: GameObject {
    fn set_velocity(&mut self, current_time: Time, velocity: Velocity, duration: Time);
    fn get_predicted_movement_end_time(&self) -> Time;
    fn predict_movement(&mut self, collision_box_list: &[CollisionBox]);
}
