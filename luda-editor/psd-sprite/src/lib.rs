mod layer_tree;

use anyhow::Result;
use layer_tree::make_tree;
use namui_type::*;
use std::collections::BTreeMap;

pub struct PartsSprite {
    pub name: String,
    pub parts: BTreeMap<String, SpritePart>,
}

impl PartsSprite {
    pub fn from_psd_bytes(psd_bytes: &[u8]) -> Result<Self> {
        let psd = psd::Psd::from_bytes(psd_bytes)?;

        let layer_trees = make_tree(&psd)?;

        todo!()
    }
}

pub struct SpritePart {
    pub name: String,
    pub kind: SpritePartKind,
    pub blend_mode: psd::BlendMode,
}

pub enum SpritePartKind {
    Fixed { image: SpriteImage },
    SingleSelect { options: Vec<SpritePartOption> },
    MultiSelect { options: Vec<SpritePartOption> },
}

pub struct SpritePartOption {
    pub name: String,
    pub blend_mode: psd::BlendMode,
    pub image: SpriteImage,
}

pub struct SpriteImage {
    pub dest_rect: Rect<Px>,
    pub webp: Box<[u8]>,
}
