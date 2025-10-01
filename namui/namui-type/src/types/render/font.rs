use crate::*;

#[derive(Debug, bincode::Decode, bincode::Encode, PartialEq, Clone, Eq, Hash)]
pub struct Font {
    pub size: IntPx,
    pub name: String,
}
