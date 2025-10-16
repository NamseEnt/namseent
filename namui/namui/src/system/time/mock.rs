use super::*;
use crate::system::InitResult;
use anyhow::*;
use namui_type::*;
use std::sync::Arc;

pub(crate) fn init() -> InitResult {
    super::TIME_SYSTEM
        .set(Arc::new(MockTimeSystem {}))
        .map_err(|_| anyhow!("Failed to set time system"))?;

    Ok(())
}

lazy_static::lazy_static! {
    static ref INSTANT_NOW: std::sync::Mutex<std::time::Instant> = std::sync::Mutex::new(std::time::Instant::now());
    static ref SYSTEM_TIME_NOW: std::sync::Mutex<SystemTime> = std::sync::Mutex::new(SystemTime::now());
}

pub fn set_instant_now(now: std::time::Instant) {
    *INSTANT_NOW.lock().unwrap() = now;
}

pub fn set_system_time_now(now: SystemTime) {
    *SYSTEM_TIME_NOW.lock().unwrap() = now;
}

struct MockTimeSystem;

impl TimeSystem for MockTimeSystem {
    fn since_start(&self) -> Duration {
        Duration::from_std(true, INSTANT_NOW.lock().unwrap().elapsed())
    }

    fn system_time_now(&self) -> SystemTime {
        *SYSTEM_TIME_NOW.lock().unwrap()
    }

    fn now(&self) -> Instant {
        Instant::new(self.since_start())
    }

    fn sleep(&self, duration: Duration) -> time::Sleep {
        time::sleep(duration.to_std().unwrap_or_default())
    }
}
