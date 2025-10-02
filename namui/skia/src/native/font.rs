// use super::*;
// use std::sync::Arc;

// pub struct NativeFont {
//     skia_font: skia_safe::Font,
//     pub(crate) metrics: FontMetrics,
//     glyph_ids_caches: LruCache<String, GlyphIds>,
//     glyph_widths_caches: LruCache<(GlyphIds, Paint), Vec<Px>>,
//     glyph_bounds_caches: LruCache<(GlyphIds, Paint), Vec<Rect<Px>>>,
// }
// impl NativeFont {
//     pub(crate) fn get(font: &Font) -> Option<Arc<Self>> {
//         static FONT_MAP: StaticHashMap<Font, NativeFont> = StaticHashMap::new();

//         FONT_MAP.get_or_try_create(font.clone(), |font| {
//             let typeface = NativeTypeface::get(&font.name)?;

//             let skia_font = skia_safe::Font::from_typeface(
//                 &typeface.skia_typeface,
//                 Some(font.size.as_i32() as f32),
//             );

//             let metrics = {
//                 let (_line_spacing, skia_font_metrics) = &skia_font.metrics();

//                 FontMetrics {
//                     ascent: skia_font_metrics.ascent.into(),
//                     descent: skia_font_metrics.descent.into(),
//                     leading: skia_font_metrics.leading.into(),
//                 }
//             };

//             Some(NativeFont {
//                 skia_font,
//                 metrics,
//                 glyph_ids_caches: Default::default(),
//                 glyph_widths_caches: Default::default(),
//                 glyph_bounds_caches: Default::default(),
//             })
//         })
//     }
// }

// impl NativeFont {
//     pub(crate) fn glyph_ids(&self, text: impl AsRef<str>) -> GlyphIds {
//         let text = text.as_ref().to_string();
//         if text.is_empty() {
//             return vec![];
//         }

//         self.glyph_ids_caches
//             .get_or_create(&text, |text| self.skia_font.str_to_glyphs_vec(text))
//             .to_vec()
//     }
//     pub(crate) fn glyph_widths(&self, glyph_ids: GlyphIds, paint: &Paint) -> Vec<Px> {
//         if glyph_ids.is_empty() {
//             return vec![];
//         }
//         self.glyph_widths_caches
//             .get_or_create(&(glyph_ids, paint.clone()), |(glyph_ids, paint)| {
//                 let native_paint = NativePaint::get(paint);

//                 let mut widths = vec![0.0; glyph_ids.len()];

//                 self.skia_font.get_widths_bounds(
//                     glyph_ids,
//                     Some(&mut widths),
//                     None,
//                     Some(native_paint.skia()),
//                 );

//                 widths.into_iter().map(|n| n.into()).collect()
//             })
//             .to_vec()
//     }
//     pub(crate) fn glyph_bounds(&self, glyph_ids: GlyphIds, paint: &Paint) -> Vec<Rect<Px>> {
//         if glyph_ids.is_empty() {
//             return vec![];
//         }

//         self.glyph_bounds_caches
//             .get_or_create(&(glyph_ids, paint.clone()), |(glyph_ids, paint)| {
//                 let native_paint = NativePaint::get(paint);

//                 let mut bounds = vec![skia_safe::Rect::default(); glyph_ids.len()];

//                 self.skia_font
//                     .get_bounds(glyph_ids, &mut bounds, Some(native_paint.skia()));

//                 bounds
//                     .into_iter()
//                     .map(|rect| Rect::Ltrb {
//                         left: rect.left.into(),
//                         top: rect.top.into(),
//                         right: rect.right.into(),
//                         bottom: rect.bottom.into(),
//                     })
//                     .collect()
//             })
//             .to_vec()
//     }
// }
