use crate::namui::{Typeface, TypefaceType};
use dashmap::{setref::multiple::RefMulti, DashMap, DashSet};
use std::sync::Arc;
mod load_sans_typeface_of_all_languages;
use load_sans_typeface_of_all_languages::*;

pub struct TypefaceSystem {
    typefaces: DashMap<TypefaceType, Arc<Typeface>>,
    fallback_font_typefaces: DashSet<Arc<Typeface>>,
}

lazy_static::lazy_static! {
    static ref TYPEFACE_SYSTEM: Arc<TypefaceSystem> = Arc::new(TypefaceSystem {
        typefaces: DashMap::new(),
        fallback_font_typefaces: DashSet::new(),
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

pub(crate) fn load_typeface(option: &TypefaceType, bytes: &impl AsRef<[u8]>) {
    let typeface = Typeface::new(bytes);
    crate::log!("Loaded typeface: {:?}", option);

    TYPEFACE_SYSTEM
        .typefaces
        .insert(*option, Arc::new(typeface));
}

pub fn get_fallback_font_typefaces<'a>() -> impl Iterator<Item = RefMulti<'a, Arc<Typeface>>> {
    TYPEFACE_SYSTEM.fallback_font_typefaces.iter()
}

pub(crate) fn load_fallback_font_typeface(typeface: Typeface) {
    TYPEFACE_SYSTEM
        .fallback_font_typefaces
        .insert(Arc::new(typeface));
}
