mod field_area_effect;
mod once;

pub use field_area_effect::*;
use namui::*;
pub use once::*;

struct EmitSchedule {
    emit_interval: Duration,
    next_emit_at: Instant,
    left_emit_count: usize,
}
impl EmitSchedule {
    fn new(emit_interval: Duration, emit_count: usize, now: Instant) -> Self {
        Self {
            emit_interval,
            next_emit_at: now,
            left_emit_count: emit_count,
        }
    }

    pub fn try_emit(&mut self, now: Instant) -> bool {
        if self.left_emit_count == 0 || now < self.next_emit_at {
            return false;
        }

        let emits = ((now - self.next_emit_at).as_secs_f64() / self.emit_interval.as_secs_f64())
            .floor() as usize
            + 1;
        let emits = emits.min(self.left_emit_count);
        self.left_emit_count = self.left_emit_count.saturating_sub(emits);
        self.next_emit_at += self.emit_interval * (emits as i32);
        true
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.left_emit_count == 0
    }
}
