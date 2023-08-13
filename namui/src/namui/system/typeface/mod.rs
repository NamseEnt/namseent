mod load_sans_typeface_of_all_languages;
use super::InitResult;
use load_sans_typeface_of_all_languages::*;

// pub struct TypefaceSystem {
//     typefaces: DashMap<TypefaceType, Arc<Typeface>>,
//     fallback_font_typefaces: DashMap<FontFamily, Arc<Typeface>>,
// }

// lazy_static::lazy_static! {
//     static ref TYPEFACE_SYSTEM: Arc<TypefaceSystem> = Arc::new(TypefaceSystem {
//         typefaces: DashMap::new(),
//         fallback_font_typefaces: DashMap::new(),
//     });
// }

pub(super) async fn init() -> InitResult {
    // lazy_static::initialize(&TYPEFACE_SYSTEM);
    load_all_typefaces().await?;
    Ok(())
}

pub(crate) fn register_typeface(typeface_name: &str, bytes: &[u8]) {
    crate::system::drawer::load_typeface(typeface_name, bytes);
    crate::system::skia::load_typeface(typeface_name, bytes);
}

// pub fn get_typeface(option: TypefaceType) -> Option<Arc<Typeface>> {
//     TYPEFACE_SYSTEM
//         .typefaces
//         .get(&option)
//         .map(|typeface| typeface.clone())
// }

// pub(crate) fn load_typeface(option: &TypefaceType, typeface: Arc<Typeface>) {
//     TYPEFACE_SYSTEM.typefaces.insert(*option, typeface.clone());
// }

// pub fn get_fallback_font_typefaces<'a>() -> impl Iterator<Item = RefMulti<'a, String, Arc<Typeface>>>
// {
//     TYPEFACE_SYSTEM.fallback_font_typefaces.iter()
// }

// pub(crate) fn load_fallback_font_typeface(font_family: FontFamily, typeface: Arc<Typeface>) {
//     TYPEFACE_SYSTEM
//         .fallback_font_typefaces
//         .insert(font_family, typeface);
// }
