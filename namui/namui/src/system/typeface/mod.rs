mod load_sans_typeface_of_all_languages;
use super::InitResult;
use anyhow::Result;
use load_sans_typeface_of_all_languages::*;

pub(super) async fn init() -> InitResult {
    load_all_typefaces().await?;
    Ok(())
}

/// Supported font formats: TTF, WOFF2
/// Not tested with WOFF and OTF
pub async fn register_typeface(typeface_name: impl ToString, bytes: Vec<u8>) -> Result<()> {
    // TODO
    // crate::system::skia::load_typeface(typeface_name.to_string(), bytes).await
    Ok(())
}
