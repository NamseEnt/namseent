use crate::*;
use namui_type::*;
use std::sync::Arc;

pub struct NativeTextBlob {
    pub skia_text_blob: skia_safe::TextBlob,
}

impl NativeTextBlob {
    #[allow(dead_code)]
    pub fn from_text(string: &str, font: &NativeFont) -> Self {
        NativeTextBlob {
            skia_text_blob: skia_safe::TextBlob::new(string, font.skia()).unwrap(),
        }
    }
    pub fn from_glyph_ids(glyph_ids: GlyphIds, font: &Font) -> Option<Arc<Self>> {
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct CacheKey {
            glyph_ids: GlyphIds,
            font: Font,
        }
        static CACHE: LruCache<CacheKey, NativeTextBlob> = LruCache::new();
        let cache_key = CacheKey {
            glyph_ids: glyph_ids.clone(),
            font: font.clone(),
        };

        CACHE.get_or_try_create(&cache_key, |key| {
            let native_font = NativeFont::get(&key.font)?;
            let skia_text_blob =
                skia_safe::TextBlob::from_text(glyph_ids.as_slice(), native_font.skia());

            skia_text_blob.map(|skia_text_blob| NativeTextBlob { skia_text_blob })
        })
    }
    pub fn skia(&self) -> &skia_safe::TextBlob {
        &self.skia_text_blob
    }
}
