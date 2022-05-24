use dashmap::DashMap;

use crate::namui::{self, Typeface, TypefaceType};

pub struct TypefaceManager {
    typefaces: DashMap<TypefaceType, Typeface>,
}

impl TypefaceManager {
    pub fn new() -> Self {
        TypefaceManager {
            typefaces: DashMap::new(),
        }
    }
    pub fn get_typeface<'a>(&'a self, option: &'a TypefaceType) -> Option<Typeface> {
        namui::log(format!("typefaces: {:?}", self.typefaces.len()));
        self.typefaces.get(option).map(|x| x.clone())
    }
    pub fn load_typeface(&self, option: &TypefaceType, bytes: &Vec<u8>) {
        let typeface = Typeface::new(bytes);
        namui::log(format!("Loaded typeface: {:?}", option));

        self.typefaces.insert(*option, typeface);
    }
}
