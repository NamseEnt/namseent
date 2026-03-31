use super::*;
use crate::*;
use bytes::*;
use std::{
    fmt::Debug,
    hash::Hash,
    sync::{Arc, LazyLock},
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
    pub const fn id(&self) -> usize {
        self.id
    }
    #[allow(dead_code)]
    pub fn get_default_shader(&self) -> Shader {
        Shader::Image {
            src: *self,
            tile_mode: Xy::single(TileMode::Clamp),
        }
    }

    pub fn info(&self) -> ImageInfo {
        if let Some(image) = IMAGES.get(&self.id) {
            let skia_info = image.image_info();
            ImageInfo {
                alpha_type: skia_info.alpha_type().into(),
                color_type: skia_info.color_type().into(),
                width: px(skia_info.width() as f32),
                height: px(skia_info.height() as f32),
            }
        } else {
            IMAGE_INFOS
                .get(&self.id)
                .map(|info| *info)
                .unwrap_or_else(|| panic!("Image {} not registered", self.id))
        }
    }

    pub fn skia_image(&self) -> Arc<skia_safe::Image> {
        IMAGES
            .get(&self.id)
            .unwrap_or_else(|| panic!("Image {} not registered", self.id))
            .clone()
    }
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

static IMAGES: LazyLock<dashmap::DashMap<usize, Arc<skia_safe::image::Image>>> =
    LazyLock::new(dashmap::DashMap::new);
/// Stores (buffer_ptr as usize, buffer_len) for each registered image.
/// Pointer stored as usize to satisfy Send+Sync for DashMap.
static IMAGE_BUFFER_PTR: LazyLock<dashmap::DashMap<usize, (usize, usize)>> =
    LazyLock::new(dashmap::DashMap::new);
static IMAGE_INFOS: LazyLock<dashmap::DashMap<usize, ImageInfo>> =
    LazyLock::new(dashmap::DashMap::new);

/// Register an image from raw encoded bytes. Uses a mangled Rust symbol,
/// so callers always resolve to their own statically-linked copy (no
/// dynamic symbol interposition from RTLD_GLOBAL).
///
/// # Safety
/// `buffer_ptr` must point to valid encoded image data of `buffer_len` bytes.
pub unsafe fn register_image(image_id: usize, buffer_ptr: *const u8, buffer_len: usize) {
    IMAGE_BUFFER_PTR.insert(image_id, (buffer_ptr as usize, buffer_len));

    let data =
        unsafe { skia_safe::Data::new_bytes(std::slice::from_raw_parts(buffer_ptr, buffer_len)) };
    let image = skia_safe::image::Image::from_encoded(data)
        .unwrap_or_else(|| panic!("Failed to decode image {image_id}"));
    IMAGES.insert(image_id, Arc::new(image));
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn _register_image(
    image_id: usize,
    buffer_ptr: *const u8,
    buffer_len: usize,
) {
    unsafe { register_image(image_id, buffer_ptr, buffer_len) };
}

/// Returns `[id, ptr_as_usize, len]` for each registered image buffer.
pub fn image_buffer_list() -> Vec<[usize; 3]> {
    IMAGE_BUFFER_PTR
        .iter()
        .map(|e| {
            let (&id, &(ptr, len)) = (e.key(), e.value());
            [id, ptr, len]
        })
        .collect()
}

/// Bulk-register image info from a packed buffer.
/// Buffer layout per image (14 bytes): id(u32 LE), alpha_type(u8), color_type(u8), width(u32 LE), height(u32 LE)
#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn _set_image_infos(ptr: *const u8, count: usize) {
    let image_info_size = 14;
    let mut reader: &[u8] =
        unsafe { std::slice::from_raw_parts(ptr, count * image_info_size) };
    for _ in 0..count {
        let id = reader.get_u32_le() as usize;
        let alpha_type = AlphaType::from(reader.get_u8());
        let color_type = ColorType::from(reader.get_u8());
        let width = px(reader.get_u32_le() as f32);
        let height = px(reader.get_u32_le() as f32);
        IMAGE_INFOS.insert(
            id,
            ImageInfo {
                alpha_type,
                color_type,
                width,
                height,
            },
        );
    }
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
pub unsafe extern "C" fn _image_infos(ptr: *mut u8, max_count: usize) -> usize {
    let images = &*IMAGES;
    let count = images.len().min(max_count);

    let image_info_size = 14;
    let mut bytes = unsafe { std::slice::from_raw_parts_mut(ptr, count * image_info_size) };
    let mut written = 0;
    for image in images.iter() {
        if written >= count {
            break;
        }
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
        written += 1;
    }
    written
}
