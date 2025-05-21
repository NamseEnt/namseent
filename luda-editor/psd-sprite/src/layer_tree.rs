use crate::*;
use anyhow::Result;
use namui_type::*;
use psd::{image_data_section::ChannelBytes, IntoRgba, PsdLayer, ToMask};
use rayon::prelude::*;
use sk_position_image::SkPositionImage;
use skia_safe::{Data, ImageInfo, Paint, Surface};
use std::collections::HashMap;

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

    fn get_mask(&self) -> Result<Option<SkPositionImage>> {
        let masks = match self {
            LayerTree::Group { group, .. } => (
                mask_to_sk_position_image(group.raster_mask())?,
                mask_to_sk_position_image(group.vector_mask())?,
            ),
            LayerTree::Layer { layer } => (
                mask_to_sk_position_image(layer.raster_mask())?,
                mask_to_sk_position_image(layer.vector_mask())?,
            ),
        };

        match masks {
            (None, None) => Ok(None),
            (None, Some(mask)) | (Some(mask), None) => Ok(Some(mask)),
            (Some(raster_mask), Some(vector_mask)) => {
                Ok(raster_mask.intersect_as_mask(&vector_mask))
            }
        }
    }
}
pub(crate) fn make_tree(psd: &psd::Psd) -> Vec<LayerTree> {
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

fn layer_to_rgba(layer: &PsdLayer) -> Result<Vec<u8>> {
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
    let mut rgba = Vec::with_capacity(r.len() * 4);
    for i in 0..r.len() {
        rgba.push(r[i]);
        rgba.push(g[i]);
        rgba.push(b[i]);
        rgba.push(a[i]);
    }
    Ok(rgba)
}

fn layer_to_sk_image(layer: &PsdLayer) -> Result<skia_safe::Image> {
    let rgba = layer_to_rgba(layer)?;

    skia_safe::image::images::raster_from_data(
        &ImageInfo::new_n32(
            (layer.width(), layer.height()),
            skia_safe::AlphaType::Unpremul,
            None,
        ),
        Data::new_copy(&rgba),
        layer.width() as usize * 4,
    )
    .ok_or(anyhow::anyhow!("Failed to create image from layer"))
}

pub(crate) type SpriteImages = HashMap<SpriteImageId, Vec<u8>>;
type SpriteImagesMutex = std::sync::Arc<std::sync::Mutex<SpriteImages>>;

pub(crate) fn into_psd_sprite(
    layer_tree: Vec<LayerTree>,
    wh: Wh<Px>,
) -> Result<(PsdSprite, SpriteImages)> {
    let images = SpriteImagesMutex::default();
    let entries = into_entries(
        layer_tree,
        vec![],
        wh.to_rect().map(|x| x.as_f32() as i32),
        images.clone(),
    )?;
    Ok((
        PsdSprite { entries, wh },
        std::sync::Arc::try_unwrap(images)
            .unwrap()
            .into_inner()
            .unwrap(),
    ))
}

fn into_entries(
    layer_tree: Vec<LayerTree>,
    prefixes: Vec<&str>,
    psd_rect: Rect<i32>,
    images: SpriteImagesMutex,
) -> Result<Vec<Entry>> {
    layer_tree
        .into_par_iter()
        .map_with(images, |images, layer_tree| -> Result<Entry> {
            let mut prefixes = prefixes.clone();
            let layer_rect = layer_tree.rect();

            match &layer_tree {
                LayerTree::Group { group, .. } => {
                    prefixes.push(group.name());
                }
                LayerTree::Layer { layer } => {
                    prefixes.push(layer.name());
                }
            }

            let name = prefixes.join(".");

            let mask = 'outer: {
                let Some(mask) = layer_tree.get_mask()? else {
                    break 'outer None;
                };

                let id = SpriteImageId::Mask {
                    prefix: name.clone(),
                };

                let encoded = encode_sk_position_image(&mask)?;

                images.lock().unwrap().insert(id.clone(), encoded);

                Some(mask.to_sprite_image(id))
            };

            match layer_tree {
                LayerTree::Group { group, children } => {
                    let entries =
                        into_entries(children, prefixes.clone(), psd_rect, images.clone())?;
                    Ok(Entry {
                        name,
                        blend_mode: group.blend_mode(),
                        clipping_base: !group.is_clipping_mask(),
                        opacity: group.opacity(),
                        mask,
                        kind: psd_sprite::EntryKind::Group { entries },
                    })
                }
                LayerTree::Layer { layer } => {
                    if layer_rect.width() == 0 || layer_rect.height() == 0 {
                        return Err(anyhow::anyhow!("No layer to rasterize"));
                    }
                    let clipped_rect = layer_rect.intersect(psd_rect).unwrap_or_default();
                    let bottom_image_info = ImageInfo::new_n32(
                        (clipped_rect.width(), clipped_rect.height()),
                        skia_safe::AlphaType::Unpremul,
                        None,
                    );
                    let mut surface: Surface =
                        skia_safe::surfaces::raster(&bottom_image_info, None, None).unwrap();
                    let canvas = surface.canvas();
                    canvas.translate((-layer_rect.left(), -layer_rect.top()));
                    if let std::result::Result::Ok(image) = layer_to_sk_image(layer) {
                        let paint = Paint::default();
                        canvas.draw_image(
                            image,
                            (layer.layer_left(), layer.layer_top()),
                            Some(&paint),
                        );
                    };
                    let image = surface.image_snapshot();
                    let encoded = crate::encode::encode_image(&image)?;
                    let id = SpriteImageId::Layer {
                        prefix: name.clone(),
                    };
                    images.lock().unwrap().insert(id.clone(), encoded.clone());

                    Ok(Entry {
                        name: name.clone(),
                        blend_mode: layer.blend_mode(),
                        clipping_base: !layer.is_clipping_mask(),
                        opacity: layer.opacity(),
                        mask,
                        kind: EntryKind::Layer {
                            image: SpriteImage {
                                id,
                                dest_rect: Rect::Ltrb {
                                    left: layer.layer_left().px(),
                                    top: layer.layer_top().px(),
                                    right: layer.layer_right().px(),
                                    bottom: layer.layer_bottom().px(),
                                },
                            },
                        },
                    })
                }
            }
        })
        .collect()
}

