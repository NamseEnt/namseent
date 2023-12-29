use super::TimeSystem;
use crate::system::InitResult;
use anyhow::*;
use namui_type::*;
use std::sync::Arc;

pub(crate) async fn init() -> InitResult {
    super::TIME_SYSTEM
        .set(Arc::new(NonWasmTimeSystem {}))
        .map_err(|_| anyhow!("Failed to set time system"))?;

    Ok(())
}

lazy_static::lazy_static! {
    static ref INSTANT_NOW: std::sync::Mutex<std::time::Instant> = std::sync::Mutex::new(std::time::Instant::now());
    static ref SYSTEM_NOW: std::sync::Mutex<SystemTime> = std::sync::Mutex::new(SystemTime::new(std::time::SystemTime::now()));
}

pub fn set_instant_now(now: std::time::Instant) {
    *INSTANT_NOW.lock().unwrap() = now;
}

pub fn set_system_now(now: SystemTime) {
    *SYSTEM_NOW.lock().unwrap() = now;
}

struct MockTimeSystem {}

impl TimeSystem for MockTimeSystem {
    fn since_start(&self) -> Duration {
        *INSTANT_NOW.lock().unwrap().elapsed()
    }

    fn system_now(&self) -> SystemTime {
        *SYSTEM_NOW.lock().unwrap()
    }

    fn sleep(&self, duration: Duration) -> Result<tokio::time::Sleep> {
        Ok(tokio::time::sleep(duration.to_std()?))
    }
}
