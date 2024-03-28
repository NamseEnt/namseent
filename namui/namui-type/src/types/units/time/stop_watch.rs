use crate::{Duration, Instant};
use std::fmt::Debug;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct StopWatch {
    #[cfg(not(target_family = "wasm"))]
    inner: Instant,
    #[cfg(target_family = "wasm")]
    inner: todo,
    now_fn: fn() -> Instant,
    key: String,
    index: usize,
}

impl StopWatch {
    #[cfg(feature = "namui_internal")]
    pub fn new(key: String, inner: Instant, now_fn: fn() -> Instant) -> Self {
        Self {
            inner,
            now_fn,
            key,
            index: 0,
        }
    }

    pub fn lap(&mut self) -> Duration {
        self.index += 1;
        let now = (self.now_fn)();
        let elapsed = now - self.inner;
        self.inner = now;
        elapsed
    }

    /// Note: This function also has side effect of printing the elapsed time.
    /// So if you use this recursively, the outside one will have the bigger elapsed time.
    /// You should test outside without inner one later.
    pub fn lap_and_print(&mut self) {
        let now = (self.now_fn)();
        let elapsed = now - self.inner;
        println!("StopWatch - {:?}({}): {elapsed:?}", self.key, self.index);

        self.index += 1;
        self.inner = (self.now_fn)();
    }
}

impl Debug for StopWatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
