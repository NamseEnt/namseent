use namui_type::*;
use psd::BlendMode;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PsdSprite {
    pub entries: Vec<Entry>,
    pub wh: Wh<Px>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Entry {
    pub name: String,
    pub blend_mode: BlendMode,
    pub clipping_base: bool,
    pub opacity: u8,
    pub mask: Option<SpriteImage>,
    pub kind: EntryKind,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum EntryKind {
    Layer { image: SpriteImage },
    Group { entries: Vec<Entry> },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SpriteImage {
    pub id: SpriteImageId,
    pub dest_rect: Rect<Px>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum SpriteImageId {
    Mask { prefix: String },
    Layer { prefix: String },
}
