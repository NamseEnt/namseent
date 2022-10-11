use super::CollisionPrediction;
use crate::app::game::{Tile, TileExt, Velocity};
use namui::prelude::*;

#[derive(Clone, Debug, Copy)]
pub struct Movement {
    pub start_time: Time,
    pub end_time: Time,
    pub start_position: Xy<Tile>,
    pub end_position: Xy<Tile>,
    pub velocity: Velocity,
    pub collision_state: CollisionState,
}
impl Movement {
    pub fn get_position(&self, current_time: Time) -> Option<Xy<Tile>> {
        let out_of_range = current_time < self.start_time || current_time > self.end_time;
        if out_of_range {
            return None;
        }
        let velocity_is_zero = check_velocity_is_zero(self.velocity);
        let start_time_is_finity = check_time_is_finity(self.start_time);
        let end_time_is_finity = check_time_is_finity(self.end_time);
        let start_position_is_finity = check_position_is_finity(self.start_position);
        let end_position_is_finity = check_position_is_finity(self.end_position);
        match (
            velocity_is_zero,
            start_time_is_finity,
            end_time_is_finity,
            start_position_is_finity,
            end_position_is_finity,
        ) {
            (true, _, _, _, true) => Some(self.end_position),
            (true, _, _, true, _) => Some(self.start_position),
            (_, _, true, _, true) => {
                let remaining_time = self.end_time - current_time;
                Some(Xy {
                    x: self.end_position.x - self.velocity.x * remaining_time,
                    y: self.end_position.y - self.velocity.y * remaining_time,
                })
            }
            (_, true, _, true, _) => {
                let delta_time = current_time - self.start_time;
                Some(Xy {
                    x: self.start_position.x + self.velocity.x * delta_time,
                    y: self.start_position.y + self.velocity.y * delta_time,
                })
            }
            _ => None,
        }
    }
    pub fn stay_forever(position: Xy<Tile>, current_time: Time) -> Self {
        Self {
            start_time: current_time,
            end_time: f32::INFINITY.ms(),
            start_position: position,
            end_position: position,
            velocity: Xy::single(Per::new(0.tile(), 1.ms())),
            collision_state: CollisionState::FreeMove,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum CollisionState {
    FreeMove,
    MoveToCollide(CollisionPrediction),
    MoveAlongAxis,
    MoveAlongAxisToCollide,
    Stuck,
}

fn check_velocity_is_zero(velocity: Velocity) -> bool {
    let zero_velocity = Xy::single(Per::new(0.tile(), 1.ms()));
    velocity == zero_velocity
}

fn check_time_is_finity(time: Time) -> bool {
    time.as_millis().is_finite()
}

fn check_position_is_finity(position: Xy<Tile>) -> bool {
    position.x.is_finite() && position.y.is_finite()
}
