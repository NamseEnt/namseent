#[cfg(test)]
mod mock;
#[cfg(not(target_family = "wasm"))]
#[cfg(not(test))]
mod non_wasm;
#[cfg(target_family = "wasm")]
#[cfg(not(test))]
mod web;

use anyhow::Result;
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

trait TimeSystem {
    fn since_start(&self) -> Duration;
    fn system_now(&self) -> SystemTime;
    fn sleep(&self, duration: Duration) -> Result<tokio::time::Sleep>;
}

static TIME_SYSTEM: OnceLock<Arc<dyn TimeSystem + Send + Sync>> = OnceLock::new();

/// It's time since the program started.
pub fn since_start() -> Duration {
    TIME_SYSTEM.get().unwrap().since_start()
}
pub fn system_now() -> SystemTime {
    TIME_SYSTEM.get().unwrap().system_now()
}

/// You can await on this.
/// ```no_run
/// sleep(Duration::from_secs(1)).await;
/// ```
/// `Err` if duration is less than 0.
pub fn sleep(duration: Duration) -> Result<tokio::time::Sleep> {
    TIME_SYSTEM.get().unwrap().sleep(duration)
}
