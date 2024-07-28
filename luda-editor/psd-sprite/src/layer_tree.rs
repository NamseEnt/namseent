use crate::*;
use anyhow::Result;
use psd::{IntoRgba, PsdLayer};
use rayon::prelude::*;
use skia_safe::{Data, ImageInfo, Paint, Surface};
use skia_util::sk_image_to_webp;

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
}
pub fn make_tree(psd: &psd::Psd) -> anyhow::Result<Vec<LayerTree>> {
    let mut tree = vec![];

    for layer in psd.layers() {
        let group_children = open_group_children(psd, &mut tree, layer.parent_id())?;
        group_children.push(LayerTree::Layer { layer })
    }

    return Ok(tree);

    fn open_group_children<'psd, 'tree>(
        psd: &'psd psd::Psd,
        tree: &'tree mut Vec<LayerTree<'psd>>,
        group_id: Option<u32>,
    ) -> anyhow::Result<&'tree mut Vec<LayerTree<'psd>>> {
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
        Ok(group_children)
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

    Ok(skia_safe::image::images::raster_from_data(
        &ImageInfo::new_n32(
            (layer.width() as i32, layer.height() as i32),
            skia_safe::AlphaType::Unpremul,
            None,
        ),
        Data::new_copy(&rgba),
        layer.width() as usize * 4,
    )
    .ok_or(anyhow::anyhow!("Failed to create image from layer"))?)
}

pub fn into_parts_sprite(layer_tree: Vec<LayerTree>, name: String) -> Result<PartsSprite> {
    let entries = into_sprite_parts(layer_tree, vec![])?;
    let rect = entries
        .iter()
        .fold(None, |rect, part| {
            let Some(rect) = rect else {
                return Some(part.rect);
            };
            Some(rect.get_minimum_rectangle_containing(part.rect))
        })
        .unwrap_or_default();

    Ok(PartsSprite {
        name,
        kind: SpritePartKind::Directory { entries },
        blend_mode: BlendMode::Normal,
        clipping_base: true,
        opacity: 255,
        rect,
    })
}

fn into_sprite_parts(layer_tree: Vec<LayerTree>, prefixes: Vec<&str>) -> Result<Vec<PartsSprite>> {
    layer_tree
        .into_par_iter()
        .map(|layer_tree| -> Result<PartsSprite> {
            let mut prefixes = prefixes.clone();
            let rect = layer_tree.rect();
            let rect_px = Rect::Xywh {
                x: rect.x().px(),
                y: rect.y().px(),
                width: rect.width().px(),
                height: rect.height().px(),
            };
            match layer_tree {
                LayerTree::Group { group, children } => {
                    prefixes.push(group.name());
                    let parts = into_sprite_parts(children, prefixes.clone())?;
                    let kind = match group.name() {
                        name if name.ends_with("_m") => {
                            SpritePartKind::MultiSelect { options: parts }
                        }
                        name if name.ends_with("_s") => {
                            SpritePartKind::SingleSelect { options: parts }
                        }
                        _ => SpritePartKind::Directory { entries: parts },
                    };
                    return Ok(PartsSprite {
                        name: prefixes.join("."),
                        kind,
                        blend_mode: group.blend_mode(),
                        clipping_base: !group.is_clipping_mask(),
                        opacity: group.opacity(),
                        rect: rect_px,
                    });
                }
                LayerTree::Layer { layer } => {
                    prefixes.push(layer.name());
                    if rect.width() == 0 || rect.height() == 0 {
                        return Err(anyhow::anyhow!("No layer to rasterize"));
                    }
                    let bottom_image_info = ImageInfo::new_n32(
                        (rect.width(), rect.height()),
                        skia_safe::AlphaType::Unpremul,
                        None,
                    );
                    let mut surface: Surface =
                        skia_safe::surfaces::raster(&bottom_image_info, None, None).unwrap();
                    let canvas = surface.canvas();
                    canvas.translate((-rect.left(), -rect.top()));
                    if let std::result::Result::Ok(image) = layer_to_sk_image(layer) {
                        let paint = Paint::default();
                        canvas.draw_image(
                            image,
                            (layer.layer_left(), layer.layer_top()),
                            Some(&paint),
                        );
                    };
                    let image = surface.image_snapshot();
                    let webp_bytes = sk_image_to_webp(&image)?;

                    Ok(PartsSprite {
                        name: prefixes.join("."),
                        blend_mode: layer.blend_mode(),
                        kind: SpritePartKind::Fixed {
                            image: SpriteImage {
                                dest_rect: Rect::Xywh {
                                    x: rect.x().px(),
                                    y: rect.y().px(),
                                    width: rect.width().px(),
                                    height: rect.height().px(),
                                },
                                webp: Box::from(webp_bytes.as_ref()),
                            },
                        },
                        clipping_base: !layer.is_clipping_mask(),
                        opacity: layer.opacity(),
                        rect: rect_px,
                    })
                }
            }
        })
        .collect()
}
