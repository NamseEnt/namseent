pub mod bundle;
pub mod kv_store;
pub mod picker;
pub mod system_drive;
pub mod types;

use crate::system::InitResult;

pub fn init() -> InitResult {
    bundle::init()?;
    kv_store::init()?;
    Ok(())
}
