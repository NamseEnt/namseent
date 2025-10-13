use namui_skia::skia_safe;
use std::sync::{
    OnceLock,
    atomic::{AtomicUsize, Ordering},
};

static IMAGES: OnceLock<dashmap::DashMap<usize, skia_safe::image::Image>> = OnceLock::new();

static VALUE: AtomicUsize = AtomicUsize::new(0);

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn _test() {
    println!("test {}", VALUE.fetch_add(1, Ordering::SeqCst));
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn _register_image(
    image_id: usize,
    image_bytes_ptr: *const u8,
    image_bytes_len: usize,
) {
    let bytes = unsafe { std::slice::from_raw_parts(image_bytes_ptr, image_bytes_len) };
    let data = skia_safe::Data::new_copy(bytes);
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
pub unsafe extern "C" fn _image_infos(ptr: *mut u8) {
    let images = IMAGES.get().unwrap();
    let count = images.len();

    let bytes = unsafe { std::slice::from_raw_parts_mut(ptr, count * 10) };
    for i in 0..count {
        let image = images.get(&i).unwrap();
        let info = image.image_info();

        let alpha_type: namui_skia::AlphaType = info.alpha_type().into();
        let alpha_type: u8 = alpha_type.into();
        bytes[i * 10] = alpha_type;

        let color_type: namui_skia::ColorType = info.color_type().into();
        let color_type: u8 = color_type.into();
        bytes[i * 10 + 1] = color_type;

        let width = info.width();
        let height = info.height();
        bytes[i * 10 + 2..i * 10 + 6].copy_from_slice(&width.to_le_bytes());
        bytes[i * 10 + 6..i * 10 + 10].copy_from_slice(&height.to_le_bytes());
    }
}
