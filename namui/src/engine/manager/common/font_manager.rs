use std::{collections::HashMap, sync::Arc};

use crate::engine::{manager::TypefaceManager, skia::Font, FontType, TypefaceType};

pub struct FontManager {
    fonts: HashMap<FontType, Arc<Font>>,
    pub typeface_manager: TypefaceManager,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            typeface_manager: TypefaceManager::new(),
        }
    }
    pub fn get_font(&mut self, font_type: &FontType) -> Option<Arc<Font>> {
        let font = self.fonts.get(font_type);
        match font {
            Some(font) => Some(font.clone()),
            None => match self.create_font(font_type) {
                Ok(font) => {
                    let font = Arc::new(font);
                    self.fonts.insert(font_type.clone(), font.clone());
                    Some(font)
                }
                Err(_) => None,
            },
        }
    }
    fn create_font(&self, font_type: &FontType) -> Result<Font, String> {
        let typeface_type = TypefaceType {
            font_weight: font_type.font_weight,
            language: font_type.language,
            serif: font_type.serif,
        };
        let typeface = self.typeface_manager.get_typeface(&typeface_type);
        match typeface {
            Some(typeface) => {
                let font = Font::new(&typeface, &font_type.size);
                Ok(font)
            }
            None => Err(format!("Could not find typeface for {:?}", font_type)),
        }
    }
}
