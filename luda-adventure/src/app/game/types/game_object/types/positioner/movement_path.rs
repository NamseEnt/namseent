use crate::app::game::Tile;
use namui::prelude::*;

#[derive(Clone, Debug)]
pub struct MovementPath {
    start_time: Time,
    end_time: Time,
    start_xy: Xy<Tile>,
    end_xy: Xy<Tile>,
}

impl MovementPath {
    pub fn new(start_time: Time, end_time: Time, start_xy: Xy<Tile>, end_xy: Xy<Tile>) -> Self {
        Self {
            start_time,
            end_time,
            start_xy,
            end_xy,
        }
    }

    pub fn xy(&self, time: Time) -> Option<Xy<Tile>> {
        let out_range = self.start_time > time || time > self.end_time;
        if out_range {
            return None;
        }

        let delta_xy = self.end_xy - self.start_xy;
        let delta_time = self.end_time - self.start_time;
        let remaining_time = self.end_time - time;

        if delta_time == 0.ms() || remaining_time == 0.ms() {
            return Some(self.end_xy);
        }

        let remaining_xy = delta_xy * (remaining_time / delta_time);

        Some(self.end_xy - remaining_xy)
    }

    pub fn end_time(&self) -> Time {
        self.end_time
    }
}
