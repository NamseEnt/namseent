mod buffer_pool;
mod insert_js;
pub(crate) mod new_event_system;

use super::InitResult;
pub use insert_js::*;

pub(crate) async fn init() -> InitResult {
    new_event_system::init().await?;

    Ok(())
}

pub(crate) fn hardware_concurrency() -> u32 {
    unsafe { _hardware_concurrency() }
}

unsafe extern "C" {
    fn _hardware_concurrency() -> u32;
}
