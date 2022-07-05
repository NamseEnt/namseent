use crate::system::InitResult;
use std::time::Duration;

pub(crate) async fn init() -> InitResult {
    Ok(())
}

lazy_static::lazy_static! {
    static ref NOW: std::sync::Mutex<Duration> = std::sync::Mutex::new(Duration::from_millis(0));
}

pub fn now() -> Duration {
    NOW.lock().unwrap().clone()
}

pub fn set_now(now: Duration) {
    *NOW.lock().unwrap() = now;
}
