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
use skia_util::{set_photoshop_blend_mode, AutoRestoreCanvas};
use std::{
    borrow::Borrow,
    collections::{BTreeMap, BTreeSet},
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

    let image_info = ImageInfo::new_n32(
        (rect.width(), rect.height()),
        skia_safe::AlphaType::Unpremul,
        None,
    );
    let mut surface: Surface = skia_safe::surfaces::raster(&image_info, None, None).unwrap();
    let canvas = surface.canvas();
    canvas.translate((-rect.left(), -rect.top()));
    render_parts_sprite_to_canvas(canvas, std::slice::from_ref(sprite_part), parts_sprite_lock)?;

    let image = surface.image_snapshot();
    Ok(image)
}

fn render_parts_sprite_to_canvas<T: Borrow<PartsSprite>>(
    canvas: &skia_safe::Canvas,
    parts_sprites: &[T],
    parts_sprite_lock: &PartsSpriteLock,
) -> Result<()> {
    let _auto_restore = AutoRestoreCanvas::new(canvas);
    let mut parts_sprites = parts_sprites.into_iter().rev().peekable();

    while let Some(parts_sprite) = parts_sprites.next() {
        let _auto_restore = AutoRestoreCanvas::new(canvas);
        let parts_sprite = parts_sprite.borrow();
        let blend_mode = parts_sprite.blend_mode;
        {
            let paint = create_paint_from_parts_sprite(parts_sprite);
            let save_layer_rec = SaveLayerRec::default().paint(&paint);
            canvas.save_layer(&save_layer_rec);
        }

        if matches!(blend_mode, BlendMode::PassThrough) {
            canvas.restore();
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
                )?
            }
            SpritePartKind::Directory { entries } => {
                render_parts_sprite_to_canvas(canvas, entries.as_slice(), parts_sprite_lock)?
            }
            SpritePartKind::Fixed { image } => {
                let std::result::Result::Ok(sk_image) = image.to_sk_image() else {
                    // Maybe layer is empty
                    continue;
                };
                let paint = Paint::default();
                let left_top = (image.dest_rect.x().as_f32(), image.dest_rect.y().as_f32());
                canvas.draw_image(sk_image, left_top, Some(&paint));
            }
        }

        if has_clipping_layer(&mut parts_sprites) {
            let _auto_restore = AutoRestoreCanvas::new(canvas);
            {
                let mut paint = Paint::default();
                paint.set_blend_mode(skia_safe::BlendMode::SrcATop);
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
