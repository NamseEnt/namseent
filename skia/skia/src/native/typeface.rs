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
        log!(
            "try load typeface: {}, bytes: {}",
            name.as_ref(),
            bytes.len()
        );

        let data = if woff2::decode::is_woff2(bytes) {
            log!("is woff2");
            let ttf =
                woff2::decode::convert_woff2_to_ttf(&mut std::io::Cursor::new(bytes)).unwrap();
            skia_safe::Data::new_copy(&ttf)
        } else {
            skia_safe::Data::new_copy(bytes)
        };

        TYPEFACE_MAP.insert(
            name.as_ref().to_string(),
            NativeTypeface {
                // skia_typeface: skia_safe::FontMgr::default()
                //     .new_from_data(bytes, None)
                //     .unwrap_or_else(|| panic!("Failed to load typeface: {}", name.as_ref())),
                skia_typeface: skia_safe::Typeface::from_data(data, None)
                    .unwrap_or_else(|| panic!("Failed to load typeface: {}", name.as_ref())),
            },
        );

        log!("Loaded typeface: {}", name.as_ref());
    }
}
