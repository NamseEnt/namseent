use crate::*;
use ::image::ImageReader;
use namui::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use schema_0::SceneSprite;
use skia_safe::Data;
use std::{borrow::Borrow, collections::HashMap, io::Cursor, iter::Peekable};

impl PsdSprite {
    pub fn render(&self, ctx: &RenderCtx, scene_sprite: &SceneSprite, screen_wh: Wh<Px>) {
        let (image_filter, set_image_filter) = ctx.state(|| None);

        ctx.effect("create image filter", || {
            set_image_filter.set(create_image_filter(scene_sprite, self));
        });

        ctx.compose(|ctx| {
            let Some(image_filter) = image_filter.as_ref() else {
                return;
            };
            let SceneSprite { circumcircle, .. } = scene_sprite;
            let paint = Paint::default().set_image_filter(image_filter.clone());
            let ratio = screen_wh.length() * circumcircle.radius / self.wh.length();
            let sprite_wh = self.wh * ratio;
            let ctx = ctx.translate((
                screen_wh.width * circumcircle.xy.x - (sprite_wh.width / 2),
                screen_wh.height * circumcircle.xy.y - (sprite_wh.height / 2),
            ));
            let path = Path::new().add_rect(sprite_wh.to_rect());
            ctx.add(PathDrawCommand { path, paint });
        });
    }
}

impl SpriteImage {
    fn to_namui_image(&self) -> anyhow::Result<Image> {
        let image = ImageReader::new(Cursor::new(&self.webp))
            .with_guessed_format()?
            .decode()?;
        let rgba = image.to_rgba8().into_vec();
        skia_safe::image::images::raster_from_data(
            &skia_safe::ImageInfo::new_n32(
                (image.width() as i32, image.height() as i32),
                skia_safe::AlphaType::Unpremul,
                None,
            ),
            Data::new_copy(&rgba),
            image.width() as usize * 4,
        )
        .map(|sk_image| {
            Image::new(
                ImageInfo {
                    alpha_type: namui::AlphaType::Unpremul,
                    color_type: namui::ColorType::Rgba8888,
                    height: (image.height() as f32).px(),
                    width: (image.width() as f32).px(),
                },
                sk_image,
            )
        })
        .ok_or(anyhow::anyhow!("Failed to create image from SpriteImage"))
    }

    fn to_namui_image_a8(&self) -> anyhow::Result<Image> {
        let image = ImageReader::new(Cursor::new(&self.webp))
            .with_guessed_format()?
            .decode()?;
        let rgba = image.to_luma8().into_vec();
        skia_safe::image::images::raster_from_data(
            &skia_safe::ImageInfo::new_a8((image.width() as i32, image.height() as i32)),
            Data::new_copy(&rgba),
            image.width() as usize,
        )
        .map(|sk_image| {
            Image::new(
                ImageInfo {
                    alpha_type: namui::AlphaType::Unpremul,
                    color_type: namui::ColorType::Alpha8,
                    height: (image.height() as f32).px(),
                    width: (image.width() as f32).px(),
                },
                sk_image,
            )
        })
        .ok_or(anyhow::anyhow!(
            "Failed to create a8 image from SpriteImage"
        ))
    }
}

fn load_parts_sprite_images(sprite_part: &PsdSprite) -> Vec<(String, ImageFilter)> {
    let images = sprite_part
        .entries
        .par_iter()
        .flat_map(load_parts_sprite_images_from_entry)
        .collect();
    return images;

    fn load_parts_sprite_images_from_entry(entry: &Entry) -> Vec<(String, ImageFilter)> {
        match &entry.kind {
            EntryKind::Layer { image } => image
                .to_namui_image()
                .map(|src| {
                    let image_filter = ImageFilter::Image { src }.offset(image.dest_rect.xy());
                    vec![(entry.name.clone(), image_filter)]
                })
                .unwrap_or_default(),
            EntryKind::Group { entries } => entries
                .par_iter()
                .flat_map(load_parts_sprite_images_from_entry)
                .collect(),
        }
    }
}

