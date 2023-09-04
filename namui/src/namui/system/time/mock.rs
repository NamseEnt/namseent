use crate::{system::InitResult, Time};

pub(crate) async fn init() -> InitResult {
    Ok(())
}

lazy_static::lazy_static! {
    static ref NOW: std::sync::Mutex<Time> = std::sync::Mutex::new(Time::Ms(0.0));
}

pub fn now() -> Time {
    *NOW.lock().unwrap()
}

pub fn set_now(now: Time) {
    *NOW.lock().unwrap() = now;
}
