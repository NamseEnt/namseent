mod asset;
mod layer_tree;
mod sk_position_image;
pub mod skia_util;

use anyhow::{Ok, Result};
use layer_tree::make_tree;
use namui_type::*;
use psd::BlendMode;
use skia_safe::{
    canvas::{SaveLayerFlags, SaveLayerRec},
    Data, Image, ImageInfo, Paint, Surface,
};
use skia_util::{set_photoshop_blend_mode, sk_image_to_webp, AutoRestoreCanvas};
use std::{
    borrow::Borrow,
    collections::{BTreeMap, BTreeSet, HashMap},
    io::Cursor,
    iter::Peekable,
};

#[derive(Debug)]
pub struct PartsSprite {
    pub name: String,
    pub parts: BTreeMap<String, SpritePart>,
}

// PartsSprite 를 직접 만들지 말고 new-server/database/schema/0/src/asset/sprite/parts.rs 를 사용해주세요.

impl PartsSprite {
    pub fn from_parts_sprite_resource(parts_sprite_resource: &PartsSpriteAsset) -> Result<Self> {
        fn collect_parts(parts_sprite_resource: &PartsSpriteAsset) -> Vec<SpritePart> {
            match &parts_sprite_resource.kind {
                SpritePartKind::Fixed { .. } => {
                    vec![]
                }
                SpritePartKind::SingleSelect { options } => {
                    let options = options
                        .par_iter()
                        .map(|option| SpritePartManifestOption {
                            name: option.name.clone(),
                        })
                        .collect();
                    vec![SpritePart {
                        name: parts_sprite_resource.name.to_string(),
                        kind: SpritePartOption::SingleSelect { options },
                    }]
                }
                SpritePartKind::MultiSelect { options } => {
                    let options = options
                        .par_iter()
                        .map(|option| SpritePartManifestOption {
                            name: option.name.clone(),
                        })
                        .collect();
                    vec![SpritePart {
                        name: parts_sprite_resource.name.to_string(),
                        kind: SpritePartOption::MultiSelect { options },
                    }]
                }
                SpritePartKind::Directory { entries } => entries
                    .par_iter()
                    .flat_map(|entry| collect_parts(entry))
                    .collect(),
            }
        }
        let parts = collect_parts(parts_sprite_resource)
            .into_iter()
            .map(|part| (part.name.clone(), part));
        Ok(Self {
            name: parts_sprite_resource.name.to_string(),
            parts: BTreeMap::from_iter(parts),
        })
    }
}

#[derive(Debug)]
pub struct SpritePart {
    pub name: String,
    pub is_single_select: bool,
    pub options: Vec<String>,
}

pub struct PartsSpriteLock {
    pub name: String,
    pub part_names: BTreeSet<String>,
}
