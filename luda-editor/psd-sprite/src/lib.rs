mod layer_tree;
pub mod skia_util;

use anyhow::{Ok, Result};
use layer_tree::make_tree;
use namui_type::*;
use psd::BlendMode;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use skia_safe::{
    canvas::{SaveLayerFlags, SaveLayerRec},
    Data, Image, ImageInfo, Paint, Surface,
};
use skia_util::{set_photoshop_blend_mode, sk_image_to_webp, AutoRestoreCanvas};
use std::{
    borrow::Borrow,
    collections::{BTreeMap, BTreeSet, HashMap},
    io::Cursor,
    iter::Peekable,
};

#[derive(Debug)]
pub struct PartsSpriteManifest {
    pub name: String,
    pub parts: BTreeMap<String, SpritePartManifest>,
}

impl PartsSpriteManifest {
    pub fn from_parts_sprite_resource(parts_sprite_resource: &PartsSprite) -> Result<Self> {
        fn collect_parts(parts_sprite_resource: &PartsSprite) -> Vec<SpritePartManifest> {
            match &parts_sprite_resource.kind {
                SpritePartKind::Fixed { .. } => {
                    vec![]
                }
                SpritePartKind::SingleSelect { options } => {
                    let options = options
                        .par_iter()
                        .map(|option| SpritePartManifestOption {
                            name: option.name.clone(),
                        })
                        .collect();
                    vec![SpritePartManifest {
                        name: parts_sprite_resource.name.to_string(),
                        kind: SpritePartManifestKind::SingleSelect { options },
                    }]
                }
                SpritePartKind::MultiSelect { options } => {
                    let options = options
                        .par_iter()
                        .map(|option| SpritePartManifestOption {
                            name: option.name.clone(),
                        })
                        .collect();
                    vec![SpritePartManifest {
                        name: parts_sprite_resource.name.to_string(),
                        kind: SpritePartManifestKind::MultiSelect { options },
                    }]
                }
                SpritePartKind::Directory { entries } => entries
                    .par_iter()
                    .flat_map(|entry| collect_parts(entry))
                    .collect(),
            }
        }
        let parts = collect_parts(parts_sprite_resource)
            .into_iter()
            .map(|part| (part.name.clone(), part));
        Ok(Self {
            name: parts_sprite_resource.name.to_string(),
            parts: BTreeMap::from_iter(parts),
        })
    }
}

#[derive(Debug)]
pub struct SpritePartManifest {
    pub name: String,
    pub kind: SpritePartManifestKind,
}

#[derive(Debug)]
pub enum SpritePartManifestKind {
    Fixed,
    SingleSelect {
        options: Vec<SpritePartManifestOption>,
    },
    MultiSelect {
        options: Vec<SpritePartManifestOption>,
    },
}

#[derive(Debug)]
pub struct SpritePartManifestOption {
    pub name: String,
}

pub struct PartsSpriteLock {
    pub name: String,
    pub part_names: BTreeSet<String>,
}

#[derive(Debug)]
pub struct PartsSprite {
    pub name: String,
    pub kind: SpritePartKind,
    pub blend_mode: BlendMode,
    pub clipping_base: bool,
    pub opacity: u8,
    pub rect: Rect<Px>,
    pub mask: Option<SpriteImage>,
}
impl PartsSprite {
    pub fn from_psd_bytes(psd_bytes: &[u8], name: String) -> Result<Self> {
        let psd = psd::Psd::from_bytes(psd_bytes)?;
        let layer_trees = make_tree(&psd)?;
        Ok(layer_tree::into_parts_sprite(layer_trees, name)?)
    }

    pub fn render(&self, lock: &PartsSpriteLock) -> Result<Image> {
        render_parts_sprite(self, lock)
    }
}

#[derive(Debug)]
pub enum SpritePartKind {
    SingleSelect { options: Vec<PartsSprite> },
    MultiSelect { options: Vec<PartsSprite> },
    Fixed { image: SpriteImage },
    Directory { entries: Vec<PartsSprite> },
}

