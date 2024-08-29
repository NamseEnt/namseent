use crate::*;
use anyhow::Result;
use mask_image::MaskImage;
use namui_type::*;
use psd::{image_data_section::ChannelBytes, IntoRgba, PsdLayer, ToMask};
use rayon::prelude::*;
use std::borrow::Cow;

#[derive(Debug)]
pub enum LayerTree<'psd> {
    Group {
        group: &'psd psd::PsdGroup,
        children: Vec<LayerTree<'psd>>,
    },
    Layer {
        layer: &'psd psd::PsdLayer,
    },
}
impl LayerTree<'_> {
    fn rect(&self) -> Rect<i32> {
        match self {
            LayerTree::Group { children, .. } => {
                children.iter().fold(Rect::default(), |rect, child| {
                    rect.get_minimum_rectangle_containing(child.rect())
                })
            }
            LayerTree::Layer { layer } => Rect::Xywh {
                x: layer.layer_left(),
                y: layer.layer_top(),
                width: layer.width(),
                height: layer.height(),
            },
        }
    }

    fn get_mask(&self) -> Option<MaskImage> {
        let masks = match self {
            LayerTree::Group { group, .. } => (
                mask_to_sk_position_image(group.raster_mask()),
                mask_to_sk_position_image(group.vector_mask()),
            ),
            LayerTree::Layer { layer } => (
                mask_to_sk_position_image(layer.raster_mask()),
                mask_to_sk_position_image(layer.vector_mask()),
            ),
        };

        match masks {
            (None, None) => None,
            (None, Some(mask)) | (Some(mask), None) => Some(mask),
            (Some(raster_mask), Some(vector_mask)) => raster_mask.intersect(&vector_mask),
        }
    }
}
pub fn make_tree(psd: &psd::Psd) -> Vec<LayerTree> {
    let mut tree = vec![];

    for layer in psd.layers() {
        let group_children = open_group_children(psd, &mut tree, layer.parent_id());
        group_children.push(LayerTree::Layer { layer })
    }

    return tree;

    fn open_group_children<'psd, 'tree>(
        psd: &'psd psd::Psd,
        tree: &'tree mut Vec<LayerTree<'psd>>,
        group_id: Option<u32>,
    ) -> &'tree mut Vec<LayerTree<'psd>> {
        let group_ids_bottom_to_top = {
            let mut group_ids_bottom_to_top = vec![];
            let mut group = group_id.and_then(|group_id| psd.groups().get(&group_id));
            while let Some(group_) = group {
                group_ids_bottom_to_top.push(group_.id());
                group = group_
                    .parent_id()
                    .and_then(|parent_id| psd.groups().get(&parent_id));
            }
            group_ids_bottom_to_top
        };

        let group_children = group_ids_bottom_to_top
            .iter()
            .rev()
            .fold(tree, |tree, group_id| {
                let group_tree = if let Some(group_index) =
                    tree.iter_mut().position(|tree| match tree {
                        LayerTree::Group { group, .. } => group.id() == *group_id,
                        LayerTree::Layer { .. } => false,
                    }) {
                    &mut tree[group_index]
                } else {
                    tree.push(LayerTree::Group {
                        group: psd.groups().get(group_id).expect("No group exist"),
                        children: vec![],
                    });
                    tree.last_mut().unwrap()
                };

                match group_tree {
                    LayerTree::Group { children, .. } => children,
                    LayerTree::Layer { .. } => unreachable!("It should be group"),
                }
            });
        group_children
    }
}

fn layer_to_rgba_channels(layer: &PsdLayer) -> Result<[Cow<'_, [u8]>; 4]> {
    let r = layer.red().to_raw_data();
    let g = layer
        .green()
        .ok_or(anyhow::anyhow!("Green channel is missing in layer"))?
        .to_raw_data();
    let b = layer
        .blue()
        .ok_or(anyhow::anyhow!("Blue channel is missing in layer"))?
        .to_raw_data();
    let a = layer
        .alpha()
        .ok_or(anyhow::anyhow!("Alpha channel is missing in layer"))?
        .to_raw_data();
    Ok([r, g, b, a])
}

pub fn into_psd_sprite(layer_tree: Vec<LayerTree>, wh: Wh<Px>) -> Result<PsdSprite> {
    let entries = into_entries(layer_tree, vec![], wh.to_rect().map(|x| x.as_f32() as i32))?;
    Ok(PsdSprite { entries, wh })
}

