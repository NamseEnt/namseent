use namui::prelude::*;

// 25 tick per second
pub const MAX_TICK_INTERVAL: Time = Time::Ms(40.0);

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

    pub fn delta_time(&self) -> Time {
        self.current_time - self.last_tick_time
    }

    pub fn need_to_evaluate_more_than_one_tick(&self) -> bool {
        self.delta_time() > 0.ms()
    }

    pub fn consume_one_tick(&mut self) {
        self.last_tick_time += self.delta_time().min(MAX_TICK_INTERVAL);
    }
}