#[derive(Debug)]
pub struct SpriteImage {
    pub dest_rect: Rect<Px>,
    pub webp: Box<[u8]>,
}
impl SpriteImage {
    fn to_sk_image(&self) -> Result<Image> {
        let image = image::ImageReader::new(Cursor::new(&self.webp))
            .with_guessed_format()?
            .decode()?;
        let rgba = image.to_rgba8().into_vec();
        Ok(skia_safe::image::images::raster_from_data(
            &ImageInfo::new_n32(
                (image.width() as i32, image.height() as i32),
                skia_safe::AlphaType::Unpremul,
                None,
            ),
            Data::new_copy(&rgba),
            image.width() as usize * 4,
        )
        .ok_or(anyhow::anyhow!("Failed to create image from SpriteImage"))?)
    }

    fn to_sk_image_a8(&self) -> Result<Image> {
        let image = image::ImageReader::new(Cursor::new(&self.webp))
            .with_guessed_format()?
            .decode()?;
        let rgba = image.to_luma8().into_vec();
        Ok(skia_safe::image::images::raster_from_data(
            &ImageInfo::new_a8((image.width() as i32, image.height() as i32)),
            Data::new_copy(&rgba),
            image.width() as usize,
        )
        .ok_or(anyhow::anyhow!(
            "Failed to create a8 image from SpriteImage"
        ))?)
    }
}

fn render_parts_sprite(
    sprite_part: &PartsSprite,
    parts_sprite_lock: &PartsSpriteLock,
) -> Result<Image> {
    let rect: Rect<i32> = Rect::Xywh {
        x: sprite_part.rect.x().as_f32() as _,
        y: sprite_part.rect.y().as_f32() as _,
        width: sprite_part.rect.width().as_f32() as _,
        height: sprite_part.rect.height().as_f32() as _,
    };
    if rect.width() == 0 || rect.height() == 0 {
        return Err(anyhow::anyhow!("No layer to rasterize"));
    }

    let parts_sprite_images = HashMap::from_iter(load_parts_sprite_images(sprite_part));
    let parts_sprite_mask_images = HashMap::from_iter(load_parts_sprite_mask_images(sprite_part));

    let image_info = ImageInfo::new_n32(
        (rect.width(), rect.height()),
        skia_safe::AlphaType::Unpremul,
        None,
    );
    let mut surface: Surface = skia_safe::surfaces::raster(&image_info, None, None).unwrap();
    let canvas = surface.canvas();
    canvas.translate((-rect.left(), -rect.top()));

    render_parts_sprite_to_canvas(
        canvas,
        std::slice::from_ref(sprite_part),
        parts_sprite_lock,
        &parts_sprite_images,
        &parts_sprite_mask_images,
        &None,
    )?;

    let image = surface.image_snapshot();
    Ok(image)
}

fn load_parts_sprite_images(sprite_part: &PartsSprite) -> Vec<(String, RenderingImage)> {
    match &sprite_part.kind {
        SpritePartKind::SingleSelect { options: entries }
        | SpritePartKind::MultiSelect { options: entries }
        | SpritePartKind::Directory { entries } => entries
            .par_iter()
            .flat_map(|entry| load_parts_sprite_images(entry))
            .collect(),
        SpritePartKind::Fixed { image } => image
            .to_sk_image()
            .map(|sk_image| RenderingImage {
                dest_rect: Rect::Ltrb {
                    left: image.dest_rect.left().as_f32() as i32,
                    top: image.dest_rect.top().as_f32() as i32,
                    right: image.dest_rect.right().as_f32() as i32,
                    bottom: image.dest_rect.bottom().as_f32() as i32,
                },
                sk_image,
            })
            .map_or(Vec::new(), |image| vec![(sprite_part.name.clone(), image)]),
    }
}

