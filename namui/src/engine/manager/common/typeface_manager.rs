use crate::engine::{self, Typeface, TypefaceType};
use std::collections::HashMap;

pub struct TypefaceManager {
    typefaces: HashMap<TypefaceType, Box<Typeface>>,
}

impl TypefaceManager {
    pub fn new() -> Self {
        TypefaceManager {
            typefaces: HashMap::new(),
        }
    }
    pub fn get_typeface(&self, option: &TypefaceType) -> Option<&Typeface> {
        engine::log(format!("typefaces: {:?}", self.typefaces.len()));
        self.typefaces
            .get(option)
            .map(|x| -> &Typeface { x.as_ref() })
    }
    pub fn load_typeface(&mut self, option: &TypefaceType, bytes: &Vec<u8>) {
        let typeface = Typeface::new(bytes);
        engine::log(format!("Loaded typeface: {:?}", option));

        self.typefaces.insert(*option, Box::new(typeface));
    }
}
