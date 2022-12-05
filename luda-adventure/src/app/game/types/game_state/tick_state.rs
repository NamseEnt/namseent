use namui::prelude::*;

// 25 tick per second
pub const TICK_INTERVAL: Time = Time::Ms(40.0);

pub struct TickState {
    pub last_tick_time: Time,
    pub current_time: Time,
}

impl TickState {
    pub fn new() -> Self {
        Self {
            last_tick_time: 0.0.ms(),
            current_time: 0.0.ms(),
        }
    }

    pub fn need_to_evaluate_more_than_one_tick(&self) -> bool {
        self.current_time - self.last_tick_time > TICK_INTERVAL
    }

    pub fn consume_one_tick(&mut self) {
        self.last_tick_time += TICK_INTERVAL;
    }

    pub fn interpolation_progress(&self) -> f32 {
        (self.current_time - self.last_tick_time) / TICK_INTERVAL
    }
}
