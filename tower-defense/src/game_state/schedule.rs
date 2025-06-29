use namui::*;

#[derive(Clone, Debug)]
pub struct CountBasedSchedule {
    pub interval: Duration,
    pub next_at: Instant,
    pub left_count: usize,
}

impl CountBasedSchedule {
    pub fn new(interval: Duration, count: usize, now: Instant) -> Self {
        Self {
            interval,
            next_at: now,
            left_count: count,
        }
    }

    pub fn new_once(now: Instant) -> Self {
        Self {
            interval: Duration::ZERO,
            next_at: now,
            left_count: 1,
        }
    }

    pub fn new_at_time(end_at: Instant) -> Self {
        Self {
            interval: Duration::ZERO,
            next_at: end_at,
            left_count: 1,
        }
    }

    pub fn try_emit(&mut self, now: Instant) -> bool {
        if self.left_count == 0 || now < self.next_at {
            return false;
        }

        let emits = if self.interval == Duration::ZERO {
            1
        } else {
            let elapsed = now - self.next_at;
            (elapsed.as_secs_f64() / self.interval.as_secs_f64()).floor() as usize + 1
        };
        let emits = emits.min(self.left_count);
        self.left_count = self.left_count.saturating_sub(emits);
        self.next_at += self.interval * (emits as i32);
        true
    }

    pub fn is_done(&self, _now: Instant) -> bool {
        self.left_count == 0
    }
}
