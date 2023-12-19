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

pub async fn delay(time: crate::Time) {
    fluvio_wasm_timer::Delay::new(time.as_duration())
        .await
        .unwrap();
}
