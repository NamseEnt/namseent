use std::rc::Rc;

use super::{Font, FontType, Typeface, TypefaceType, Xy};

#[cfg(target_family = "wasm")]
pub mod web;

#[cfg(target_family = "wasm")]
pub use web::*;

pub mod common;
pub use common::*;

pub trait MouseManager {
    fn mouse_position(&self) -> Xy<i16>;
}

pub trait FontManager {
    fn get_font(&self, font_type: &FontType) -> Option<Rc<dyn Font>>;
}

pub trait TypefaceManager {
    fn get_typeface(&self, option: TypefaceType) -> Option<Rc<dyn Typeface>>;
    fn load_typeface(&mut self, option: TypefaceType, bytes: &Vec<u8>);
}
