use super::{platform_utils::web::window, InitResult};
use std::time::Duration;

pub(super) async fn init() -> InitResult {
    Ok(())
}

pub fn now() -> Duration {
    Duration::from_millis(window().performance().unwrap().now() as u64)
}
