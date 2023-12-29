use super::TimeSystem;
use crate::system::InitResult;
use anyhow::*;
use namui_type::*;
use std::sync::Arc;

pub(crate) async fn init() -> InitResult {
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
    fn instant_now(&self) -> Instant {
        Instant::new(Duration::from_std(true, self.start_instant.elapsed()))
    }

    fn system_now(&self) -> SystemTime {
        SystemTime::new(std::time::SystemTime::now())
    }

    fn sleep(&self, duration: Duration) -> Result<tokio::time::Sleep> {
        Ok(tokio::time::sleep(duration.to_std()?))
    }
}
