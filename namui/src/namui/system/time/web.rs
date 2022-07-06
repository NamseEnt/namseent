use crate::system::{platform_utils::web::window, InitResult};
use std::time::Duration;

pub(crate) async fn init() -> InitResult {
    Ok(())
}

pub fn now() -> Duration {
    Duration::from_millis(window().performance().unwrap().now() as u64)
}