use crate::*;
use layer_tree::*;
use namui_type::*;
use psd::{BlendMode, IntoRgba};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
pub struct PsdSprite {
    pub(crate) entries: Vec<Entry>,
    pub wh: Wh<Px>,
}
impl PsdSprite {
    pub fn from_psd_bytes(psd_bytes: &[u8]) -> anyhow::Result<Self> {
        let psd = psd::Psd::from_bytes(psd_bytes)?;
        let wh = Wh::new(psd.psd_width(), psd.psd_height()).map(|x| (x as f32).px());
        let layer_trees = make_tree(&psd);
        layer_tree::into_psd_sprite(layer_trees, wh)
    }

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

#[derive(Debug)]
#[cfg_attr(not(feature = "namui_render"), allow(dead_code))]
pub(crate) struct Entry {
    pub(crate) name: String,
    pub(crate) blend_mode: BlendMode,
    pub(crate) clipping_base: bool,
    pub(crate) opacity: u8,
    pub(crate) mask: Option<SpriteImage>,
    pub(crate) kind: EntryKind,
}

#[derive(Debug)]
#[cfg_attr(not(feature = "namui_render"), allow(dead_code))]
pub(crate) enum EntryKind {
    Layer { image: SpriteImage },
    Group { entries: Vec<Entry> },
}

#[derive(Debug)]
#[cfg_attr(not(feature = "namui_render"), allow(dead_code))]
pub(crate) struct SpriteImage {
    pub(crate) dest_rect: Rect<Px>,
    pub(crate) webp: Box<[u8]>,
}
