use crate::*;

#[type_derives()]
pub struct ImageDrawCommand {
    pub rect: Rect<Px>,
    pub source: ImageSource,
    pub fit: ImageFit,
    pub paint: Option<Paint>,
}
