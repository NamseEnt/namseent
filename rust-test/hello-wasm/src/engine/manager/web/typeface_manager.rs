use std::{collections::HashMap, rc::Rc};

use crate::canvas_kit::*;
use crate::engine::manager::TypefaceManager;
use crate::engine::{Typeface, TypefaceType};

impl Typeface for CanvasKitTypeFace {}

pub struct WebTypefaceManager {
    typefaces: HashMap<TypefaceType, Rc<dyn Typeface>>,
    canvas_kit_font_mgr: FontMgr,
}

impl TypefaceManager for WebTypefaceManager {
    fn get_typeface(&self, option: TypefaceType) -> Option<Rc<dyn Typeface>> {
        self.typefaces.get(&option).cloned()
    }
    fn load_typeface(&mut self, option: TypefaceType, bytes: &Vec<u8>) {
        let array_buffer = js_sys::ArrayBuffer::new(bytes.len() as u32);

        let array_buffer_view = js_sys::Uint8Array::new(&array_buffer);
        array_buffer_view.copy_from(bytes);

        let typeface = self.canvas_kit_font_mgr.MakeTypefaceFromData(array_buffer);

        self.typefaces.insert(option, Rc::new(typeface));
    }
}
