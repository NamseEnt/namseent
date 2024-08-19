use anyhow::{Ok, Result};
use image::ImageBuffer;
use skia_safe::Image;
use std::io::Cursor;

pub fn sk_image_to_webp(image: &Image) -> Result<Vec<u8>> {
    let image_info = image.image_info();
    let color_type = image_info.color_type();
    let row_bytes = image.width() as usize * image_info.bytes_per_pixel();
    let mut pixels = vec![0; image.height() as usize * row_bytes];
    image.read_pixels(
        image_info,
        &mut pixels,
        row_bytes,
        (0, 0),
        skia_safe::image::CachingHint::Disallow,
    );
    let mut webp_bytes: Vec<u8> = Vec::new();
    match color_type {
        skia_safe::ColorType::Alpha8 => {
            let image_buffer = ImageBuffer::<image::Luma<u8>, Vec<u8>>::from_vec(
                image.width() as _,
                image.height() as _,
                pixels,
            )
            .ok_or(anyhow::anyhow!(
                "Failed to create image buffer from Alpha8 layer"
            ))?;
            image_buffer.write_to(&mut Cursor::new(&mut webp_bytes), image::ImageFormat::WebP)?;
        }
        skia_safe::ColorType::RGBA8888 => {
            let image_buffer = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_vec(
                image.width() as _,
                image.height() as _,
                pixels,
            )
            .ok_or(anyhow::anyhow!(
                "Failed to create image buffer from RGBA8888 layer"
            ))?;
            image_buffer.write_to(&mut Cursor::new(&mut webp_bytes), image::ImageFormat::WebP)?;
        }
        skia_safe::ColorType::BGRA8888 => {
            for i in 0..pixels.len() / 4 {
                let b = pixels[i * 4];
                let r = pixels[i * 4 + 2];
                pixels[i * 4] = r;
                pixels[i * 4 + 2] = b;
            }
            let image_buffer = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_vec(
                image.width() as _,
                image.height() as _,
                pixels,
            )
            .ok_or(anyhow::anyhow!(
                "Failed to create image buffer from BGRA8888 layer"
            ))?;
            image_buffer.write_to(&mut Cursor::new(&mut webp_bytes), image::ImageFormat::WebP)?;
        }
        _ => unimplemented!("Unsupported color type: {:?}", color_type),
    }
    Ok(webp_bytes)
}
