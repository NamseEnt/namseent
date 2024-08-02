use crate::{Duration, Instant};
use std::fmt::Debug;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct StopWatch {
    last: Instant,
    key: String,
    index: usize,
}

impl StopWatch {
    pub fn new(key: String) -> Self {
        Self {
            last: Instant::now(),
            key,
            index: 0,
        }
    }

    pub fn lap(&mut self) -> Duration {
        self.index += 1;
        let now = Instant::now();
        let elapsed = now - self.last;
        self.last = now;
        elapsed
    }

    /// Note: This function also has side effect of printing the elapsed time.
    /// So if you use this recursively, the outside one will have the bigger elapsed time.
    /// You should test outside without inner one later.
    pub fn lap_and_print(&mut self) {
        let now = Instant::now();
        let elapsed = now - self.last;
        crate::log!("StopWatch - {:?}({}): {elapsed:?}", self.key, self.index);

        self.index += 1;
        self.last = Instant::now();
    }
}

impl Debug for StopWatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.last.fmt(f)
    }
}
