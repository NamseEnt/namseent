use crate::*;
use anyhow::Result;
use image::ImageBuffer;
use psd::{BlendMode, IntoRgba, PsdLayer};
use rayon::prelude::*;
use skia_safe::{
    canvas::{SaveLayerFlags, SaveLayerRec},
    Blender, Data, ImageInfo, Paint, RuntimeEffect, Surface,
};
use std::{io::Cursor, iter::Peekable};

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
    pub fn name(&self) -> &str {
        match self {
            LayerTree::Group { group, .. } => group.name(),
            LayerTree::Layer { layer } => layer.name(),
        }
    }
    pub fn is_clipping(&self) -> bool {
        !match self {
            LayerTree::Group { group, .. } => group.is_clipping_mask(),
            LayerTree::Layer { layer } => layer.is_clipping_mask(),
        }
    }
    pub(crate) fn blend_mode(&self) -> psd::BlendMode {
        match self {
            LayerTree::Group { group, .. } => group.blend_mode(),
            LayerTree::Layer { layer } => layer.blend_mode(),
        }
    }
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
    fn opacity(&self) -> u8 {
        match self {
            LayerTree::Group { group, .. } => group.opacity(),
            LayerTree::Layer { layer } => layer.opacity(),
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

pub fn into_sprite_parts(
    layer_tree: Vec<LayerTree>,
    prefixes: Vec<&str>,
) -> Result<Vec<SpritePart>> {
    layer_tree
        .into_par_iter()
        .map(|layer_tree| -> Result<SpritePart> {
            let mut prefixes = prefixes.clone();
            if let LayerTree::Group { group, children } = &layer_tree {
                if group.name().ends_with("_m") {
                    prefixes.push(group.name());
                    let options = generate_options(children, prefixes)?;
                    let kind = SpritePartKind::MultiSelect { options };
                    return Ok(SpritePart {
                        name: group.name().to_string(),
                        blend_mode: group.blend_mode(),
                        kind,
                    });
                }

                if group.name().ends_with("_s") {
                    prefixes.push(group.name());
                    let options = generate_options(children, prefixes)?;
                    let kind = SpritePartKind::SingleSelect { options };
                    return Ok(SpritePart {
                        name: group.name().to_string(),
                        blend_mode: group.blend_mode(),
                        kind,
                    });
                }
            }

            let image = rasterize_layer_tree(&layer_tree)?;
            prefixes.push(layer_tree.name());
            Ok(SpritePart {
                name: prefixes.join("."),
                blend_mode: layer_tree.blend_mode(),
                kind: SpritePartKind::Fixed { image },
            })
        })
        .collect()
}

fn generate_options(children: &[LayerTree], prefixes: Vec<&str>) -> Result<Vec<SpritePartOption>> {
    children
        .par_iter()
        .map(|child| -> Result<SpritePartOption> {
            let image = rasterize_layer_tree(child)?;
            let mut prefixes = prefixes.clone();
            prefixes.push(child.name());
            Ok(SpritePartOption {
                name: prefixes.join("."),
                blend_mode: child.blend_mode(),
                image,
            })
        })
        .collect()
}

fn rasterize_layer_tree(layer_tree: &LayerTree) -> Result<SpriteImage> {
    let layer_rect = layer_tree.rect();
    if layer_rect.width() == 0 || layer_rect.height() == 0 {
        return Err(anyhow::anyhow!("No layer to rasterize"));
    }

    let bottom_image_info = ImageInfo::new_n32(
        (layer_rect.width(), layer_rect.height()),
        skia_safe::AlphaType::Unpremul,
        None,
    );
    let mut bottom_surface: Surface =
        skia_safe::surfaces::raster(&bottom_image_info, None, None).unwrap();
    let bottom_canvas = bottom_surface.canvas();
    bottom_canvas.translate((-layer_rect.left(), -layer_rect.top()));
    render_layer_trees_to_canvas(bottom_canvas, std::slice::from_ref(layer_tree))?;

    let image = bottom_surface.image_snapshot();
    let row_bytes = layer_rect.width() as usize * 4;
    let mut pixels = vec![0; layer_rect.height() as usize * row_bytes];
    image.read_pixels(
        &bottom_image_info,
        &mut pixels,
        row_bytes,
        (0, 0),
        skia_safe::image::CachingHint::Disallow,
    );
    let image_buffer = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_vec(
        layer_rect.width() as _,
        layer_rect.height() as _,
        pixels,
    )
    .ok_or(anyhow::anyhow!("Failed to create image buffer from layer"))?;
    let mut webp_bytes: Vec<u8> = Vec::new();
    image_buffer.write_to(&mut Cursor::new(&mut webp_bytes), image::ImageFormat::WebP)?;

    Ok(SpriteImage {
        dest_rect: Rect::Xywh {
            x: layer_rect.x().px(),
            y: layer_rect.y().px(),
            width: layer_rect.width().px(),
            height: layer_rect.height().px(),
        },
        webp: Box::from(webp_bytes.as_ref()),
    })
}

fn has_clipping_layer(
    layer_tree: &mut Peekable<std::iter::Rev<std::slice::Iter<LayerTree>>>,
) -> bool {
    layer_tree
        .peek()
        .is_some_and(|layer_tree| layer_tree.is_clipping())
}

fn render_layer_trees_to_canvas(
    canvas: &skia_safe::Canvas,
    layer_trees: &[LayerTree],
) -> Result<()> {
    let _auto_restore = AutoRestoreCanvas::new(canvas);
    let mut layer_trees = layer_trees.into_iter().rev().peekable();

    while let Some(layer_tree) = layer_trees.next() {
        let _auto_restore = AutoRestoreCanvas::new(canvas);
        let blend_mode = layer_tree.blend_mode();
        {
            let paint = create_paint_from_layer_tree(layer_tree);
            let save_layer_rec = SaveLayerRec::default().paint(&paint);
            canvas.save_layer(&save_layer_rec);
        }

        match layer_tree {
            LayerTree::Group { children, .. } => {
                if matches!(blend_mode, BlendMode::PassThrough) {
                    canvas.restore();
                }
                render_layer_trees_to_canvas(canvas, children.as_slice())?;
            }
            LayerTree::Layer { layer } => {
                let Ok(image) = layer_to_sk_image(layer) else {
                    // Maybe layer is empty
                    continue;
                };
                let paint = Paint::default();
                canvas.draw_image(image, (layer.layer_left(), layer.layer_top()), Some(&paint));
            }
        }

        if has_clipping_layer(&mut layer_trees) {
            let _auto_restore = AutoRestoreCanvas::new(canvas);
            {
                let mut paint = Paint::default();
                paint.set_blend_mode(skia_safe::BlendMode::SrcATop);
                let save_layer_rec = SaveLayerRec::default()
                    .flags(SaveLayerFlags::INIT_WITH_PREVIOUS)
                    .paint(&paint);
                canvas.save_layer(&save_layer_rec);
            }
            while let Some(clipping_layer_tree) = layer_trees.peek() {
                if !clipping_layer_tree.is_clipping() {
                    break;
                }
                let clipping_layer_tree = layer_trees.next().unwrap();
                render_layer_trees_to_canvas(canvas, std::slice::from_ref(clipping_layer_tree))?;
            }
        }
    }
    Ok(())
}

fn create_paint_from_layer_tree(layer_tree: &LayerTree) -> skia_safe::Paint {
    let mut paint = Paint::default();

    set_blend_mode(&mut paint, layer_tree.blend_mode());
    paint.set_alpha(layer_tree.opacity());

    paint
}

fn set_blend_mode(paint: &mut Paint, blend_mode: BlendMode) {
    match blend_mode {
        // BlendMode::PassThrough => todo!(),
        BlendMode::Normal => paint.set_blend_mode(skia_safe::BlendMode::SrcOver),
        // BlendMode::Dissolve => todo!(),
        BlendMode::Darken => paint.set_blend_mode(skia_safe::BlendMode::Darken),
        BlendMode::Multiply => paint.set_blend_mode(skia_safe::BlendMode::Multiply),
        BlendMode::ColorBurn => paint.set_blend_mode(skia_safe::BlendMode::ColorBurn),
        BlendMode::LinearBurn => {
            let blender = Blender::arithmetic(0.0, 1.0, 1.0, -1.0, false);
            paint.set_blender(blender)
        }
        BlendMode::DarkerColor => {
            let sksl = r#"
                vec4 BRIGHTNESS_MAP = vec4(0.299, 0.587, 0.114, 0.0);
                vec4 main(vec4 src, vec4 dst) {
                    float src_brightness, dst_brightness;
                    vec4 new_src;

                    src_brightness = dot(src, BRIGHTNESS_MAP);
                    dst_brightness = dot(dst, BRIGHTNESS_MAP);
                    new_src = vec4(src_brightness > dst_brightness ? dst.rgb : src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            paint.set_blender(blender)
        }
        BlendMode::Lighten => paint.set_blend_mode(skia_safe::BlendMode::Lighten),
        BlendMode::Screen => paint.set_blend_mode(skia_safe::BlendMode::Screen),
        BlendMode::ColorDodge => paint.set_blend_mode(skia_safe::BlendMode::ColorDodge),
        BlendMode::LinearDodge => {
            let blender = Blender::arithmetic(0.0, 1.0, 1.0, 0.0, false);
            paint.set_blender(blender)
        }
        BlendMode::LighterColor => {
            let sksl = r#"
                vec4 BRIGHTNESS_MAP = vec4(0.299, 0.587, 0.114, 0.0);
                vec4 main(vec4 src, vec4 dst) {
                    float src_brightness, dst_brightness;
                    vec4 new_src;

                    src_brightness = dot(src, BRIGHTNESS_MAP);
                    dst_brightness = dot(dst, BRIGHTNESS_MAP);
                    new_src = vec4(src_brightness > dst_brightness ? src.rgb : dst.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            paint.set_blender(blender)
        }
        BlendMode::Overlay => paint.set_blend_mode(skia_safe::BlendMode::Overlay),
        BlendMode::SoftLight => paint.set_blend_mode(skia_safe::BlendMode::SoftLight),
        BlendMode::HardLight => paint.set_blend_mode(skia_safe::BlendMode::HardLight),
        BlendMode::VividLight => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    for (int i = 0; i < 3; i++) {
                        if (src[i] <= 0.5) {
                            new_src[i] = max(0, 1 - (1 - dst[i]) / (2 * src[i]));
                        } else {
                            new_src[i] = min(1, dst[i] / (2 * (1 - src[i])));
                        }
                    }
                    new_src.a = src.a;

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            paint.set_blender(blender)
        }
        BlendMode::LinearLight => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    for (int i = 0; i < 3; i++) {
                        if (src[i] <= 0.5) {
                            new_src[i] = dst[i] + 2 * src[i] - 1;
                        } else {
                            new_src[i] = dst[i] + 2 * (src[i] - 0.5);
                        }
                    }
                    new_src.a = src.a;

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            paint.set_blender(blender)
        }
        BlendMode::PinLight => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    for (int i = 0; i < 3; i++) {
                        if (src[i] > 0.5) {
                            new_src[i] = max(dst[i], 2 * (src[i] - 0.5));
                        } else {
                            new_src[i] = min(dst[i], 2 * src[i]);
                        }
                    }
                    new_src.a = src.a;

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            paint.set_blender(blender)
        }
        BlendMode::HardMix => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(min(floor(src.rgb + dst.rgb), 1), src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            paint.set_blender(blender)
        }
        BlendMode::Difference => paint.set_blend_mode(skia_safe::BlendMode::Difference),
        BlendMode::Exclusion => paint.set_blend_mode(skia_safe::BlendMode::Exclusion),
        BlendMode::Subtract => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(dst.rgb - src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            paint.set_blender(blender)
        }
        BlendMode::Divide => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(dst.rgb / src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            paint.set_blender(blender)
        }
        BlendMode::Hue => paint.set_blend_mode(skia_safe::BlendMode::Hue),
        BlendMode::Saturation => paint.set_blend_mode(skia_safe::BlendMode::Saturation),
        BlendMode::Color => paint.set_blend_mode(skia_safe::BlendMode::Color),
        BlendMode::Luminosity => paint.set_blend_mode(skia_safe::BlendMode::Luminosity),
        // TODO: implement other blend modes
        _ => &mut paint.set_blend_mode(skia_safe::BlendMode::SrcOver),
    };
}

struct AutoRestoreCanvas<'canvas> {
    canvas: &'canvas skia_safe::Canvas,
    save_count: usize,
}
impl<'canvas> AutoRestoreCanvas<'canvas> {
    fn new(canvas: &'canvas skia_safe::Canvas) -> Self {
        let save_count = canvas.save_count();
        Self { canvas, save_count }
    }
}
impl Drop for AutoRestoreCanvas<'_> {
    fn drop(&mut self) {
        self.canvas.restore_to_count(self.save_count);
    }
}
