use super::TimeSystem;
use crate::system::InitResult;
use anyhow::*;
use namui_type::*;
use std::sync::Arc;

pub(crate) fn init() -> InitResult {
    super::TIME_SYSTEM
        .set(Arc::new(NonWasmTimeSystem {
            start_instant: std::time::Instant::now(),
        }))
        .map_err(|_| anyhow!("Failed to set time system"))?;

    Ok(())
}

struct NonWasmTimeSystem {
    start_instant: std::time::Instant,
}

impl TimeSystem for NonWasmTimeSystem {
    fn since_start(&self) -> Duration {
        Duration::from_std(true, self.start_instant.elapsed())
    }

    fn system_time_now(&self) -> SystemTime {
        SystemTime::now()
    }

    fn now(&self) -> Instant {
        Instant::new(self.since_start())
    }

    fn sleep(&self, duration: Duration) -> tokio::time::Sleep {
        tokio::time::sleep(duration.to_std().unwrap_or_default())
    }
}
