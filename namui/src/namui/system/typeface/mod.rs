use crate::namui::{Typeface, TypefaceType};
use dashmap::{mapref::multiple::RefMulti, DashMap};
use std::sync::Arc;
mod load_sans_typeface_of_all_languages;
use load_sans_typeface_of_all_languages::*;

type FontFamily = String;

pub struct TypefaceSystem {
    typefaces: DashMap<TypefaceType, Arc<Typeface>>,
    fallback_font_typefaces: DashMap<FontFamily, Arc<Typeface>>,
}

lazy_static::lazy_static! {
    static ref TYPEFACE_SYSTEM: Arc<TypefaceSystem> = Arc::new(TypefaceSystem {
        typefaces: DashMap::new(),
        fallback_font_typefaces: DashMap::new(),
    });
}

pub(super) async fn init() -> Result<(), Box<dyn std::error::Error>> {
    lazy_static::initialize(&TYPEFACE_SYSTEM);
    load_all_typefaces().await
}

pub fn get_typeface(option: TypefaceType) -> Option<Arc<Typeface>> {
    TYPEFACE_SYSTEM
        .typefaces
        .get(&option)
        .map(|typeface| typeface.clone())
}

pub(crate) fn load_typeface(option: &TypefaceType, typeface: Arc<Typeface>) {
    TYPEFACE_SYSTEM.typefaces.insert(*option, typeface.clone());
}

pub fn get_fallback_font_typefaces<'a>() -> impl Iterator<Item = RefMulti<'a, String, Arc<Typeface>>>
{
    TYPEFACE_SYSTEM.fallback_font_typefaces.iter()
}

pub(crate) fn load_fallback_font_typeface(font_family: FontFamily, typeface: Arc<Typeface>) {
    TYPEFACE_SYSTEM
        .fallback_font_typefaces
        .insert(font_family, typeface);
}