fn mask_to_sk_position_image(
    mask: Option<(&ChannelBytes, i32, i32, i32, i32)>,
) -> Result<Option<SkPositionImage>> {
    let Some((bytes, top, right, bottom, left)) = mask else {
        return Ok(None);
    };
    let rect = Rect::Ltrb {
        left,
        top,
        right,
        bottom,
    };
    if rect.width() == 0 || rect.height() == 0 {
        return Ok(None);
    }
    let raw_data = bytes.to_raw_data();
    let bottom_image_info = ImageInfo::new_a8((rect.width(), rect.height()));
    let sk_image = skia_safe::image::images::raster_from_data(
        &bottom_image_info,
        Data::new_copy(&raw_data),
        rect.width() as _,
    )
    .ok_or(anyhow::anyhow!("Failed to create image from mask"))?;

    Ok(Some(SkPositionImage {
        dest_rect: Rect::Ltrb {
            left,
            top,
            right,
            bottom,
        },
        sk_image,
    }))
}

fn encode_sk_position_image(position_image: &SkPositionImage) -> Result<Vec<u8>> {
    let mut surface: Surface =
        skia_safe::surfaces::raster(position_image.sk_image.image_info(), None, None).ok_or(
            anyhow::anyhow!("Failed to create surface from SkPositionImage"),
        )?;
    let canvas = surface.canvas();
    canvas.translate((
        -position_image.dest_rect.left(),
        -position_image.dest_rect.top(),
    ));

    canvas.draw_image(
        &position_image.sk_image,
        (
            position_image.dest_rect.left(),
            position_image.dest_rect.top(),
        ),
        Some(&Paint::default()),
    );
    let image = surface.image_snapshot();
    let encoded = crate::encode::encode_image(&image)?;

    Ok(encoded)
}
