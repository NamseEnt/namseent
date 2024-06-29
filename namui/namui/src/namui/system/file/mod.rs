pub mod bundle;
pub mod local_storage;
pub mod picker;
pub mod system_drive;
pub mod types;

use super::*;
use crate::namui::system::InitResult;
use tokio::try_join;

pub async fn init() -> InitResult {
    try_join![bundle::init(), local_storage::init(),]?;
    Ok(())
}
