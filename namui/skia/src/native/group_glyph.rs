use super::*;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct NativeGroupGlyph {
    font: Font,
    paint: Paint,
}

impl NativeGroupGlyph {
    pub(crate) fn get(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
        #[derive(Hash, PartialEq, Eq, Clone)]
        struct Key {
            font: Font,
            paint: Paint,
        }
        static CACHE: LruCache<Key, NativeGroupGlyph> = LruCache::new();

        let key = Key {
            font: font.clone(),
            paint: paint.clone(),
        };

        CACHE.get_or_create(&key, |key| NativeGroupGlyph {
            font: key.font.clone(),
            paint: key.paint.clone(),
        }) as Arc<dyn GroupGlyph>
    }
}

impl GroupGlyph for NativeGroupGlyph {
    fn groups(&self, text: &str) -> Vec<GlyphGroup> {
        // TODO: Handle fallback font
        let Some(native_font) = NativeFont::get(&self.font) else {
            return vec![];
        };

        let glyph_ids = native_font.glyph_ids(text);
        let glyph_widths = native_font.glyph_widths(glyph_ids.clone(), &self.paint);

        let glyphs = glyph_ids
            .into_iter()
            .zip(glyph_widths)
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
        match NativeFont::get(&self.font) {
            Some(font) => font.metrics,
            None => FontMetrics::default(),
        }
    }

    fn bounds(&self, text: &str) -> Vec<Rect<Px>> {
        let Some(native_font) = NativeFont::get(&self.font) else {
            return vec![];
        };

        let glyph_ids = native_font.glyph_ids(text);
        native_font.glyph_bounds(glyph_ids.clone(), &self.paint)
    }

    fn bound(&self, text: &str) -> Rect<Px> {
        self.bounds(text).into_iter().fold(Rect::default(), |a, b| {
            a.get_minimum_rectangle_containing(b)
        })
    }
}

unsafe impl Send for NativeGroupGlyph {}
unsafe impl Sync for NativeGroupGlyph {}
