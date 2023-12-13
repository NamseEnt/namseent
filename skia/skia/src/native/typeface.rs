use super::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct NativeTypeface {
    pub skia_typeface: skia_safe::Typeface,
}

static TYPEFACE_MAP: StaticHashMap<String, NativeTypeface> = StaticHashMap::new();

impl NativeTypeface {
    pub(crate) fn get(name: impl AsRef<str>) -> Option<Arc<Self>> {
        TYPEFACE_MAP.get(&name.as_ref().to_string())
    }
    pub(crate) fn load(name: impl AsRef<str>, bytes: &[u8]) {
        TYPEFACE_MAP.insert(
            name.as_ref().to_string(),
            NativeTypeface {
                skia_typeface: skia_safe::FontMgr::default()
                    .new_from_data(bytes, None)
                    .unwrap(),
            },
        );
    }
}
