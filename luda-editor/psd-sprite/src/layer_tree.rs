use crate::*;
use anyhow::Result;
use image::Rgba;
use psd::{layer, IntoRgba, PsdLayer};
use rayon::prelude::*;
use skia_safe::{Data, ImageInfo};
use std::{collections::VecDeque, io::Cursor};

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
    pub fn name(&self) -> &str {
        match self {
            LayerTree::Group { group, .. } => group.name(),
            LayerTree::Layer { layer } => layer.name(),
        }
    }
    pub fn has_no_selection(&self) -> bool {
        match self {
            LayerTree::Group { .. } => {
                let mut child_group_queue = VecDeque::new();
                child_group_queue.push_back(self);
                while let Some(child_group) = child_group_queue.pop_front() {
                    match child_group {
                        LayerTree::Group { group, children } => {
                            let name = group.name();
                            let child_has_selection_group =
                                name.ends_with("_s") || name.ends_with("_m");
                            if child_has_selection_group {
                                return false;
                            }

                            for child_group in children.iter().filter(|child| match child {
                                LayerTree::Group { .. } => true,
                                LayerTree::Layer { .. } => false,
                            }) {
                                child_group_queue.push_back(child_group)
                            }
                        }
                        LayerTree::Layer { .. } => unreachable!("It should be group"),
                    }
                }
                true
            }
            LayerTree::Layer { .. } => true,
        }
    }
    pub fn is_clipping(&self) -> bool {
        match self {
            LayerTree::Group { group, .. } => group.is_clipping_mask(),
            LayerTree::Layer { layer } => layer.is_clipping_mask(),
        }
    }
    fn calculate_alpha(&self, force_visible: bool) -> f32 {
        let (visible, opacity) = match self {
            LayerTree::Group { group, .. } => (!group.visible(), group.opacity()),
            LayerTree::Layer { layer } => (!layer.visible(), layer.opacity()),
        };
        match force_visible || visible {
            true => (opacity as f32) / 255.0,
            false => 0.0,
        }
    }
    pub(crate) fn blend_mode(&self) -> psd::BlendMode {
        match self {
            LayerTree::Group { group, .. } => group.blend_mode(),
            LayerTree::Layer { layer } => layer.blend_mode(),
        }
    }
    fn get_image_buffer(&self, psd: &psd::Psd) -> RenderResult {
        match self {
            LayerTree::Group { children, .. } => render_layer_tree(psd, children, false),
            LayerTree::Layer { layer } => {
                let whole_layer_image_buffer =
                    image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_vec(
                        psd.width(),
                        psd.height(),
                        layer.rgba(),
                    )
                    .expect("Failed to create image buffer");

                let left = layer.layer_left();
                let top = layer.layer_top();
                let right = left + layer.width() as i32;
                let bottom = top + layer.height() as i32;

                let left_inside_psd = left.clamp(0, psd.width() as i32) as u32;
                let top_inside_psd = top.clamp(0, psd.height() as i32) as u32;
                let right_inside_psd = (right.max(0) as u32).min(psd.width());
                let bottom_inside_psd = (bottom.max(0) as u32).min(psd.height());

                let x = left_inside_psd;
                let y = top_inside_psd;
                let width = (right_inside_psd - left_inside_psd).min(layer.width() as u32);
                let height = (bottom_inside_psd - top_inside_psd).min(layer.height() as u32);

                let cropped =
                    image::imageops::crop_imm(&whole_layer_image_buffer, x, y, width, height);

                RenderResult {
                    x,
                    y,
                    image_buffer: cropped.to_image(),
                }
            }
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

rayon(스레드풀)을 적극 이용해서
이미지들을 skia를 통해 blend 하고
그것을 webp로 인코딩해서
Vec<SpritePart> 만들기

pub fn into_sprite_parts(layer_tree: Vec<LayerTree>) -> Result<Vec<SpritePart>> {
    let a = layer_tree
        .into_par_iter()
        .map(|layer_tree| -> Result<SpritePart> {
            match layer_tree {
                LayerTree::Group { group, children } => {
                    todo!() 여기 마저 하세요~
                }
                LayerTree::Layer { layer } => {
                    let rgba = layer_to_rgba(layer)?;

                    let image_buffer = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_vec(
                        layer.width() as u32,
                        layer.height() as u32,
                        rgba,
                    )
                    .ok_or(anyhow::anyhow!("Failed to create image buffer from layer"))?;

                    let mut webp_bytes: Vec<u8> = Vec::new();
                    image_buffer
                        .write_to(&mut Cursor::new(&mut webp_bytes), image::ImageFormat::WebP)?;

                    Ok(SpritePart {
                        name: layer.name().to_string(),
                        blend_mode: layer.blend_mode(),
                        kind: SpritePartKind::Fixed {
                            image: SpriteImage {
                                dest_rect: Rect::Xywh {
                                    x: layer.layer_left().px(),
                                    y: layer.layer_top().px(),
                                    width: layer.width().px(),
                                    height: layer.height().px(),
                                },
                                webp: webp_bytes.into_boxed_slice(),
                            },
                        },
                    })
                }
            }
        });

    todo!()
}

// pub fn into_sprite_parts(layer_tree: Vec<LayerTree>) -> Result<Vec<SpritePart>> {
//     let a = layer_tree
//         .into_par_iter()
//         .map(|layer_tree| -> Result<SpritePart> {
//             match layer_tree {
//                 LayerTree::Group { item, children } => {
//                     todo!()
//                 }
//                 LayerTree::Layer { item } => {
//                     let r = item.red().to_raw_data();
//                     let g = item
//                         .green()
//                         .ok_or(anyhow::anyhow!("Green channel is missing in layer"))?
//                         .to_raw_data();
//                     let b = item
//                         .blue()
//                         .ok_or(anyhow::anyhow!("Blue channel is missing in layer"))?
//                         .to_raw_data();
//                     let a = item
//                         .alpha()
//                         .ok_or(anyhow::anyhow!("Alpha channel is missing in layer"))?
//                         .to_raw_data();
//                     let rgba = {
//                         let mut rgba = Vec::with_capacity(r.len() * 4);
//                         for i in 0..r.len() {
//                             rgba.push(r[i]);
//                             rgba.push(g[i]);
//                             rgba.push(b[i]);
//                             rgba.push(a[i]);
//                         }
//                         rgba
//                     };

//                     let skia_image = skia_safe::image::images::raster_from_data(
//                         &ImageInfo::new_n32(
//                             (item.width() as i32, item.height() as i32),
//                             skia_safe::AlphaType::Unpremul,
//                             None,
//                         ),
//                         Data::new_copy(&rgba),
//                         item.width() as usize * 4,
//                     )
//                     .ok_or(anyhow::anyhow!("Failed to create image from layer"))?;

//                     Ok(SpritePart {
//                         name: item.name().to_string(),
//                         blend_mode: item.blend_mode(),
//                         kind: crate::SpritePartKind::Fixed { image: () },
//                     })
//                 }
//             }
//         });

//     todo!()
// }

// pub fn gogo(psd: &psd::Psd) -> Result<()> {
//     psd.groups()
//     todo!()
// }
pub(crate) struct RenderResult {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}
pub(crate) fn render_layer_tree<'psd>(
    psd: &'psd psd::Psd,
    layer_tree: &Vec<LayerTree<'psd>>,
    force_visible: bool,
) -> RenderResult {
    let mut image_buffers = layer_tree
        .par_iter()
        .rev()
        .map(|layer_tree: &LayerTree<'_>| layer_tree.get_image_buffer(psd))
        .collect::<Vec<_>>()
        .into_iter();

    let mut layer_tree_iter = layer_tree.iter().rev().peekable();
    let mut bottom: Option<RenderResult> = None;

    while let Some(upper_layer_tree) = layer_tree_iter.next() {
        assert!(upper_layer_tree.is_clipping());

        let mut upper = image_buffers.next().unwrap();

        apply_alpha(
            &mut upper.image_buffer,
            upper_layer_tree.calculate_alpha(force_visible),
        );

        while layer_tree_iter
            .peek()
            .is_some_and(|layer_tree| !layer_tree.is_clipping())
        {
            let clipping_layer_tree = layer_tree_iter.next().unwrap();

            let mut clipping = clip_buffer(&image_buffers.next().unwrap(), &upper);
            apply_alpha(
                &mut clipping.image_buffer,
                clipping_layer_tree.calculate_alpha(false),
            );

            upper = blend_buffer(&clipping, &upper, clipping_layer_tree.blend_mode());
        }

        match &mut bottom {
            Some(bottom) => {
                *bottom = blend_buffer(&upper, bottom, upper_layer_tree.blend_mode());
            }
            None => {
                bottom = Some(upper);
            }
        }
    }

    match bottom {
        Some(bottom) => bottom,
        None => RenderResult {
            x: 0,
            y: 0,
            image_buffer: image::ImageBuffer::<Rgba<u8>, Vec<u8>>::new(0, 0),
        },
    }
}

fn clip_buffer(source: &RenderResult, destination: &RenderResult) -> RenderResult {
    let source_rect = namui_type::Rect::Xywh {
        x: source.x,
        y: source.y,
        width: source.image_buffer.width(),
        height: source.image_buffer.height(),
    };
    let destination_rect = namui_type::Rect::Xywh {
        x: destination.x,
        y: destination.y,
        width: destination.image_buffer.width(),
        height: destination.image_buffer.height(),
    };
    let clipped_rect = source_rect.intersect(destination_rect).unwrap_or_default();

    let vec = (clipped_rect.y()..clipped_rect.y() + clipped_rect.height())
        .into_par_iter()
        .flat_map(|y| {
            (clipped_rect.x()..clipped_rect.x() + clipped_rect.width())
                .flat_map(move |x| {
                    let source_pixel = source.image_buffer.get_pixel(x - source.x, y - source.y);
                    let destination_alpha = destination
                        .image_buffer
                        .get_pixel(x - destination.x, y - destination.y)
                        .0[3] as f32
                        / 255.0;
                    [
                        source_pixel.0[0],
                        source_pixel.0[1],
                        source_pixel.0[2],
                        (source_pixel.0[3] as f32 * destination_alpha) as u8,
                    ]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<u8>>();

    let image_buffer = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_vec(
        clipped_rect.width(),
        clipped_rect.height(),
        vec,
    )
    .unwrap();

    RenderResult {
        x: clipped_rect.x(),
        y: clipped_rect.y(),
        image_buffer,
    }
}

fn blend_buffer(
    source: &RenderResult,
    destination: &RenderResult,
    blend_mode: psd::BlendMode,
) -> RenderResult {
    let source_rect = namui_type::Rect::Xywh {
        x: source.x,
        y: source.y,
        width: source.image_buffer.width(),
        height: source.image_buffer.height(),
    };
    let destination_rect = namui_type::Rect::Xywh {
        x: destination.x,
        y: destination.y,
        width: destination.image_buffer.width(),
        height: destination.image_buffer.height(),
    };
    let blended_rect = source_rect.get_minimum_rectangle_containing(destination_rect);
    let blend_function = blend_function(blend_mode);

    let vec = (blended_rect.y()..blended_rect.y() + blended_rect.height())
        .into_par_iter()
        .flat_map(|y| {
            (blended_rect.x()..blended_rect.x() + blended_rect.width())
                .flat_map(move |x| {
                    let default = Rgba([0, 0, 0, 0]);

                    let source_pixel = source
                        .image_buffer
                        .get_pixel_checked(x - source.x, y - source.y)
                        .unwrap_or(&default);
                    let destination_pixel = destination
                        .image_buffer
                        .get_pixel_checked(x - destination.x, y - destination.y)
                        .unwrap_or(&default);

                    blend_pixel(source_pixel, destination_pixel, blend_function).0
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    let image_buffer = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_vec(
        blended_rect.width(),
        blended_rect.height(),
        vec,
    )
    .unwrap();

    RenderResult {
        x: blended_rect.x(),
        y: blended_rect.y(),
        image_buffer,
    }
}

fn apply_alpha(image_buffer: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, alpha: f32) {
    image_buffer
        .as_flat_samples_mut()
        .samples
        .par_chunks_mut(4)
        .for_each(|pixel| {
            pixel[3] = (pixel[3] as f32 * alpha) as u8;
        });
}

fn blend_pixel(
    source_pixel: &Rgba<u8>,
    destination_pixel: &Rgba<u8>,
    blend_function: BlendFunction,
) -> Rgba<u8> {
    fn alpha_blend(
        source_color: f32,
        source_alpha: f32,
        destination_color: f32,
        destination_alpha: f32,
        blend_function: BlendFunction,
        blended_alpha: f32,
    ) -> u8 {
        let blended_source_color = (1.0 - destination_alpha) * source_color
            + destination_alpha * blend_function(source_color, destination_color);
        let blended_color = source_alpha * blended_source_color
            + destination_alpha * destination_color * (1.0 - source_alpha);
        ((blended_color / blended_alpha).clamp(0.0, 1.0) * 255.0).round() as u8
    }

    let source_alpha = source_pixel.0[3] as f32 / 255.0;
    let destination_alpha = destination_pixel.0[3] as f32 / 255.0;
    let blended_alpha = source_alpha + destination_alpha * (1.0 - source_alpha);
    Rgba([
        alpha_blend(
            source_pixel.0[0] as f32 / 255.0,
            source_alpha,
            destination_pixel.0[0] as f32 / 255.0,
            destination_alpha,
            blend_function,
            blended_alpha,
        ),
        alpha_blend(
            source_pixel.0[1] as f32 / 255.0,
            source_alpha,
            destination_pixel.0[1] as f32 / 255.0,
            destination_alpha,
            blend_function,
            blended_alpha,
        ),
        alpha_blend(
            source_pixel.0[2] as f32 / 255.0,
            source_alpha,
            destination_pixel.0[2] as f32 / 255.0,
            destination_alpha,
            blend_function,
            blended_alpha,
        ),
        (blended_alpha.clamp(0.0, 1.0) * 255.0).round() as u8,
    ])
}

type BlendFunction = fn(f32, f32) -> f32;
fn blend_function(blend_mode: psd::BlendMode) -> BlendFunction {
    return match blend_mode {
        psd::BlendMode::PassThrough => todo!(),
        psd::BlendMode::Normal => normal,
        psd::BlendMode::Dissolve => todo!(),
        psd::BlendMode::Darken => todo!(),
        psd::BlendMode::Multiply => multiply,
        psd::BlendMode::ColorBurn => todo!(),
        psd::BlendMode::LinearBurn => linear_burn,
        psd::BlendMode::DarkerColor => todo!(),
        psd::BlendMode::Lighten => todo!(),
        psd::BlendMode::Screen => screen,
        psd::BlendMode::ColorDodge => todo!(),
        psd::BlendMode::LinearDodge => linear_dodge,
        psd::BlendMode::LighterColor => todo!(),
        psd::BlendMode::Overlay => overlay,
        psd::BlendMode::SoftLight => todo!(),
        psd::BlendMode::HardLight => hard_light,
        psd::BlendMode::VividLight => todo!(),
        psd::BlendMode::LinearLight => todo!(),
        psd::BlendMode::PinLight => todo!(),
        psd::BlendMode::HardMix => todo!(),
        psd::BlendMode::Difference => todo!(),
        psd::BlendMode::Exclusion => todo!(),
        psd::BlendMode::Subtract => todo!(),
        psd::BlendMode::Divide => todo!(),
        psd::BlendMode::Hue => todo!(),
        psd::BlendMode::Saturation => todo!(),
        psd::BlendMode::Color => todo!(),
        psd::BlendMode::Luminosity => todo!(),
    };

    fn normal(source: f32, _destination: f32) -> f32 {
        source
    }
    fn multiply(source: f32, destination: f32) -> f32 {
        source * destination
    }
    fn linear_burn(source: f32, destination: f32) -> f32 {
        (source + destination - 1.0).max(0.0)
    }
    fn screen(source: f32, destination: f32) -> f32 {
        1.0 - (1.0 - source) * (1.0 - destination)
    }
    fn linear_dodge(source: f32, destination: f32) -> f32 {
        source + destination
    }
    fn overlay(source: f32, destination: f32) -> f32 {
        match destination < 0.5 {
            true => 2.0 * source * destination,
            false => 1.0 - (2.0 * (1.0 - source) * (1.0 - destination)),
        }
    }
    fn hard_light(source: f32, destination: f32) -> f32 {
        match source < 0.5 {
            true => 2.0 * source * destination,
            false => 1.0 - (2.0 * (1.0 - source) * (1.0 - destination)),
        }
    }
}
