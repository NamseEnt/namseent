use std::{collections::HashMap, sync::Arc};

use crate::{
    namui::{manager::TypefaceManager, skia::Font, FontType, TypefaceType},
    Typeface,
};

pub struct FontManager {
    font_type_fonts: HashMap<FontType, Arc<Font>>,
    typeface_fonts: HashMap<String, Arc<Font>>,
    pub typeface_manager: TypefaceManager,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            font_type_fonts: HashMap::new(),
            typeface_fonts: HashMap::new(),
            typeface_manager: TypefaceManager::new(),
        }
    }
    pub fn get_font(&mut self, font_type: &FontType) -> Option<Arc<Font>> {
        let font = self.font_type_fonts.get(font_type);
        match font {
            Some(font) => Some(font.clone()),
            None => match self.create_font_from_font_type(font_type) {
                Ok(font) => {
                    self.font_type_fonts.insert(font_type.clone(), font.clone());
                    Some(font)
                }
                Err(_) => None,
            },
        }
    }
    pub fn get_font_of_typeface(&mut self, typeface: Arc<Typeface>, font_size: i16) -> Arc<Font> {
        let key = Font::generate_id(&typeface, font_size);
        let font = self.typeface_fonts.get(&key);
        match font {
            Some(font) => font.clone(),
            None => {
                let font = Self::crate_font(&typeface, font_size);
                self.typeface_fonts.insert(key, font.clone());
                font
            }
        }
    }
    fn create_font_from_font_type(&self, font_type: &FontType) -> Result<Arc<Font>, String> {
        let typeface_type = TypefaceType {
            font_weight: font_type.font_weight,
            language: font_type.language,
            serif: font_type.serif,
        };
        let typeface = self.typeface_manager.get_typeface(&typeface_type);
        match typeface {
            Some(typeface) => Ok(Self::crate_font(&typeface, font_type.size)),
            None => Err(format!("Could not find typeface for {:?}", font_type)),
        }
    }
    fn crate_font(typeface: &Typeface, font_size: i16) -> Arc<Font> {
        Arc::new(Font::new(typeface, font_size))
    }
}
