use crate::*;
use dashmap::DashMap;
use std::sync::{Arc, OnceLock};

#[derive(Clone)]
pub struct NativeTypeface {
    pub skia_typeface: skia_safe::Typeface,
}

static TYPEFACE_MAP: OnceLock<DashMap<String, Arc<NativeTypeface>>> = OnceLock::new();
fn typeface_map() -> &'static DashMap<String, Arc<NativeTypeface>> {
    TYPEFACE_MAP.get_or_init(DashMap::new)
}

impl NativeTypeface {
    pub fn get(name: impl AsRef<str>) -> Option<Arc<Self>> {
        typeface_map()
            .get(&name.as_ref().to_string())
            .map(|v| v.value().clone())
    }
    pub fn load(name: impl AsRef<str>, bytes: &[u8]) -> anyhow::Result<()> {
        let skia_typeface = skia_safe::FontMgr::default()
            .new_from_data2(bytes, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to create a typeface from data."))?;

        typeface_map().insert(
            name.as_ref().to_string(),
            Arc::new(NativeTypeface { skia_typeface }),
        );

        Ok(())
    }
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn _register_font(
    name_ptr: *const u8,
    name_len: usize,
    buffer_ptr: *const u8,
    buffer_len: usize,
) {
    let name_bytes = unsafe { std::slice::from_raw_parts(name_ptr, name_len) };
    let name = String::from_utf8_lossy(name_bytes).to_string();

    let buffer_bytes = unsafe { std::slice::from_raw_parts(buffer_ptr, buffer_len) };
    let buffer = Vec::from(buffer_bytes);

    if let Err(e) = NativeTypeface::load(&name, &buffer) {
        panic!("Failed to load font {}: {}", name, e);
    }
}
