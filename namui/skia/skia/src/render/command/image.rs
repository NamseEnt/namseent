use crate::*;

#[type_derives()]
pub struct ImageDrawCommand {
    pub rect: Rect<Px>,
    pub image: Image,
    pub fit: ImageFit,
    pub paint: Option<Paint>,
}
