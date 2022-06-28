use super::InitResult;
use crate::{
    namui::{skia::Font, FontType, TypefaceType},
    Typeface,
};
use dashmap::DashMap;
use std::sync::Arc;

struct FontSystem {
    font_type_fonts: DashMap<FontType, Arc<Font>>,
    typeface_fonts: DashMap<String, Arc<Font>>,
}

lazy_static::lazy_static! {
    static ref FONT_SYSTEM: Arc<FontSystem> = Arc::new(FontSystem {
        font_type_fonts: DashMap::new(),
        typeface_fonts: DashMap::new(),
    });
}

pub(super) async fn init() -> InitResult {
    lazy_static::initialize(&FONT_SYSTEM);
    Ok(())
}

pub fn get_font(font_type: FontType) -> Option<Arc<Font>> {
    let font = FONT_SYSTEM.font_type_fonts.get(&font_type);
    match font {
        Some(font) => Some(font.clone()),
        None => match create_font_from_font_type(font_type) {
            Ok(font) => {
                FONT_SYSTEM
                    .font_type_fonts
                    .insert(font_type.clone(), font.clone());
                Some(font)
            }
            Err(_) => None,
        },
    }
}
pub fn get_font_of_typeface(typeface: Arc<Typeface>, font_size: i16) -> Arc<Font> {
    let key = Font::generate_id(&typeface, font_size);
    let font = FONT_SYSTEM.typeface_fonts.get(&key);
    match font {
        Some(font) => font.clone(),
        None => {
            let font = crate_font(&typeface, font_size);
            FONT_SYSTEM.typeface_fonts.insert(key, font.clone());
            font
        }
    }
}
fn create_font_from_font_type(font_type: FontType) -> Result<Arc<Font>, String> {
    let typeface_type = TypefaceType {
        font_weight: font_type.font_weight,
        language: font_type.language,
        serif: font_type.serif,
    };
    let typeface = crate::typeface::get_typeface(typeface_type);
    match typeface {
        Some(typeface) => Ok(crate_font(&typeface, font_type.size)),
        None => Err(format!("Could not find typeface for {:?}", font_type)),
    }
}
fn crate_font(typeface: &Typeface, font_size: i16) -> Arc<Font> {
    Arc::new(Font::new(typeface, font_size))
}
