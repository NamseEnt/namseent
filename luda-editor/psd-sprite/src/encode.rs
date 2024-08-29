use anyhow::{bail, Result};
use skia_safe::Image;

pub(crate) fn encode_image(image: &Image) -> Result<nimg::Nimg> {
    let width = image.width() as usize;
    let height = image.height() as usize;

    let image_info = image.image_info();
    let dest_color_type = match image_info.color_type() {
        skia_safe::ColorType::Alpha8 => skia_safe::ColorType::Alpha8,
        skia_safe::ColorType::BGRA8888 => skia_safe::ColorType::RGBA8888,
        skia_safe::ColorType::RGBA8888 => skia_safe::ColorType::RGBA8888,
        color_type => bail!("Unsupported color type: {:?}", color_type),
    };
    let row_bytes = image.width() as usize * image_info.bytes_per_pixel();
    let mut pixels = vec![0; image.height() as usize * row_bytes];

    image.read_pixels(
        &image_info.with_color_type(dest_color_type),
        &mut pixels,
        row_bytes,
        (0, 0),
        skia_safe::image::CachingHint::Disallow,
    );

    Ok(match dest_color_type {
        skia_safe::ColorType::Alpha8 => nimg::encode_a8(width, height, &pixels)?,
        skia_safe::ColorType::RGBA8888 => nimg::encode_rgba8(width, height, &pixels)?,
        _ => unreachable!(),
    })
}
