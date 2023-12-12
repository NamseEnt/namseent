use super::bundle::{self};
use crate::namui::system::InitResult;

pub async fn init() -> InitResult {
    bundle::init().await
}
