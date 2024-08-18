use crate::*;
use layer_tree::*;
use namui::{ColorFilter, Image, ImageFilter, ImageInfo};
use namui_type::*;
use psd::{BlendMode, IntoRgba};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use schema_0::SceneSprite;
use skia_safe::Data;
use skia_util::*;
use std::{borrow::Borrow, collections::HashMap, io::Cursor, iter::Peekable};

#[derive(Debug)]
pub struct PsdSprite {
    pub(crate) entries: Vec<Entry>,
    pub rect: Rect<Px>,
}
impl PsdSprite {
    pub fn from_psd_bytes(psd_bytes: &[u8]) -> anyhow::Result<Self> {
        let psd = psd::Psd::from_bytes(psd_bytes)?;
        let rect = Wh::new(psd.psd_width(), psd.psd_height())
            .map(|x| (x as f32).px())
            .to_rect();
        let layer_trees = make_tree(&psd)?;
        Ok(layer_tree::into_psd_sprite(layer_trees, rect)?)
    }

    pub fn render(&self, scene_sprite: &SceneSprite) -> anyhow::Result<Option<ImageFilter>> {
        render_psd_sprite(scene_sprite, self)
    }

    pub fn to_parts_sprite(&self, name: String) -> anyhow::Result<schema_0::PartsSprite> {
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
                                    part_options: to_options(&entries),
                                },
                            )]
                        }
                        name if name.ends_with("_s") => {
                            vec![(
                                name,
                                schema_0::SpritePart {
                                    name: entry.name.clone(),
                                    is_single_select: true,
                                    part_options: to_options(&entries),
                                },
                            )]
                        }
                        _ => entries
                            .par_iter()
                            .flat_map(|entry| collect_parts(entry))
                            .collect(),
                    }
                }
            }
        }

        let parts = self
            .entries
            .par_iter()
            .flat_map(|entry| collect_parts(entry))
            .collect();
        Ok(schema_0::PartsSprite { name, parts })
    }
}

#[derive(Debug)]
pub(crate) struct Entry {
    pub(crate) name: String,
    pub(crate) blend_mode: BlendMode,
    pub(crate) clipping_base: bool,
    pub(crate) opacity: u8,
    pub(crate) mask: Option<SpriteImage>,
    pub(crate) kind: EntryKind,
}

#[derive(Debug)]
pub(crate) enum EntryKind {
    Layer { image: SpriteImage },
    Group { entries: Vec<Entry> },
}

#[derive(Debug)]
pub(crate) struct SpriteImage {
    pub(crate) dest_rect: Rect<Px>,
    pub(crate) webp: Box<[u8]>,
}
impl SpriteImage {
    pub fn to_namui_image(&self) -> anyhow::Result<Image> {
        let image = image::ImageReader::new(Cursor::new(&self.webp))
            .with_guessed_format()?
            .decode()?;
        let rgba = image.to_rgba8().into_vec();
        Ok(skia_safe::image::images::raster_from_data(
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
        .ok_or(anyhow::anyhow!("Failed to create image from SpriteImage"))?)
    }

    pub fn to_namui_image_a8(&self) -> anyhow::Result<Image> {
        let image = image::ImageReader::new(Cursor::new(&self.webp))
            .with_guessed_format()?
            .decode()?;
        let rgba = image.to_luma8().into_vec();
        Ok(skia_safe::image::images::raster_from_data(
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
        ))?)
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
                .flat_map(|entry| load_parts_sprite_images_from_entry(entry))
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
                    .flat_map(|entry| load_parts_sprite_mask_images_from_entry(entry))
                    .collect();
                masks.extend(child_masks);
                masks
            }
        }
    }
}

fn render_psd_sprite(
    scene_sprite: &schema_0::SceneSprite,
    psd_sprite: &PsdSprite,
) -> anyhow::Result<Option<ImageFilter>> {
    let parts_sprite_images = HashMap::from_iter(load_parts_sprite_images(psd_sprite));
    let parts_sprite_mask_images = HashMap::from_iter(load_parts_sprite_mask_images(psd_sprite));
    let rendered = render_entries(
        None,
        &psd_sprite.entries,
        scene_sprite,
        &parts_sprite_images,
        &parts_sprite_mask_images,
        &None,
        255,
    )?;

    Ok(rendered)
}

fn render_entries<T: Borrow<Entry>>(
    mut background: Option<ImageFilter>,
    entries: &[T],
    scene_sprite: &SceneSprite,
    parts_sprite_images: &HashMap<String, ImageFilter>,
    parts_sprite_mask_images: &HashMap<String, ImageFilter>,
    parent_mask: &Option<ImageFilter>,
    parent_opacity: u8,
) -> anyhow::Result<Option<ImageFilter>> {
    let mut entries = entries.into_iter().rev().peekable();

    while let Some(entry) = entries.next() {
        let entry = <T as Borrow<Entry>>::borrow(&entry);
        let blend_mode = entry.blend_mode;
        let passthrough = matches!(blend_mode, BlendMode::PassThrough);
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
                                .into_iter()
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
                    )?
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
                        )?;
                        continue;
                    }

                    render_entries(
                        Some(ImageFilter::Empty),
                        &entries,
                        scene_sprite,
                        parts_sprite_images,
                        parts_sprite_mask_images,
                        &None,
                        255,
                    )?
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
                )?;
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

    Ok(background)
}

fn has_clipping_layer<T: Borrow<Entry>>(
    entries: &mut Peekable<std::iter::Rev<std::slice::Iter<T>>>,
) -> bool {
    entries
        .peek()
        .is_some_and(|parts_sprite| <T as Borrow<Entry>>::borrow(parts_sprite).clipping_base)
}
