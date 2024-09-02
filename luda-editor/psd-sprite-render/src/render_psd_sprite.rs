use crate::blender::photoshop_blend_mode_into_blender;
use namui::*;
use psd_sprite::*;
use schema_0::SceneSprite;
use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::{Hash, Hasher},
    iter::Peekable,
    ops::Deref,
    sync::{Arc, RwLock},
};

lazy_static! {
    static ref IMAGE_FILTER_STORAGE: ImageFilterStorage = ImageFilterStorage::new();
}

type ImageFilterCreationKey = u64;

pub struct RenderPsdSprite<'a> {
    pub psd_sprite: Arc<PsdSprite>,
    pub scene_sprite: &'a SceneSprite,
    pub loaded_images: Arc<HashMap<SpriteImageId, SpriteLoadedImage>>,
    pub screen_wh: Wh<Px>,
}
impl Component for RenderPsdSprite<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            psd_sprite,
            scene_sprite,
            loaded_images,
            screen_wh,
        } = self;
        let image_filter_recreation_key =
            ctx.track_eq(&get_image_filter_creation_key(scene_sprite));

        ctx.effect("create image filter", {
            let psd_sprite = psd_sprite.clone();
            move || {
                let image_filter_recreation_key = *image_filter_recreation_key;

                if IMAGE_FILTER_STORAGE
                    .try_get(&image_filter_recreation_key)
                    .is_some()
                {
                    return;
                }

                IMAGE_FILTER_STORAGE.set(
                    image_filter_recreation_key,
                    ImageFilterCreateState::Creating,
                );

                let scene_sprite = scene_sprite.clone();
                ctx.spawn(async move {
                    let result =
                        match create_image_filter(&scene_sprite, &psd_sprite, &loaded_images) {
                            Ok(image_filter) => match image_filter {
                                Some(image_filter) => {
                                    ImageFilterCreateState::Created { image_filter }
                                }
                                None => ImageFilterCreateState::Error {
                                    error: Arc::new(anyhow::anyhow!("image_filter is None")),
                                },
                            },
                            Err(error) => ImageFilterCreateState::Error {
                                error: Arc::new(error),
                            },
                        };
                    IMAGE_FILTER_STORAGE.set(image_filter_recreation_key, result);
                });
            }
        });

        ctx.compose(|ctx| {
            let Some(state) = IMAGE_FILTER_STORAGE.try_get(&image_filter_recreation_key) else {
                return;
            };
            let ImageFilterCreateState::Created { image_filter } = state.deref() else {
                return;
            };

            let SceneSprite { circumcircle, .. } = scene_sprite;
            let paint = Paint::default().set_image_filter(image_filter.clone());
            let ratio = screen_wh.length() * circumcircle.radius / psd_sprite.wh.length();
            let ctx = ctx
                .translate((
                    screen_wh.width * circumcircle.xy.x,
                    screen_wh.height * circumcircle.xy.y,
                ))
                .scale(Xy::single(ratio))
                .translate(psd_sprite.wh.as_xy() * -0.5);
            let path = Path::new();
            ctx.add(PathDrawCommand { path, paint });
        });
    }
}

