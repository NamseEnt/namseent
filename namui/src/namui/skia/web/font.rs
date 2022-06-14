pub use super::base::*;
use super::*;
use std::sync::{Arc, Mutex};

unsafe impl Sync for CanvasKitFont {}
unsafe impl Send for CanvasKitFont {}
pub struct Font {
    pub(crate) id: String,
    pub(crate) canvas_kit_font: CanvasKitFont,
    pub(crate) size: i16,
    pub(crate) metrics: FontMetrics,
    glyph_ids_caches: Mutex<lru::LruCache<String, Arc<GlyphIds>>>,
    glyph_widths_caches: Mutex<lru::LruCache<(Arc<GlyphIds>, Option<Paint>), Vec<f32>>>,
    glyph_bounds_caches: Mutex<lru::LruCache<(Arc<GlyphIds>, Option<Paint>), Vec<LtrbRect>>>,
}

impl Font {
    pub fn generate_id(typeface: &Typeface, size: i16) -> String {
        format!("{}-{}", typeface.id, size)
    }
    pub fn new(typeface: &Typeface, size: i16) -> Self {
        let canvas_kit_font = CanvasKitFont::new(&typeface.canvas_kit_typeface, size);
        Font {
            id: Self::generate_id(typeface, size),
            size,
            metrics: {
                let canvas_kit_font_metrics = &canvas_kit_font.getMetrics();
                let bounds = canvas_kit_font_metrics.bounds().map(|numbers| LtrbRect {
                    left: numbers[0],
                    top: numbers[1],
                    right: numbers[2],
                    bottom: numbers[3],
                });

                FontMetrics {
                    ascent: canvas_kit_font_metrics.ascent(),
                    descent: canvas_kit_font_metrics.descent(),
                    leading: canvas_kit_font_metrics.leading(),
                    bounds,
                }
            },
            canvas_kit_font,
            glyph_ids_caches: Mutex::new(lru::LruCache::new(1024)),
            glyph_widths_caches: Mutex::new(lru::LruCache::new(1024)),
            glyph_bounds_caches: Mutex::new(lru::LruCache::new(1024)),
        }
    }
    pub fn get_glyph_ids(&self, text: impl AsRef<str>) -> Arc<GlyphIds> {
        let mut caches = self.glyph_ids_caches.lock().unwrap();

        match caches.get(text.as_ref()) {
            Some(glyph_ids) => glyph_ids.clone(),
            None => {
                let glyph_ids: Arc<GlyphIds> =
                    self.canvas_kit_font.getGlyphIDs(text.as_ref()).into();
                caches.put(text.as_ref().to_string(), glyph_ids.clone());
                glyph_ids
            }
        }
    }
    pub(crate) fn get_glyph_widths(
        &self,
        glyph_ids: Arc<GlyphIds>,
        paint: Option<&Paint>,
    ) -> Vec<f32> {
        let mut caches = self.glyph_widths_caches.lock().unwrap();

        let key = (glyph_ids.clone(), paint.cloned());
        match caches.get(&key) {
            Some(glyph_widths) => glyph_widths.clone(),
            None => {
                let glyph_widths = self
                    .canvas_kit_font
                    .getGlyphWidths(&glyph_ids, paint.map(|paint| &paint.canvas_kit_paint))
                    .to_vec();
                caches.put(key, glyph_widths.clone());
                glyph_widths
            }
        }
    }
    pub(crate) fn get_glyph_bounds(
        &self,
        glyph_ids: Arc<GlyphIds>,
        paint: Option<&Paint>,
    ) -> Vec<LtrbRect> {
        let mut caches = self.glyph_bounds_caches.lock().unwrap();

        let key = (glyph_ids.clone(), paint.cloned());
        match caches.get(&key) {
            Some(glyph_bounds) => glyph_bounds.clone(),
            None => {
                let bound_items = self
                    .canvas_kit_font
                    .getGlyphBounds(
                        glyph_ids.as_ref(),
                        paint.map(|paint| &paint.canvas_kit_paint),
                    )
                    .to_vec();

                let mut iter = bound_items.iter().peekable();
                let mut bounds = Vec::new();

                while iter.peek().is_some() {
                    bounds.push(LtrbRect {
                        left: *iter.next().unwrap(),
                        top: *iter.next().unwrap(),
                        right: *iter.next().unwrap(),
                        bottom: *iter.next().unwrap(),
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
