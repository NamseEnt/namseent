use crate::*;

#[type_derives(Eq, Hash)]
pub struct Font {
    pub size: IntPx,
    pub name: String,
}
