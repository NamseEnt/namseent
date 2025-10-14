pub mod bundle;
pub mod kv_store;
pub mod picker;
pub mod system_drive;
pub mod types;

use super::*;
use crate::system::InitResult;
use tokio::try_join;

pub fn init() -> InitResult {
    bundle::init()?;
    kv_store::init()?;
    Ok(())
}
