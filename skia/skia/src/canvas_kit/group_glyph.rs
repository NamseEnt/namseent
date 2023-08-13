use super::*;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct CkGroupGlyph {
    font: Font,
    paint: Paint,
}

impl CkGroupGlyph {
    pub(crate) fn get(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
        #[derive(serde::Serialize)]
        struct Key {
            font: Font,
            paint: Paint,
        }
        static CACHE: SerdeLruCache<Key, CkGroupGlyph> = SerdeLruCache::new();

        let key = Key {
            font: font.clone(),
            paint: paint.clone(),
        };

        CACHE.get_or_create(&key, |key| CkGroupGlyph {
            font: key.font.clone(),
            paint: key.paint.clone(),
        }) as Arc<dyn GroupGlyph>
    }
}

impl GroupGlyph for CkGroupGlyph {
    fn groups(&self, text: &str) -> Vec<GlyphGroup> {
        // TODO: Handle fallback font
        let Some(ck_font) = CkFont::get(&self.font) else {
            return vec![];
        };

        let glyph_ids = ck_font.glyph_ids(text);
        let glyph_widths = ck_font.glyph_widths(glyph_ids.clone(), &self.paint);

        let glyphs = glyph_ids
            .into_iter()
            .zip(glyph_widths.into_iter())
            .map(|(id, width)| Glyph { id, width })
            .collect::<Vec<_>>();

        let width = glyphs.iter().map(|glyph| glyph.width).sum();

        vec![GlyphGroup {
            font: self.font.clone(),
            glyphs,
            width,
        }]
    }

    fn width(&self, text: &str) -> Px {
        self.groups(text).into_iter().map(|group| group.width).sum()
    }

    fn widths(&self, text: &str) -> Vec<Px> {
        self.groups(text)
            .into_iter()
            .flat_map(|group| group.glyphs.into_iter().map(|glyph| glyph.width))
            .collect()
    }

    fn font_metrics(&self) -> FontMetrics {
        match CkFont::get(&self.font) {
            Some(font) => font.metrics,
            None => FontMetrics::default(),
        }
    }

    fn bounds(&self, text: &str) -> Vec<Rect<Px>> {
        let Some(ck_font) = CkFont::get(&self.font) else {
            return vec![];
        };

        let glyph_ids = ck_font.glyph_ids(text);
        ck_font.glyph_bounds(glyph_ids.clone(), &self.paint)
    }
}

unsafe impl Send for CkGroupGlyph {}
unsafe impl Sync for CkGroupGlyph {}