fn load_parts_sprite_mask_images(sprite_part: &PsdSprite) -> Vec<(String, ImageFilter)> {
    let masks = sprite_part
        .entries
        .par_iter()
        .flat_map(load_parts_sprite_mask_images_from_entry)
        .collect();
    return masks;

    fn load_parts_sprite_mask_images_from_entry(entry: &Entry) -> Vec<(String, ImageFilter)> {
        let mut masks = entry
            .mask
            .as_ref()
            .map(|mask: &SpriteImage| {
                mask.to_namui_image_a8()
                    .map(|src| {
                        let image_filter = ImageFilter::Image { src }.offset(mask.dest_rect.xy());
                        vec![(entry.name.clone(), image_filter)]
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        match &entry.kind {
            EntryKind::Layer { .. } => masks,
            EntryKind::Group { entries } => {
                let child_masks: Vec<_> = entries
                    .par_iter()
                    .flat_map(load_parts_sprite_mask_images_from_entry)
                    .collect();
                masks.extend(child_masks);
                masks
            }
        }
    }
}

fn create_image_filter(
    scene_sprite: &schema_0::SceneSprite,
    psd_sprite: &PsdSprite,
) -> Option<ImageFilter> {
    let parts_sprite_images = HashMap::from_iter(load_parts_sprite_images(psd_sprite));
    let parts_sprite_mask_images = HashMap::from_iter(load_parts_sprite_mask_images(psd_sprite));
    render_entries(
        None,
        &psd_sprite.entries,
        scene_sprite,
        &parts_sprite_images,
        &parts_sprite_mask_images,
        &None,
        255,
    )
}

fn render_entries<T: Borrow<Entry>>(
    mut background: Option<ImageFilter>,
    entries: &[T],
    scene_sprite: &SceneSprite,
    parts_sprite_images: &HashMap<String, ImageFilter>,
    parts_sprite_mask_images: &HashMap<String, ImageFilter>,
    parent_mask: &Option<ImageFilter>,
    parent_opacity: u8,
) -> Option<ImageFilter> {
    let mut entries = entries.iter().rev().peekable();

    while let Some(entry) = entries.next() {
        let entry = <T as Borrow<Entry>>::borrow(entry);
        let blend_mode = entry.blend_mode;
        let passthrough = matches!(blend_mode, psd::BlendMode::PassThrough);
        let has_clipping_layer = has_clipping_layer(&mut entries);
        let mask = {
            let mask_image = parts_sprite_mask_images.get(&entry.name);
            match (parent_mask, mask_image) {
                (None, None) => None,
                (None, Some(mask)) | (Some(mask), None) => Some(mask.clone()),
                (Some(parent_mask), Some(mask_image)) => Some(ImageFilter::blend(
                    namui::BlendMode::DstIn,
                    parent_mask.clone(),
                    mask_image.clone(),
                )),
            }
        };

        let mut foreground = match &entry.kind {
            EntryKind::Layer { .. } => parts_sprite_images.get(&entry.name).cloned(),
            EntryKind::Group { entries } => match &entry.name {
                name if name.ends_with("_m") || name.ends_with("_s") => {
                    let entries =
                        if let Some(part_names) = scene_sprite.part_option_selections.get(name) {
                            entries
                                .iter()
                                .filter(|entry| part_names.contains(&entry.name))
                                .collect::<Vec<_>>()
                        } else {
                            vec![]
                        };
                    render_entries(
                        Some(ImageFilter::Empty),
                        &entries,
                        scene_sprite,
                        parts_sprite_images,
                        parts_sprite_mask_images,
                        &None,
                        255,
                    )
                }
                _ => {
                    if passthrough && !has_clipping_layer {
                        background = render_entries(
                            background,
                            entries,
                            scene_sprite,
                            parts_sprite_images,
                            parts_sprite_mask_images,
                            &mask,
                            entry.opacity,
                        );
                        continue;
                    }

                    render_entries(
                        Some(ImageFilter::Empty),
                        entries,
                        scene_sprite,
                        parts_sprite_images,
                        parts_sprite_mask_images,
                        &None,
                        255,
                    )
                }
            },
        };

        if let Some(mask) = mask {
            foreground = Some(ImageFilter::blend(
                namui::BlendMode::DstIn,
                foreground.unwrap_or_default(),
                mask,
            ));
        }

        if has_clipping_layer {
            let mask = foreground.clone();
            while let Some(entry) = entries.peek() {
                if !<T as Borrow<Entry>>::borrow(entry).clipping_base {
                    break;
                }
                let clipping_entry = entries.next().unwrap();
                foreground = render_entries(
                    foreground,
                    std::slice::from_ref(clipping_entry),
                    scene_sprite,
                    parts_sprite_images,
                    parts_sprite_mask_images,
                    &None,
                    255,
                );
            }
            foreground = Some(ImageFilter::blend(
                namui::BlendMode::DstIn,
                foreground.unwrap_or_default(),
                mask.unwrap_or_default(),
            ));
        }

        if entry.opacity != 255 || parent_opacity != 255 {
            let opacity = (entry.opacity as f32 / 255.0) * (parent_opacity as f32 / 255.0);
            let color_filter = ColorFilter::scale_matrix(1.0, 1.0, 1.0, opacity);
            foreground = Some(foreground.unwrap_or_default().color_filter(color_filter));
        }

        background = Some(ImageFilter::blend(
            photoshop_blend_mode_into_blender(blend_mode),
            background.unwrap_or_default(),
            foreground.unwrap_or_default(),
        ));
    }

    background
}

fn has_clipping_layer<T: Borrow<Entry>>(
    entries: &mut Peekable<std::iter::Rev<std::slice::Iter<T>>>,
) -> bool {
    entries
        .peek()
        .is_some_and(|parts_sprite| <T as Borrow<Entry>>::borrow(parts_sprite).clipping_base)
}

fn photoshop_blend_mode_into_blender(blend_mode: psd::BlendMode) -> namui::Blender {
    match blend_mode {
        // BlendMode::PassThrough => todo!(),
        // BlendMode::Dissolve => todo!(),
        psd::BlendMode::Normal => namui::BlendMode::SrcOver.into(),
        psd::BlendMode::Darken => namui::BlendMode::Darken.into(),
        psd::BlendMode::Multiply => namui::BlendMode::Multiply.into(),
        psd::BlendMode::ColorBurn => namui::BlendMode::ColorBurn.into(),
        psd::BlendMode::LinearBurn => namui::Blender::arithmetic(0.0, 1.0, 1.0, -1.0),
        psd::BlendMode::DarkerColor => {
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
            namui::Blender::Sksl(sksl.to_string())
        }
        psd::BlendMode::Lighten => namui::BlendMode::Lighten.into(),
        psd::BlendMode::Screen => namui::BlendMode::Screen.into(),
        psd::BlendMode::ColorDodge => namui::BlendMode::ColorDodge.into(),
        psd::BlendMode::LinearDodge => namui::Blender::arithmetic(0.0, 1.0, 1.0, 0.0),
        psd::BlendMode::LighterColor => {
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
            namui::Blender::Sksl(sksl.to_string())
        }
        psd::BlendMode::Overlay => namui::BlendMode::Overlay.into(),
        psd::BlendMode::SoftLight => namui::BlendMode::SoftLight.into(),
        psd::BlendMode::HardLight => namui::BlendMode::HardLight.into(),
        psd::BlendMode::VividLight => {
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
            namui::Blender::Sksl(sksl.to_string())
        }
        psd::BlendMode::LinearLight => {
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
            namui::Blender::Sksl(sksl.to_string())
        }
        psd::BlendMode::PinLight => {
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
            namui::Blender::Sksl(sksl.to_string())
        }
        psd::BlendMode::HardMix => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(min(floor(src.rgb + dst.rgb), 1), src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        psd::BlendMode::Difference => namui::BlendMode::Difference.into(),
        psd::BlendMode::Exclusion => namui::BlendMode::Exclusion.into(),
        psd::BlendMode::Subtract => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(dst.rgb - src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        psd::BlendMode::Divide => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(dst.rgb / src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        psd::BlendMode::Hue => namui::BlendMode::Hue.into(),
        psd::BlendMode::Saturation => namui::BlendMode::Saturation.into(),
        psd::BlendMode::Color => namui::BlendMode::Color.into(),
        psd::BlendMode::Luminosity => namui::BlendMode::Luminosity.into(),
        _ => namui::BlendMode::SrcOver.into(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use schema_0::Circumcircle;
    use std::collections::HashSet;

    #[test]
    fn test_create_image_filter() {
        let psd_bytes = include_bytes!("test.psd");
        let psd_sprite = PsdSprite::from_psd_bytes(psd_bytes).unwrap();
        let scene_sprite = SceneSprite {
            sprite_id: None,
            circumcircle: Circumcircle {
                xy: Xy::zero(),
                radius: 0.percent(),
            },
            part_option_selections: HashMap::from_iter([
                (
                    "눈_s".to_string(),
                    HashSet::from_iter(["눈_s.옆보는".to_string()]),
                ),
                (
                    "눈썹_s".to_string(),
                    HashSet::from_iter(["눈썹_s.슬픔".to_string()]),
                ),
                (
                    "입_s".to_string(),
                    HashSet::from_iter(["입_s.놀람".to_string()]),
                ),
                (
                    "코_s".to_string(),
                    HashSet::from_iter(["코_s.코".to_string()]),
                ),
                (
                    "홍조_s".to_string(),
                    HashSet::from_iter(["홍조_s.레이어 80".to_string()]),
                ),
            ]),
        };
        let _image_filter = create_image_filter(&scene_sprite, &psd_sprite).unwrap();
    }
}
