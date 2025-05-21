use namui_type::*;
use psd::BlendMode;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashMap, hash::Hash};

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PsdSpritePart {
    pub is_single_select: bool,
    pub options: Vec<String>,
}

impl PsdSprite {
    pub fn parts(&self) -> HashMap<String, PsdSpritePart> {
        return self.entries.par_iter().flat_map(collect_parts).collect();

        fn collect_parts(entry: &Entry) -> Vec<(String, PsdSpritePart)> {
            match &entry.kind {
                EntryKind::Layer { .. } => vec![],
                EntryKind::Group { entries } => {
                    let name = entry.name.clone();
                    match name {
                        name if name.ends_with("_m") => {
                            vec![(
                                name,
                                PsdSpritePart {
                                    is_single_select: false,
                                    options: to_options(entries),
                                },
                            )]
                        }
                        name if name.ends_with("_s") => {
                            vec![(
                                name,
                                PsdSpritePart {
                                    is_single_select: true,
                                    options: to_options(entries),
                                },
                            )]
                        }
                        _ => entries.par_iter().flat_map(collect_parts).collect(),
                    }
                }
            }
        }
        fn to_options(entries: &[Entry]) -> Vec<String> {
            entries.iter().map(|entry| entry.name.clone()).collect()
        }
    }
}
