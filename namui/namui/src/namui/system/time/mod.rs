#[cfg(test)]
mod mock;
#[cfg(not(target_family = "wasm"))]
#[cfg(not(test))]
mod non_wasm;
#[cfg(target_family = "wasm")]
#[cfg(not(test))]
mod web;

#[cfg(test)]
pub use mock::*;
use namui_type::*;
#[cfg(not(target_family = "wasm"))]
#[cfg(not(test))]
pub(crate) use non_wasm::*;
use std::sync::{Arc, OnceLock};
#[cfg(target_family = "wasm")]
#[cfg(not(test))]
pub(crate) use web::*;

static TIME_SYSTEM: OnceLock<Arc<dyn TimeSystem + Send + Sync>> = OnceLock::new();

/// It's time since the program started.
pub fn since_start() -> Duration {
    TIME_SYSTEM.get().unwrap().since_start()
}

pub fn system_time_now() -> SystemTime {
    TIME_SYSTEM.get().unwrap().system_time_now()
}

/// It's just monotonic time. If you want to get the clock's date or time, use `system_time_now`.
pub fn now() -> Instant {
    TIME_SYSTEM.get().unwrap().now()
}

pub fn stop_watch(key: impl AsRef<str>) -> StopWatch {
    StopWatch::new(key.as_ref().to_string(), now(), now)
}

/// You can await on this.
/// ```no_run
/// sleep(Duration::from_secs(1)).await;
/// ```
/// Sleep 0 duration if passed duration is less than 0.
pub fn sleep(duration: Duration) -> tokio::time::Sleep {
    TIME_SYSTEM.get().unwrap().sleep(duration)
}

trait TimeSystem {
    fn since_start(&self) -> Duration;
    fn system_time_now(&self) -> SystemTime;
    fn now(&self) -> Instant;
    /// Sleep 0 duration if passed duration is less than 0.
    fn sleep(&self, duration: Duration) -> tokio::time::Sleep;
}