fn create_image_filter(
    scene_sprite: &schema_0::SceneSprite,
    psd_sprite: &PsdSprite,
    sprite_loaded_images: &SpriteLoadedImages,
) -> Result<Option<ImageFilter>> {
    let parts_sprite_images =
        HashMap::from_iter(load_parts_sprite_images(psd_sprite, sprite_loaded_images)?);
    let parts_sprite_mask_images = HashMap::from_iter(load_parts_sprite_mask_images(
        psd_sprite,
        sprite_loaded_images,
    )?);
    Ok(render_entries(
        None,
        &psd_sprite.entries,
        scene_sprite,
        &parts_sprite_images,
        &parts_sprite_mask_images,
        &None,
        255,
    ))
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

fn load_parts_sprite_images(
    sprite_part: &PsdSprite,
    sprite_loaded_images: &SpriteLoadedImages,
) -> Result<Vec<(String, ImageFilter)>> {
    return load_entries(&sprite_part.entries, sprite_loaded_images);

    fn load_entries(
        entries: &[Entry],
        sprite_loaded_images: &SpriteLoadedImages,
    ) -> Result<Vec<(String, ImageFilter)>> {
        Ok(entries
            .iter()
            .map(|entry| load_entry(entry, sprite_loaded_images))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    fn load_entry(
        entry: &Entry,
        sprite_loaded_images: &SpriteLoadedImages,
    ) -> Result<Vec<(String, ImageFilter)>> {
        match &entry.kind {
            EntryKind::Layer {
                image: sprite_image,
            } => sprite_loaded_images
                .get(&sprite_image.id)
                .ok_or(anyhow::anyhow!("image not found: {:?}", sprite_image.id))
                .map(|loaded_image| {
                    let image_filter = ImageFilter::Image {
                        src: loaded_image_to_namui(loaded_image),
                    }
                    .offset(sprite_image.dest_rect.xy());
                    vec![(entry.name.clone(), image_filter)]
                }),
            EntryKind::Group { entries } => load_entries(entries, sprite_loaded_images),
        }
    }
}

fn load_parts_sprite_mask_images(
    sprite_part: &PsdSprite,
    sprite_loaded_images: &SpriteLoadedImages,
) -> Result<Vec<(String, ImageFilter)>> {
    return load_entries(&sprite_part.entries, sprite_loaded_images);

    fn load_entries(
        entries: &[Entry],
        sprite_loaded_images: &SpriteLoadedImages,
    ) -> Result<Vec<(String, ImageFilter)>> {
        Ok(entries
            .iter()
            .map(|entry| load_entry(entry, sprite_loaded_images))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    fn load_entry(
        entry: &Entry,
        sprite_loaded_images: &SpriteLoadedImages,
    ) -> Result<Vec<(String, ImageFilter)>> {
        let mut ret = vec![];
        if let Some(mask) = entry.mask.as_ref() {
            let loaded_image = sprite_loaded_images
                .get(&mask.id)
                .ok_or(anyhow::anyhow!("image not found: {:?}", mask.id))?;

            ret.push((
                entry.name.clone(),
                ImageFilter::Image {
                    src: loaded_image_to_namui(loaded_image),
                }
                .offset(mask.dest_rect.xy()),
            ))
        }
        if let EntryKind::Group { entries } = &entry.kind {
            ret.extend(load_entries(entries, sprite_loaded_images)?);
        }
        Ok(ret)
    }
}

fn loaded_image_to_namui(
    SpriteLoadedImage { header, skia_image }: &SpriteLoadedImage,
) -> namui::Image {
    namui::Image {
        info: ImageInfo {
            alpha_type: namui::AlphaType::Unpremul,
            color_type: match header.color_type {
                psd_sprite::ColorType::Rgba8888 => namui::ColorType::Rgba8888,
                psd_sprite::ColorType::A8 => namui::ColorType::Alpha8,
            },
            height: (header.height as f32).px(),
            width: (header.width as f32).px(),
        },
        skia_image: skia_image.clone(),
    }
}

#[derive(Clone)]
enum ImageFilterCreateState {
    Creating,
    Created {
        image_filter: ImageFilter,
    },
    #[allow(dead_code)]
    Error {
        error: Arc<anyhow::Error>,
    },
}
struct ImageFilterStorage {
    storage: RwLock<HashMap<ImageFilterCreationKey, Arc<ImageFilterCreateState>>>,
}
impl ImageFilterStorage {
    fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
    fn try_get(&self, key: &ImageFilterCreationKey) -> Option<Arc<ImageFilterCreateState>> {
        self.storage.read().unwrap().get(key).cloned()
    }
    fn set(&self, key: ImageFilterCreationKey, create_state: ImageFilterCreateState) {
        self.storage
            .write()
            .unwrap()
            .insert(key, Arc::new(create_state));
    }
}

fn get_image_filter_creation_key(scene_sprite: &SceneSprite) -> ImageFilterCreationKey {
    let SceneSprite {
        sprite_id,
        part_option_selections,
        ..
    } = scene_sprite;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    sprite_id.hash(&mut hasher);
    part_option_selections
        .iter()
        .for_each(|(part_name, selections)| {
            part_name.hash(&mut hasher);
            selections
                .iter()
                .for_each(|part_option_id| part_option_id.hash(&mut hasher))
        });
    hasher.finish()
}

#[cfg(test)]
mod test {
    use super::*;
    use schema_0::Circumcircle;
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_create_image_filter() {
        let psd_bytes = include_bytes!("test.psd");

        let now = std::time::Instant::now();
        let (encoded_psd_sprite, _parts_sprite) =
            psd_sprite::encode_psd_sprite(psd_bytes, "test.psd").unwrap();
        println!("psd_sprite::encode_psd_sprite: {:?}", now.elapsed());
        println!("encoded_psd_sprite.len(): {}", encoded_psd_sprite.len());

        let now = std::time::Instant::now();
        let (psd_sprite, loaded_images) =
            psd_sprite::decode_psd_sprite(futures_util::stream::iter(vec![Ok(
                bytes::Bytes::copy_from_slice(&encoded_psd_sprite),
            )]))
            .await
            .unwrap();
        println!("decode_psd_sprite: {:?}", now.elapsed());

        let now = std::time::Instant::now();
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
        let _image_filter =
            create_image_filter(&scene_sprite, &psd_sprite, &loaded_images).unwrap();
        println!("create_image_filter: {:?}", now.elapsed());
    }
}
