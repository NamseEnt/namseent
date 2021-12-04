use std::{collections::HashMap, rc::Rc};

use crate::engine::{
    manager::{FontManager, TypefaceManager},
    Font, FontType,
};

pub struct CommonFontManager {
    fonts: HashMap<FontType, Rc<dyn Font>>,
    typeface_manager: Rc<dyn TypefaceManager>,
}

impl FontManager for CommonFontManager {
    fn get_font(&self, font_type: &FontType) -> Option<Rc<dyn Font>> {
        self.fonts.get(font_type).map(|font| font.clone())
    }
}
