use super::*;
use crate::system::InitResult;
use anyhow::*;
use namui_type::*;
use std::sync::Arc;

pub(crate) async fn init() -> InitResult {
    super::TIME_SYSTEM
        .set(Arc::new(WasmTimeSystem {
            start_instant: Instant::now(),
        }))
        .map_err(|_| anyhow!("Failed to set time system"))?;

    Ok(())
}

struct WasmTimeSystem {
    start_instant: Instant,
}

impl TimeSystem for WasmTimeSystem {
    fn since_start(&self) -> Duration {
        Instant::now() - self.start_instant
    }

    fn system_time_now(&self) -> SystemTime {
        SystemTime::now()
    }

    fn now(&self) -> Instant {
        Instant::new(self.since_start())
    }

    fn sleep(&self, duration: Duration) -> time::Sleep {
        time::sleep(duration.to_std().unwrap_or_default())
    }
}
