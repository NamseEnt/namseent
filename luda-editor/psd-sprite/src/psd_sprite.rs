use crate::*;
use layer_tree::*;
use namui_type::*;
use psd::{BlendMode, IntoRgba};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
pub struct PsdSprite {
    pub entries: Vec<Entry>,
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

    pub fn image_encoded_byte_size(&self) -> usize {
        self.entries
            .iter()
            .map(|entry| entry.image_encoded_byte_size())
            .sum()
    }
    pub fn image_encoded_bytes(&self) -> Vec<Vec<u8>> {
        self.entries
            .par_iter()
            .flat_map(|entry| entry.image_encoded_bytes())
            .collect()
    }
}

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub blend_mode: BlendMode,
    pub clipping_base: bool,
    pub opacity: u8,
    pub mask: Option<SpriteImage>,
    pub kind: EntryKind,
}

impl Entry {
    pub fn image_encoded_byte_size(&self) -> usize {
        self.mask
            .as_ref()
            .map(|mask| mask.image_encoded_byte_size())
            .unwrap_or_default()
            + match &self.kind {
                EntryKind::Layer { image } => image.image_encoded_byte_size(),
                EntryKind::Group { entries } => entries
                    .iter()
                    .map(|entry| entry.image_encoded_byte_size())
                    .sum(),
            }
    }

    pub fn image_encoded_bytes(&self) -> Vec<Vec<u8>> {
        let mask = self
            .mask
            .as_ref()
            .map(|mask| mask.image_encoded_bytes())
            .unwrap_or_default();
        let image = match &self.kind {
            EntryKind::Layer { image } => image.image_encoded_bytes(),
            EntryKind::Group { entries } => entries
                .par_iter()
                .flat_map(|entry| entry.image_encoded_bytes())
                .collect(),
        };
        mask.into_iter().chain(image.into_iter()).collect()
    }
}

#[derive(Debug)]
pub enum EntryKind {
    Layer { image: SpriteImage },
    Group { entries: Vec<Entry> },
}

#[derive(Debug)]
pub struct SpriteImage {
    pub dest_rect: Rect<Px>,
    pub encoded: nimg::Nimg,
}

impl SpriteImage {
    pub fn decode(&self) -> anyhow::Result<(Vec<u8>, nimg::ColorType)> {
        Ok((self.encoded.decode()?, self.encoded.color_type))
    }

    pub fn image_encoded_byte_size(&self) -> usize {
        self.encoded.image_encoded_byte_size()
    }

    pub fn image_encoded_bytes(&self) -> Vec<Vec<u8>> {
        self.encoded.image_encoded_bytes()
    }
}
