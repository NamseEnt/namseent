use crate::*;
use namui_type::*;
use psd::BlendMode;
use sk_position_image::SkPositionImage;

#[derive(Debug)]
pub struct PartsSpriteAsset {
    entries: Vec<Entry>,
    pub rect: Rect<Px>,
}
impl PartsSpriteAsset {
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
struct Entry {
    name: String,
    blend_mode: BlendMode,
    clipping_base: bool,
    opacity: u8,
    rect: Rect<Px>,
    mask: Option<SpriteImage>,
    kind: EntryKind,
}

#[derive(Debug)]
enum EntryKind {
    Layer { image: SpriteImage },
    Group { entries: Vec<Entry> },
}

#[derive(Debug)]
pub(crate) struct SpriteImage {
    dest_rect: Rect<Px>,
    webp: Box<[u8]>,
}
impl SpriteImage {
    pub fn to_sk_image(&self) -> Result<Image> {
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

    pub fn to_sk_image_a8(&self) -> Result<Image> {
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

    pub fn from_sk_position_image(sk_position_image: SkPositionImage) -> Result<Self> {
        let webp = sk_image_to_webp(&sk_position_image.sk_image)?;
        Ok(SpriteImage {
            dest_rect: sk_position_image.dest_rect.map(|a| a.px()),
            webp: webp.into_boxed_slice(),
        })
    }
}

fn render_parts_sprite(
    sprite_part: &PartsSpriteAsset,
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

fn load_parts_sprite_images(sprite_part: &PartsSpriteAsset) -> Vec<(String, SkPositionImage)> {
    match &sprite_part.kind {
        Entry::SingleSelect { options: entries }
        | Entry::MultiSelect { options: entries }
        | Entry::Group { entries } => entries
            .par_iter()
            .flat_map(|entry| load_parts_sprite_images(entry))
            .collect(),
        Entry::Layer { image } => image
            .to_sk_image()
            .map(|sk_image| SkPositionImage {
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

fn load_parts_sprite_mask_images(sprite_part: &PartsSpriteAsset) -> Vec<(String, SkPositionImage)> {
    match &sprite_part.kind {
        Entry::SingleSelect { options: entries }
        | Entry::MultiSelect { options: entries }
        | Entry::Group { entries } => entries
            .par_iter()
            .flat_map(|entry| load_parts_sprite_mask_images(entry))
            .collect(),
        Entry::Layer { .. } => sprite_part
            .mask
            .as_ref()
            .and_then(|mask| {
                mask.to_sk_image_a8().ok().map(|sk_image| SkPositionImage {
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

fn render_parts_sprite_to_canvas<T: Borrow<PartsSpriteAsset>>(
    canvas: &skia_safe::Canvas,
    parts_sprites: &[T],
    parts_sprite_lock: &PartsSpriteLock,
    parts_sprite_images: &HashMap<String, SkPositionImage>,
    parts_sprite_mask_images: &HashMap<String, SkPositionImage>,
    parent_mask: &Option<SkPositionImage>,
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
            Entry::SingleSelect { options } => {
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
            Entry::MultiSelect { options } => {
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
            Entry::Group { entries } => render_parts_sprite_to_canvas(
                canvas,
                entries.as_slice(),
                parts_sprite_lock,
                parts_sprite_images,
                parts_sprite_mask_images,
                &mask,
            )?,
            Entry::Layer { .. } => {
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
                if <T as Borrow<PartsSpriteAsset>>::borrow(parts_sprite).clipping_base {
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

fn create_paint_from_parts_sprite(parts_sprite: &PartsSpriteAsset) -> skia_safe::Paint {
    let mut paint = Paint::default();

    set_photoshop_blend_mode(&mut paint, parts_sprite.blend_mode);
    paint.set_alpha(parts_sprite.opacity);

    paint
}

fn has_clipping_layer<T: Borrow<PartsSpriteAsset>>(
    parts_sprites: &mut Peekable<std::iter::Rev<std::slice::Iter<T>>>,
) -> bool {
    parts_sprites.peek().is_some_and(|parts_sprite| {
        !<T as Borrow<PartsSpriteAsset>>::borrow(parts_sprite).clipping_base
    })
}