fn into_entries(
    layer_tree: Vec<LayerTree>,
    prefixes: Vec<&str>,
    psd_rect: Rect<i32>,
) -> Result<Vec<Entry>> {
    layer_tree
        .into_iter()
        .map(|layer_tree| -> Result<Entry> {
            let mut prefixes = prefixes.clone();
            let layer_rect = layer_tree.rect();
            let mask = layer_tree
                .get_mask()
                .map(|mask| mask.to_sprite_image())
                .transpose()?;

            match layer_tree {
                LayerTree::Group { group, children } => {
                    prefixes.push(group.name());
                    let entries = into_entries(children, prefixes.clone(), psd_rect)?;
                    Ok(Entry {
                        name: prefixes.join("."),
                        blend_mode: group.blend_mode(),
                        clipping_base: !group.is_clipping_mask(),
                        opacity: group.opacity(),
                        mask,
                        kind: psd_sprite::EntryKind::Group { entries },
                    })
                }
                LayerTree::Layer { layer } => {
                    prefixes.push(layer.name());
                    if layer_rect.width() == 0
                        || layer_rect.height() == 0
                        || layer_rect.bottom() < 0
                        || layer_rect.right() < 0
                    {
                        return Err(anyhow::anyhow!("No layer to rasterize"));
                    }

                    let clipped_rect = layer_rect.intersect(psd_rect).unwrap_or_default();

                    let [r, g, b, a] = layer_to_rgba_channels(layer)?;

                    let psd_wh = psd_rect.wh().map(|x| x as usize);

                    let clipped_rgb = clip_and_interleave_channels([r, g, b], layer_rect, psd_wh);
                    let clipped_a = clip_and_interleave_channels([a], layer_rect, psd_wh);

                    let encoded_rgb = nimg::encode_rgb8(
                        clipped_rect.width() as usize,
                        clipped_rect.height() as usize,
                        &clipped_rgb,
                    )?;

                    let encoded_a = nimg::encode_a8(
                        clipped_rect.width() as usize,
                        clipped_rect.height() as usize,
                        &clipped_a,
                    )?;

                    Ok(Entry {
                        name: prefixes.join("."),
                        blend_mode: layer.blend_mode(),
                        clipping_base: !layer.is_clipping_mask(),
                        opacity: layer.opacity(),
                        mask,
                        kind: EntryKind::Layer {
                            image: SpriteImage {
                                dest_rect: Rect::Ltrb {
                                    left: layer.layer_left().px(),
                                    top: layer.layer_top().px(),
                                    right: layer.layer_right().px(),
                                    bottom: layer.layer_bottom().px(),
                                },
                                encoded: SpriteImageBuffer::Rgb8A8 {
                                    rgb: encoded_rgb,
                                    a: encoded_a,
                                },
                            },
                        },
                    })
                }
            }
        })
        .collect()
}

fn clip_and_interleave_channels<const N: usize>(
    channels: [Cow<'_, [u8]>; N],
    layer_rect: Rect<i32>,
    psd_wh: Wh<usize>,
) -> Vec<u8> {
    let overflowed_left = if layer_rect.left() < 0 {
        layer_rect.left().unsigned_abs() as usize
    } else {
        0
    };
    let overflowed_top = if layer_rect.top() < 0 {
        layer_rect.top().unsigned_abs() as usize
    } else {
        0
    };
    let clipped_width = (layer_rect.width() as usize - overflowed_left).min(psd_wh.width);
    let clipped_height = (layer_rect.height() as usize - overflowed_left).min(psd_wh.height);

    let mut vec = Vec::with_capacity(clipped_width * clipped_height * N);

    let layer_width = layer_rect.width() as usize;

    for y in 0..clipped_height {
        let y_index = (y + overflowed_top) * layer_width;
        for x in 0..clipped_width {
            for channel in &channels {
                let index = y_index + x + overflowed_left;
                vec.push(channel[index]);
            }
        }
    }
    vec
}

fn mask_to_sk_position_image(
    mask: Option<(&ChannelBytes, i32, i32, i32, i32)>,
) -> Option<MaskImage> {
    let (bytes, top, right, bottom, left) = mask?;

    clipped 시리즈 잘 고쳐봐. overflow를 계산해야해.
    let clipped_left = left.min(0).unsigned_abs() as usize;
    let clipped_top = top.min(0).unsigned_abs() as usize;
    let clipped_right = right.max(0) as usize;
    let clipped_bottom = bottom.max(0) as usize;

    let clipped_rect = Rect::Ltrb {
        left: 0,
        top: 0,
        right: clipped_right,
        bottom: clipped_bottom,
    };
    if clipped_rect.width() == 0 || clipped_rect.height() == 0 {
        return None;
    }
    let raw_data = bytes.to_raw_data();

    if left > 0 && top > 0 {
        return Some(MaskImage {
            dest_rect: clipped_rect,
            bytes: raw_data,
        });
    }

    let mut bytes = Vec::with_capacity(clipped_rect.width() * clipped_rect.height());

    for y in 0..clipped_rect.height() {
        let start = (y + clipped_top) * clipped_rect.width() + clipped_left;
        let end = start + clipped_rect.width();
        let src = &raw_data[start..end];
        bytes.extend_from_slice(src);
    }

    Some(MaskImage {
        dest_rect: clipped_rect,
        bytes: bytes.into(),
    })
}
