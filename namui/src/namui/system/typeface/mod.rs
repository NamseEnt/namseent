mod load_sans_typeface_of_all_languages;
use super::InitResult;
use load_sans_typeface_of_all_languages::*;

pub(super) async fn init() -> InitResult {
    load_all_typefaces().await?;
    Ok(())
}

pub(crate) fn register_typeface(typeface_name: &str, bytes: &[u8]) {
    crate::system::drawer::load_typeface(typeface_name, bytes);
    crate::system::skia::load_typeface(typeface_name, bytes);
}
