use namui_type::*;
use psd::BlendMode;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PsdSprite {
    pub entries: Vec<Entry>,
    pub wh: Wh<Px>,
}

impl PsdSprite {
    pub fn to_parts_sprite(&self, name: String) -> schema_0::PartsSprite {
        fn to_options(entries: &[Entry]) -> Vec<schema_0::SpritePartOption> {
            entries
                .iter()
                .map(|entry| schema_0::SpritePartOption {
                    name: entry.name.clone(),
                })
                .collect()
        }
        fn collect_parts(entry: &Entry) -> Vec<(String, schema_0::SpritePart)> {
            match &entry.kind {
                EntryKind::Layer { .. } => vec![],
                EntryKind::Group { entries } => {
                    let name = entry.name.clone();
                    match name {
                        name if name.ends_with("_m") => {
                            vec![(
                                name,
                                schema_0::SpritePart {
                                    name: entry.name.clone(),
                                    is_single_select: false,
                                    part_options: to_options(entries),
                                },
                            )]
                        }
                        name if name.ends_with("_s") => {
                            vec![(
                                name,
                                schema_0::SpritePart {
                                    name: entry.name.clone(),
                                    is_single_select: true,
                                    part_options: to_options(entries),
                                },
                            )]
                        }
                        _ => entries.par_iter().flat_map(collect_parts).collect(),
                    }
                }
            }
        }

        let parts = self.entries.par_iter().flat_map(collect_parts).collect();
        schema_0::PartsSprite { name, parts }
    }
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
