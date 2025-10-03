use crate::*;

#[derive(Debug, PartialEq, Clone, Eq, Hash, bincode::Encode, bincode::Decode)]
pub struct Font {
    pub size: IntPx,
    pub name: String,
}
