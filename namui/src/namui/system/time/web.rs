use crate::{system::InitResult, Time};

pub(crate) async fn init() -> InitResult {
    Ok(())
}

pub fn now() -> Time {
    todo!()
    // Time::Ms(window().performance().unwrap().now() as f32)
}
