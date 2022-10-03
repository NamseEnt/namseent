pub use super::base::*;
use super::*;
use crate::*;
use std::sync::Mutex;

unsafe impl Sync for CanvasKitFont {}
unsafe impl Send for CanvasKitFont {}
pub struct Font {
    pub(crate) id: String,
    pub(crate) canvas_kit_font: CanvasKitFont,
    pub(crate) size: IntPx,
    pub(crate) metrics: FontMetrics,
    glyph_ids_caches: Mutex<lru::LruCache<String, GlyphIds>>,
    glyph_widths_caches: Mutex<lru::LruCache<(GlyphIds, Option<Paint>), Vec<Px>>>,
    glyph_bounds_caches: Mutex<lru::LruCache<(GlyphIds, Option<Paint>), Vec<Rect<Px>>>>,
}

impl Font {
    pub fn generate_id(typeface: &Typeface, size: IntPx) -> String {
        format!("{}-{}", typeface.id, size)
    }
    pub fn new(typeface: &Typeface, size: IntPx) -> Self {
        let canvas_kit_font = CanvasKitFont::new(&typeface.canvas_kit_typeface, size.0 as i16);
        Font {
            id: Self::generate_id(typeface, size),
            size,
            metrics: {
                let canvas_kit_font_metrics = &canvas_kit_font.getMetrics();

                FontMetrics {
                    ascent: canvas_kit_font_metrics.ascent().into(),
                    descent: canvas_kit_font_metrics.descent().into(),
                    leading: canvas_kit_font_metrics.leading().into(),
                }
            },
            canvas_kit_font,
            glyph_ids_caches: Mutex::new(lru::LruCache::new(1024)),
            glyph_widths_caches: Mutex::new(lru::LruCache::new(1024)),
            glyph_bounds_caches: Mutex::new(lru::LruCache::new(1024)),
        }
    }
    pub fn get_glyph_ids(&self, text: impl AsRef<str>) -> GlyphIds {
        let mut caches = self.glyph_ids_caches.lock().unwrap();

        match caches.get(text.as_ref()) {
            Some(glyph_ids) => glyph_ids.clone(),
            None => {
                let glyph_ids = self.canvas_kit_font.getGlyphIDs(text.as_ref());
                caches.put(text.as_ref().to_string(), glyph_ids.clone());
                glyph_ids
            }
        }
    }
    pub(crate) fn get_glyph_widths(&self, glyph_ids: GlyphIds, paint: Option<&Paint>) -> Vec<Px> {
        let mut caches = self.glyph_widths_caches.lock().unwrap();

        let key = (glyph_ids.clone(), paint.cloned());
        match caches.get(&key) {
            Some(glyph_widths) => glyph_widths.clone(),
            None => {
                let glyph_widths: Vec<Px> = self
                    .canvas_kit_font
                    .getGlyphWidths(glyph_ids, paint.map(|paint| &paint.canvas_kit_paint))
                    .to_vec()
                    .into_iter()
                    .map(|n| n.into())
                    .collect();
                caches.put(key, glyph_widths.clone());
                glyph_widths
            }
        }
    }
    pub(crate) fn get_glyph_bounds(
        &self,
        glyph_ids: GlyphIds,
        paint: Option<&Paint>,
    ) -> Vec<Rect<Px>> {
        let mut caches = self.glyph_bounds_caches.lock().unwrap();

        let key = (glyph_ids.clone(), paint.cloned());
        match caches.get(&key) {
            Some(glyph_bounds) => glyph_bounds.clone(),
            None => {
                let bound_items = self
                    .canvas_kit_font
                    .getGlyphBounds(glyph_ids, paint.map(|paint| &paint.canvas_kit_paint))
                    .to_vec();

                let mut iter = bound_items.iter().peekable();
                let mut bounds = Vec::new();

                while iter.peek().is_some() {
                    bounds.push(Rect::Ltrb {
                        left: px(*iter.next().unwrap()),
                        top: px(*iter.next().unwrap()),
                        right: px(*iter.next().unwrap()),
                        bottom: px(*iter.next().unwrap()),
                    });
                }

                caches.put(key, bounds.clone());

                bounds
            }
        }
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        self.canvas_kit_font.delete();
    }
}

impl std::fmt::Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::hash::Hash for Font {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Font {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Font {}
