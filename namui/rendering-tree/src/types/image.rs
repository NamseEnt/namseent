use super::*;
use crate::*;
use bytes::*;
use std::{
    collections::BTreeMap,
    fmt::Debug,
    hash::Hash,
    sync::{Arc, OnceLock},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, State)]
pub struct Image {
    id: usize,
}

impl Image {
    pub const STANDARD_CURSOR_SPRITE_SET: Image = Image { id: 100000 };
    pub const fn new(id: usize) -> Self {
        Self { id }
    }
    #[allow(dead_code)]
    pub fn get_default_shader(&self) -> Shader {
        Shader::Image {
            src: *self,
            tile_mode: Xy::single(TileMode::Clamp),
        }
    }
    pub fn info(&self) -> ImageInfo {
        IMAGE_INFOS.with(|image_infos| {
            image_infos
                .get_or_init(|| {
                    let image_count = unsafe { _get_image_count() };
                    let mut image_infos = BTreeMap::new();
                    let image_info_size = 14;
                    let mut buffer = vec![0u8; image_count * image_info_size];
                    unsafe { _get_image_infos(buffer.as_mut_ptr()) };

                    let mut buffer_reader: &[u8] = buffer.as_ref();
                    for _ in 0..image_count {
                        let id = buffer_reader.get_u32_le() as usize;
                        let alpha_type = AlphaType::from(buffer_reader.get_u8());
                        let color_type = ColorType::from(buffer_reader.get_u8());
                        let width = px(buffer_reader.get_u32_le() as f32);
                        let height = px(buffer_reader.get_u32_le() as f32);
                        image_infos.insert(
                            id,
                            ImageInfo {
                                alpha_type,
                                color_type,
                                width,
                                height,
                            },
                        );
                    }
                    image_infos
                })
                .get(&self.id)
                .cloned()
                .unwrap_or_else(|| panic!("Image {} not found", self.id))
        })
    }

    pub fn skia_image(&self) -> Arc<skia_safe::Image> {
        IMAGES.get().unwrap().get(&self.id).unwrap().clone()
    }
}

thread_local! {
    static IMAGE_INFOS: OnceLock<BTreeMap<usize, ImageInfo>> = const { OnceLock::new() };
}

unsafe extern "C" {
    fn _get_image_count() -> usize;
    /**
     * image info layout
     * - id: u32
     * - alpha_type: u8
     * - color_type: u8
     * - width: u32
     * - height: u32
     */
    fn _get_image_infos(buffer: *mut u8);
}

#[derive(Debug, Clone, Copy, Hash, namui_type::State)]
pub struct ImageInfo {
    pub alpha_type: AlphaType,
    pub color_type: ColorType,
    pub height: Px,
    pub width: Px,
}

impl ImageInfo {
    pub fn wh(&self) -> Wh<Px> {
        Wh {
            width: self.width,
            height: self.height,
        }
    }
}

impl From<ImageInfo> for skia_safe::ImageInfo {
    fn from(val: ImageInfo) -> Self {
        skia_safe::ImageInfo::new(
            skia_safe::ISize {
                width: val.width.as_f32() as i32,
                height: val.height.as_f32() as i32,
            },
            val.color_type.into(),
            val.alpha_type.into(),
            None,
        )
    }
}

static IMAGES: OnceLock<dashmap::DashMap<usize, Arc<skia_safe::image::Image>>> = OnceLock::new();
static IMAGE_BUFFER_PTR: OnceLock<dashmap::DashMap<usize, usize>> = OnceLock::new();

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn _register_image(
    image_id: usize,
    buffer_ptr: *const u8,
    buffer_len: usize,
) {
    IMAGE_BUFFER_PTR
        .get_or_init(dashmap::DashMap::new)
        .insert(image_id, buffer_ptr as usize);

    let data =
        unsafe { skia_safe::Data::new_bytes(std::slice::from_raw_parts(buffer_ptr, buffer_len)) };
    let image = skia_safe::image::Image::from_encoded(data).unwrap();
    IMAGES
        .get_or_init(dashmap::DashMap::new)
        .insert(image_id, Arc::new(image));
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
/**
 * image info layout
 * - id: u32
 * - alpha_type: u8
 * - color_type: u8
 * - width: u32
 * - height: u32
 */
pub unsafe extern "C" fn _image_infos(ptr: *mut u8) {
    let images = IMAGES.get().unwrap();
    let count = images.len();

    let image_info_size = 14;
    let mut bytes = unsafe { std::slice::from_raw_parts_mut(ptr, count * image_info_size) };
    for image in images.iter() {
        let info = image.image_info();
        let id = image.key();
        bytes.put_u32_le(*id as u32);

        let alpha_type: AlphaType = info.alpha_type().into();
        let alpha_type: u8 = alpha_type.into();
        bytes.put_u8(alpha_type);

        let color_type: ColorType = info.color_type().into();
        let color_type: u8 = color_type.into();
        bytes.put_u8(color_type);

        let width = info.width();
        let height = info.height();
        bytes.put_u32_le(width as u32);
        bytes.put_u32_le(height as u32);
    }
}
