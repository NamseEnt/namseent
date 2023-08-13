use super::*;
use std::sync::Arc;

pub struct CkFont {
    // pub(crate) id: String,
    pub(crate) canvas_kit_font: CanvasKitFont,
    // pub(crate) size: IntPx,
    pub(crate) metrics: FontMetrics,
    glyph_ids_caches: SerdeLruCache<String, Vec<usize>>,
    glyph_widths_caches: SerdeLruCache<(Vec<usize>, Paint), Vec<Px>>,
    // glyph_bounds_caches: SerdeLruCache<(GlyphIds, super::CkPaint), Vec<Rect<Px>>>,
}
impl CkFont {
    pub(crate) fn get(font: &Font) -> Option<Arc<Self>> {
        static FONT_MAP: StaticHashMap<Font, CkFont> = StaticHashMap::new();

        FONT_MAP.get_or_try_create(font.clone(), |font| {
            let typeface = CkTypeface::get(&font.name)?;

            let canvas_kit_font =
                CanvasKitFont::new(typeface.canvas_kit(), font.size.as_i32() as i16);

            let metrics = {
                let canvas_kit_font_metrics = &canvas_kit_font.getMetrics();

                FontMetrics {
                    ascent: canvas_kit_font_metrics.ascent().into(),
                    descent: canvas_kit_font_metrics.descent().into(),
                    leading: canvas_kit_font_metrics.leading().into(),
                }
            };

            Some(CkFont {
                canvas_kit_font,
                metrics,
                glyph_ids_caches: Default::default(),
                glyph_widths_caches: Default::default(),
            })
        })
    }
    pub(crate) fn canvas_kit(&self) -> &CanvasKitFont {
        &self.canvas_kit_font
    }
}

impl CkFont {
    //     pub fn generate_id(typeface: &CkTypeface, size: IntPx) -> String {
    //         format!("{}-{}", typeface.id, size)
    //     }
    //     pub fn new(typeface: &CkTypeface, size: IntPx) -> Self {
    //         let canvas_kit_font =
    //             CanvasKitFont::new(&typeface.canvas_kit_typeface, size.as_i32() as i16);
    //         CkFont {
    //             id: Self::generate_id(typeface, size),
    //             size,
    //             metrics: {
    //                 let canvas_kit_font_metrics = &canvas_kit_font.getMetrics();

    //                 FontMetrics {
    //                     ascent: canvas_kit_font_metrics.ascent().into(),
    //                     descent: canvas_kit_font_metrics.descent().into(),
    //                     leading: canvas_kit_font_metrics.leading().into(),
    //                 }
    //             },
    //             canvas_kit_font,
    //             glyph_ids_caches: Mutex::new(lru::LruCache::new(NonZeroUsize::new(1024).unwrap())),
    //             glyph_widths_caches: Mutex::new(lru::LruCache::new(NonZeroUsize::new(1024).unwrap())),
    //             glyph_bounds_caches: Mutex::new(lru::LruCache::new(NonZeroUsize::new(1024).unwrap())),
    //         }
    //     }
    pub(crate) fn glyph_ids(&self, text: impl AsRef<str>) -> Vec<usize> {
        let text = text.as_ref().to_string();
        if text.len() == 0 {
            return vec![];
        }

        self.glyph_ids_caches
            .get_or_create(&text, |text| {
                self.canvas_kit_font.getGlyphIDs(text.as_ref())
            })
            .to_vec()
    }
    pub(crate) fn glyph_widths(&self, glyph_ids: Vec<usize>, paint: &Paint) -> Vec<Px> {
        if glyph_ids.len() == 0 {
            return vec![];
        }
        self.glyph_widths_caches
            .get_or_create(&(glyph_ids, paint.clone()), |(glyph_ids, paint)| {
                let ck_paint = CkPaint::get(paint);

                let glyph_widths: Vec<Px> = self
                    .canvas_kit_font
                    .getGlyphWidths(glyph_ids.clone(), Some(ck_paint.canvas_kit()))
                    .to_vec()
                    .into_iter()
                    .map(|n| n.into())
                    .collect();

                glyph_widths
            })
            .to_vec()
    }
    //     pub(crate) fn get_glyph_bounds(
    //         &self,
    //         glyph_ids: GlyphIds,
    //         paint: &super::CkPaint,
    //     ) -> Vec<Rect<Px>> {
    //         let mut caches = self.glyph_bounds_caches.lock().unwrap();

    //         let key = (glyph_ids.clone(), paint.clone());
    //         match caches.get(&key) {
    //             Some(glyph_bounds) => glyph_bounds.clone(),
    //             None => {
    //                 let bound_items = self
    //                     .canvas_kit_font
    //                     .getGlyphBounds(glyph_ids, Some(&paint.canvas_kit_paint))
    //                     .to_vec();

    //                 let mut iter = bound_items.iter().peekable();
    //                 let mut bounds = Vec::new();

    //                 while iter.peek().is_some() {
    //                     bounds.push(Rect::Ltrb {
    //                         left: px(*iter.next().unwrap()),
    //                         top: px(*iter.next().unwrap()),
    //                         right: px(*iter.next().unwrap()),
    //                         bottom: px(*iter.next().unwrap()),
    //                     });
    //                 }

    //                 caches.put(key, bounds.clone());

    //                 bounds
    //             }
    //         }
    //     }
}

// impl Drop for CkFont {
//     fn drop(&mut self) {
//         self.canvas_kit_font.delete();
//     }
// }

// impl std::fmt::Debug for CkFont {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Font")
//             .field("id", &self.id)
//             .field("size", &self.size)
//             .finish()
//     }
// }

// impl std::hash::Hash for CkFont {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.id.hash(state);
//     }
// }

// impl PartialEq for CkFont {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }

// impl Eq for CkFont {}
