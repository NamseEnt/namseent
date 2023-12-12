use super::bundle::{self};
use crate::namui::system::InitResult;

pub async fn init() -> InitResult {
    Ok(bundle::init().await?)
}
