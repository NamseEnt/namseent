use namui_skia::skia_safe;
use std::sync::OnceLock;

static IMAGES: OnceLock<dashmap::DashMap<usize, skia_safe::image::Image>> = OnceLock::new();
static IMAGE_BUFFER: OnceLock<dashmap::DashMap<usize, Vec<u8>>> = OnceLock::new();

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn _malloc_image_buffer(
    image_id: usize,
    image_bytes_len: usize,
) -> *const u8 {
    IMAGE_BUFFER
        .get_or_init(dashmap::DashMap::new)
        .insert(image_id, vec![0u8; image_bytes_len]);

    IMAGE_BUFFER.get().unwrap().get(&image_id).unwrap().as_ptr()
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn _register_image(image_id: usize) {
    let buffer = IMAGE_BUFFER.get().unwrap().get(&image_id).unwrap();
    let data = unsafe { skia_safe::Data::new_bytes(&buffer) };
    let image = skia_safe::image::Image::from_encoded(data).unwrap();
    IMAGES
        .get_or_init(dashmap::DashMap::new)
        .insert(image_id, image);
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn _image_count() -> usize {
    IMAGES.get().unwrap().len()
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
    let bytes = unsafe { std::slice::from_raw_parts_mut(ptr, count * image_info_size) };
    for (i, image) in images.iter().enumerate() {
        let info = image.image_info();

        let id = image.key();
        bytes[i * image_info_size..i * image_info_size + 4].copy_from_slice(&id.to_le_bytes());

        let alpha_type: namui_skia::AlphaType = info.alpha_type().into();
        let alpha_type: u8 = alpha_type.into();
        bytes[i * image_info_size + 4] = alpha_type;

        let color_type: namui_skia::ColorType = info.color_type().into();
        let color_type: u8 = color_type.into();
        bytes[i * image_info_size + 5] = color_type;

        let width = info.width();
        let height = info.height();
        bytes[i * image_info_size + 6..i * image_info_size + 10]
            .copy_from_slice(&width.to_le_bytes());
        bytes[i * image_info_size + 10..i * image_info_size + 14]
            .copy_from_slice(&height.to_le_bytes());
    }
}
