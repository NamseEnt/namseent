use crate::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq, State)]
pub struct ImageDrawCommand {
    pub rect: Rect<Px>,
    pub image: Image,
    pub fit: ImageFit,
    pub paint: Option<Paint>,
}