fn load_parts_sprite_mask_images(sprite_part: &PartsSprite) -> Vec<(String, RenderingImage)> {
    match &sprite_part.kind {
        SpritePartKind::SingleSelect { options: entries }
        | SpritePartKind::MultiSelect { options: entries }
        | SpritePartKind::Directory { entries } => entries
            .par_iter()
            .flat_map(|entry| load_parts_sprite_mask_images(entry))
            .collect(),
        SpritePartKind::Fixed { .. } => sprite_part
            .mask
            .as_ref()
            .and_then(|mask| {
                mask.to_sk_image_a8().ok().map(|sk_image| RenderingImage {
                    dest_rect: Rect::Ltrb {
                        left: mask.dest_rect.left().as_f32() as i32,
                        top: mask.dest_rect.top().as_f32() as i32,
                        right: mask.dest_rect.right().as_f32() as i32,
                        bottom: mask.dest_rect.bottom().as_f32() as i32,
                    },
                    sk_image,
                })
            })
            .map_or(Vec::new(), |mask| vec![(sprite_part.name.clone(), mask)]),
    }
}

fn render_parts_sprite_to_canvas<T: Borrow<PartsSprite>>(
    canvas: &skia_safe::Canvas,
    parts_sprites: &[T],
    parts_sprite_lock: &PartsSpriteLock,
    parts_sprite_images: &HashMap<String, RenderingImage>,
    parts_sprite_mask_images: &HashMap<String, RenderingImage>,
    parent_mask: &Option<RenderingImage>,
) -> Result<()> {
    let _auto_restore = AutoRestoreCanvas::new(canvas);
    let mut parts_sprites = parts_sprites.into_iter().rev().peekable();

    while let Some(parts_sprite) = parts_sprites.next() {
        let _auto_restore = AutoRestoreCanvas::new(canvas);
        let parts_sprite = parts_sprite.borrow();
        let blend_mode = parts_sprite.blend_mode;
        let passthrough = matches!(blend_mode, BlendMode::PassThrough);
        let has_clipping_layer = has_clipping_layer(&mut parts_sprites);
        let mask = {
            let mask_image = parts_sprite_mask_images.get(&parts_sprite.name);
            match (parent_mask, mask_image) {
                (None, None) => None,
                (None, Some(mask)) | (Some(mask), None) => Some(mask.clone()),
                (Some(parent_mask), Some(mask)) => parent_mask.intersect_as_mask(mask),
            }
        };

        if !passthrough || !has_clipping_layer {
            let paint = create_paint_from_parts_sprite(parts_sprite);
            let save_layer_rec = SaveLayerRec::default().paint(&paint);
            canvas.save_layer(&save_layer_rec);
        }

        match &parts_sprite.kind {
            SpritePartKind::SingleSelect { options } => {
                let selected_parts_sprite = options
                    .into_iter()
                    .find(|option| parts_sprite_lock.part_names.contains(&option.name));
                let Some(selected_parts_sprite) = selected_parts_sprite else {
                    continue;
                };
                render_parts_sprite_to_canvas(
                    canvas,
                    std::slice::from_ref(selected_parts_sprite),
                    parts_sprite_lock,
                    parts_sprite_images,
                    parts_sprite_mask_images,
                    &mask,
                )?
            }
            SpritePartKind::MultiSelect { options } => {
                let selected_parts_sprite = options
                    .into_iter()
                    .filter(|option| parts_sprite_lock.part_names.contains(&option.name))
                    .collect::<Vec<_>>();
                render_parts_sprite_to_canvas(
                    canvas,
                    selected_parts_sprite.as_slice(),
                    parts_sprite_lock,
                    parts_sprite_images,
                    parts_sprite_mask_images,
                    &mask,
                )?
            }
            SpritePartKind::Directory { entries } => render_parts_sprite_to_canvas(
                canvas,
                entries.as_slice(),
                parts_sprite_lock,
                parts_sprite_images,
                parts_sprite_mask_images,
                &mask,
            )?,
            SpritePartKind::Fixed { .. } => {
                let Some(image) = parts_sprite_images.get(&parts_sprite.name) else {
                    // Maybe layer is empty
                    continue;
                };
                let paint = Paint::default();
                canvas.draw_image(&image.sk_image, image.left_top(), Some(&paint));

                if let Some(mask) = &mask {
                    let mut paint = Paint::default();
                    paint.set_blend_mode(skia_safe::BlendMode::DstIn);
                    canvas.draw_image(&mask.sk_image, mask.left_top(), Some(&paint));
                }
            }
        }

        if has_clipping_layer {
            let _auto_restore = AutoRestoreCanvas::new(canvas);
            {
                let mut paint = Paint::default();
                paint.set_blend_mode(skia_safe::BlendMode::SrcIn);
                let save_layer_rec = SaveLayerRec::default()
                    .flags(SaveLayerFlags::INIT_WITH_PREVIOUS)
                    .paint(&paint);
                canvas.save_layer(&save_layer_rec);
            }
            while let Some(parts_sprite) = parts_sprites.peek() {
                if <T as Borrow<PartsSprite>>::borrow(parts_sprite).clipping_base {
                    break;
                }
                let clipping_parts_sprite = parts_sprites.next().unwrap();
                render_parts_sprite_to_canvas(
                    canvas,
                    std::slice::from_ref(clipping_parts_sprite),
                    parts_sprite_lock,
                    parts_sprite_images,
                    parts_sprite_mask_images,
                    &mask,
                )?;
            }
        }
    }
    Ok(())
}

