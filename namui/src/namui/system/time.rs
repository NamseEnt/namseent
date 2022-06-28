use super::platform_utils::web::window;
use std::time::Duration;

pub fn now() -> Duration {
    Duration::from_millis(window().performance().unwrap().now() as u64)
}
