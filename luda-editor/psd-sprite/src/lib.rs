mod layer_tree;

use anyhow::Result;
use layer_tree::{into_sprite_parts, make_tree};
use namui_type::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct PartsSprite {
    pub name: String,
    pub parts: BTreeMap<String, SpritePart>,
}

impl PartsSprite {
    pub fn from_psd_bytes(psd_bytes: &[u8]) -> Result<Self> {
        let psd = psd::Psd::from_bytes(psd_bytes)?;

        let layer_trees = make_tree(&psd)?;
        let parts = into_sprite_parts(layer_trees, Vec::new())?
            .into_iter()
            .map(|part| (part.name.clone(), part));

        Ok(Self {
            name: "TODO".to_string(),
            parts: BTreeMap::from_iter(parts),
        })
    }
}

#[derive(Debug)]
pub struct SpritePart {
    pub name: String,
    pub kind: SpritePartKind,
    pub blend_mode: psd::BlendMode,
}

#[derive(Debug)]
pub enum SpritePartKind {
    Fixed { image: SpriteImage },
    SingleSelect { options: Vec<SpritePartOption> },
    MultiSelect { options: Vec<SpritePartOption> },
}

#[derive(Debug)]
pub struct SpritePartOption {
    pub name: String,
    pub blend_mode: psd::BlendMode,
    pub image: SpriteImage,
}

#[derive(Debug)]
pub struct SpriteImage {
    pub dest_rect: Rect<Px>,
    pub webp: Box<[u8]>,
}