fn create_paint_from_parts_sprite(parts_sprite: &PartsSprite) -> skia_safe::Paint {
    let mut paint = Paint::default();

    set_photoshop_blend_mode(&mut paint, parts_sprite.blend_mode);
    paint.set_alpha(parts_sprite.opacity);

    paint
}

fn has_clipping_layer<T: Borrow<PartsSprite>>(
    parts_sprites: &mut Peekable<std::iter::Rev<std::slice::Iter<T>>>,
) -> bool {
    parts_sprites
        .peek()
        .is_some_and(|parts_sprite| !<T as Borrow<PartsSprite>>::borrow(parts_sprite).clipping_base)
}

#[derive(Debug, Clone)]
pub struct RenderingImage {
    pub dest_rect: Rect<i32>,
    pub sk_image: Image,
}
impl RenderingImage {
    pub fn intersect_as_mask(&self, other: &RenderingImage) -> Option<Self> {
        let merged_rect = self.dest_rect.intersect(other.dest_rect);
        let Some(merged_rect) = merged_rect else {
            return None;
        };

        let mut surface = skia_safe::surfaces::raster(
            &ImageInfo::new_a8((merged_rect.width(), merged_rect.height())),
            None,
            None,
        )?;
        let canvas = surface.canvas();
        canvas.translate((-merged_rect.left(), -merged_rect.top()));
        let mut paint = Paint::default();
        paint.set_blend_mode(skia_safe::BlendMode::SrcIn);
        canvas.draw_image(&self.sk_image, self.left_top(), Some(&paint));
        canvas.draw_image(&other.sk_image, other.left_top(), Some(&paint));
        let sk_image = surface.image_snapshot();

        Some(Self {
            dest_rect: merged_rect,
            sk_image,
        })
    }

    fn left_top(&self) -> (i32, i32) {
        (self.dest_rect.left(), self.dest_rect.top())
    }

    pub fn to_sprite_image(self) -> Result<SpriteImage> {
        let webp = sk_image_to_webp(&self.sk_image)?;
        Ok(SpriteImage {
            dest_rect: Rect::Ltrb {
                left: self.dest_rect.left().px(),
                top: self.dest_rect.top().px(),
                right: self.dest_rect.right().px(),
                bottom: self.dest_rect.bottom().px(),
            },
            webp: webp.into_boxed_slice(),
        })
    }
}
impl AsRef<RenderingImage> for &RenderingImage {
    fn as_ref(&self) -> &RenderingImage {
        self
    }
}
