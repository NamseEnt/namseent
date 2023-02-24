use namui::prelude::*;
use serde::{Deserialize, Serialize};

// 25 tick per second
pub const MAX_TICK_INTERVAL: Time = Time::Ms(40.0);
// 250 tick per second
pub const MIN_TICK_INTERVAL: Time = Time::Ms(4.0);

#[derive(Serialize, Deserialize)]
pub struct TickState {
    pub last_tick_time: Time,
    pub current_time: Time,
    pub time_offset: Time,
}

impl TickState {
    pub fn new() -> Self {
        Self {
            last_tick_time: 0.0.ms(),
            current_time: 0.0.ms(),
            time_offset: 0.0.ms(),
        }
    }

    pub fn try_consume_one_tick(&mut self) -> Option<Time> {
        let delta_time = (self.current_time - self.last_tick_time).min(MAX_TICK_INTERVAL);
        if delta_time > MIN_TICK_INTERVAL {
            self.last_tick_time += delta_time;
            Some(delta_time)
        } else {
            None
        }
    }
}
