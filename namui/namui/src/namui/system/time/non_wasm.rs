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

struct NonWasmTimeSystem {}

impl TimeSystem for NonWasmTimeSystem {
    fn instant_now(&self) -> Instant {
        Instant::new(std::time::Instant::now())
    }

    fn system_now(&self) -> SystemTime {
        SystemTime::new(std::time::SystemTime::now())
    }

    fn sleep(&self, duration: Duration) -> Result<tokio::time::Sleep> {
        Ok(tokio::time::sleep(duration.to_std()?))
    }
}
