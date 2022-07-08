use crate::{
    system::{platform_utils::web::window, InitResult},
    Time,
};

pub(crate) async fn init() -> InitResult {
    Ok(())
}

pub fn now() -> Time {
    Time::Ms(window().performance().unwrap().now() as f32)
}
