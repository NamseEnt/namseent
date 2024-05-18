use super::*;
use crate::*;
use std::sync::Arc;

pub struct CkTextBlob {
    pub canvas_kit_text_blob: CanvasKitTextBlob,
}

impl CkTextBlob {
    #[allow(dead_code)]
    pub fn from_text(string: &str, font: &CkFont) -> Self {
        CkTextBlob {
            canvas_kit_text_blob: canvas_kit()
                .TextBlob()
                .MakeFromText(string, &font.canvas_kit_font),
        }
    }
    pub fn from_glyph_ids(glyph_ids: GlyphIds, font: &Font) -> Option<Arc<Self>> {
        #[derive(serde::Serialize)]
        struct CacheKey {
            glyph_ids: GlyphIds,
            font: Font,
        }
        static CACHE: SerdeLruCache<CacheKey, CkTextBlob> = SerdeLruCache::new();
        let cache_key = CacheKey {
            glyph_ids: glyph_ids.clone(),
            font: font.clone(),
        };

        CACHE.get_or_try_create(&cache_key, |key| {
            let ck_font = CkFont::get(&key.font)?;

            Some(CkTextBlob {
                canvas_kit_text_blob: canvas_kit()
                    .TextBlob()
                    .MakeFromGlyphs(glyph_ids, ck_font.canvas_kit()),
            })
        })
    }
    pub fn canvas_kit(&self) -> &CanvasKitTextBlob {
        &self.canvas_kit_text_blob
    }
}
impl Drop for CkTextBlob {
    fn drop(&mut self) {
        self.canvas_kit_text_blob.delete();
    }
}
