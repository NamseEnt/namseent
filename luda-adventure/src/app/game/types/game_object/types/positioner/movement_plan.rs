use super::Movement;
use crate::app::game::Tile;
use namui::prelude::*;

pub struct MovementPlan {
    pub directed_movement: Movement,
    pub predicted_movement_list: Vec<Movement>,
}
impl MovementPlan {
    pub fn get_position(&self, current_time: Time) -> Option<Xy<Tile>> {
        for movement in self.predicted_movement_list.iter() {
            if let Some(position) = movement.get_position(current_time) {
                return Some(position);
            }
        }
        None
    }
    pub fn stay_forever(position: Xy<Tile>, current_time: Time) -> Self {
        Self {
            directed_movement: Movement::stay_forever(position, current_time),
            predicted_movement_list: vec![Movement::stay_forever(position, current_time)],
        }
    }
    pub fn move_now(
        position: Xy<Tile>,
        current_time: Time,
        duration: namui::Time,
        velocity: namui::Xy<namui::Per<crate::app::game::Tile, namui::Time>>,
    ) -> Self {
        let end_position = Xy {
            x: velocity.x * duration + position.x,
            y: velocity.y * duration + position.y,
        };
        Self {
            directed_movement: Movement {
                start_time: current_time,
                end_time: current_time + duration,
                start_position: position,
                end_position,
                velocity,
            },
            predicted_movement_list: vec![Movement {
                start_time: current_time,
                end_time: current_time,
                start_position: position,
                end_position: position,
                velocity,
            }],
        }
    }
    pub fn get_predicted_movement_end_time(&self) -> Time {
        self.predicted_movement_list
            .last()
            .map(|last_predicated_movement| last_predicated_movement.end_time)
            .unwrap_or(self.directed_movement.start_time)
    }
}
