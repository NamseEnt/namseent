use super::InitResult;
use namui_type::*;
use std::sync::Arc;
// use crate::*;
// use dashmap::DashMap;
// use std::sync::Arc;

// struct FontSystem {
//     font_type_fonts: DashMap<Font, Arc<Font>>,
//     typeface_fonts: DashMap<String, Arc<Font>>,
// }

// lazy_static::lazy_static! {
//     static ref FONT_SYSTEM: Arc<FontSystem> = Arc::new(FontSystem {
//         font_type_fonts: DashMap::new(),
//         typeface_fonts: DashMap::new(),
//     });
// }

pub(super) async fn init() -> InitResult {
    //     lazy_static::initialize(&FONT_SYSTEM);
    Ok(())
}

/// None when font is not found.
pub fn font_metrics(font: &Font) -> Option<FontMetrics> {
    todo!()
}

pub fn group_glyph(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
    crate::system::skia::group_glyph(font, paint)
}

// pub fn get_font(font: Font) -> Option<Arc<Font>> {
//     let font = FONT_SYSTEM.font_type_fonts.get(&font);
//     match font {
//         Some(font) => Some(font.clone()),
//         None => match create_font_from_font_type(font) {
//             Ok(font) => {
//                 FONT_SYSTEM
//                     .font_type_fonts
//                     .insert(font.clone(), font.clone());
//                 Some(font)
//             }
//             Err(_) => None,
//         },
//     }
// }
// pub fn get_font_with_fallbacks(font: Font) -> Vec<Arc<Font>> {
//     let font = get_font(font);
//     let mut fonts = vec![];
//     if let Some(font) = font {
//         fonts.push(font);
//     }
//     fonts
//         .into_iter()
//         .chain(std::iter::once_with(|| get_fallback_fonts(font.size)).flatten())
//         .collect::<Vec<_>>()
// }
// pub fn with_fallbacks(font: Arc<Font>) -> Vec<Arc<Font>> {
//     let font_size = font.size;
//     std::iter::once(font)
//         .chain(std::iter::once_with(|| get_fallback_fonts(font_size)).flatten())
//         .collect::<Vec<_>>()
// }
// pub fn get_font_of_typeface(typeface: Arc<Typeface>, font_size: IntPx) -> Arc<Font> {
//     let key = Font::generate_id(&typeface, font_size);
//     let font = FONT_SYSTEM.typeface_fonts.get(&key);
//     match font {
//         Some(font) => font.clone(),
//         None => {
//             let font = crate_font(&typeface, font_size);
//             FONT_SYSTEM.typeface_fonts.insert(key, font.clone());
//             font
//         }
//     }
// }
// fn create_font_from_font_type(font: Font) -> Result<Arc<Font>, String> {
//     let typeface_type = TypefaceType {
//         font_weight: font.font_weight,
//         language: font.language,
//         serif: font.serif,
//     };
//     let typeface = crate::typeface::get_typeface(typeface_type);
//     match typeface {
//         Some(typeface) => Ok(crate_font(&typeface, font.size)),
//         None => Err(format!("Could not find typeface for {:?}", font)),
//     }
// }
// fn crate_font(typeface: &Typeface, font_size: IntPx) -> Arc<Font> {
//     Arc::new(Font::new(typeface, font_size))
// }
// pub fn get_fallback_fonts(font_size: IntPx) -> Vec<Arc<Font>> {
//     crate::typeface::get_fallback_font_typefaces()
//         .map(|typeface| crate::font::get_font_of_typeface(typeface.clone(), font_size))
//         .collect()
// }
