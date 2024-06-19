pub mod http;
pub mod ws;

use super::InitResult;

pub(super) async fn init() -> InitResult {
    Ok(())
}
