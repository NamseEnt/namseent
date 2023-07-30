use crate::{system::InitResult, Time};

pub(crate) async fn init() -> InitResult {
    Ok(())
}

pub fn now() -> Time {
    Time::Ms(0 as f32)
}
