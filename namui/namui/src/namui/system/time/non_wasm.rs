use crate::{system::InitResult, Time};

pub(crate) async fn init() -> InitResult {
    Ok(())
}

pub fn now() -> Time {
    Time::Ms(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f32(),
    )
}

pub async fn delay(time: crate::Time) {
    tokio::time::sleep(time.as_duration()).await;
}
