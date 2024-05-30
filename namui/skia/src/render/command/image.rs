use crate::*;

#[type_derives(Hash, Eq,-serde::Serialize, -serde::Deserialize)]
pub struct ImageDrawCommand {
    pub rect: Rect<Px>,
    pub image: Image,
    pub fit: ImageFit,
    pub paint: Option<Paint>,
}
