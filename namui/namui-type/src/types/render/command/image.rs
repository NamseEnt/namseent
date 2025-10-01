use crate::*;

#[derive(Debug, bincode::Decode, bincode::Encode, PartialEq, Clone, Hash, Eq)]
pub struct ImageDrawCommand {
    pub rect: Rect<Px>,
    pub image: Image,
    pub fit: ImageFit,
    pub paint: Option<Paint>,
}
