use crate::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq, bincode::Encode, bincode::Decode)]
pub struct ImageDrawCommand {
    pub rect: Rect<Px>,
    pub image: Image,
    pub fit: ImageFit,
    pub paint: Option<Paint>,
}
