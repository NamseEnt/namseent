use super::*;
use crate::system::InitResult;
use anyhow::*;
use namui_type::*;
use std::sync::Mutex;

pub(crate) fn init() -> InitResult {
    super::TIME_SYSTEM
        .set(Arc::new(MockTimeSystem {}))
        .map_err(|_| anyhow!("Failed to set time system"))?;

    Ok(())
}

static INSTANT_NOW: OnceLock<Mutex<std::time::Instant>> = OnceLock::new();
static SYSTEM_TIME_NOW: OnceLock<Mutex<SystemTime>> = OnceLock::new();

pub fn set_instant_now(now: std::time::Instant) {
    *INSTANT_NOW.get_or_init(|| Mutex::new(now)).lock().unwrap() = now;
}

pub fn set_system_time_now(now: SystemTime) {
    *SYSTEM_TIME_NOW
        .get_or_init(|| Mutex::new(now))
        .lock()
        .unwrap() = now;
}

struct MockTimeSystem;

impl TimeSystem for MockTimeSystem {
    fn since_start(&self) -> Duration {
        Duration::from_std(
            true,
            INSTANT_NOW
                .get_or_init(|| Mutex::new(std::time::Instant::now()))
                .lock()
                .unwrap()
                .elapsed(),
        )
    }

    fn system_time_now(&self) -> SystemTime {
        *SYSTEM_TIME_NOW
            .get_or_init(|| Mutex::new(SystemTime::now()))
            .lock()
            .unwrap()
    }

    fn now(&self) -> Instant {
        Instant::new(self.since_start())
    }

    fn sleep(&self, duration: Duration) -> time::Sleep {
        time::sleep(duration.to_std().unwrap_or_default())
    }
}
