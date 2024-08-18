use anyhow::{Ok, Result};
use image::ImageBuffer;
use psd::BlendMode;
use skia_safe::Image;
use std::io::Cursor;

pub fn sk_image_to_webp(image: &Image) -> Result<Vec<u8>> {
    let image_info = image.image_info();
    let color_type = image_info.color_type();
    let row_bytes = image.width() as usize * image_info.bytes_per_pixel();
    let mut pixels = vec![0; image.height() as usize * row_bytes];
    image.read_pixels(
        &image_info,
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

pub fn photoshop_blend_mode_into_blender(blend_mode: psd::BlendMode) -> namui::Blender {
    match blend_mode {
        // BlendMode::PassThrough => todo!(),
        // BlendMode::Dissolve => todo!(),
        BlendMode::Normal => namui::BlendMode::SrcOver.into(),
        BlendMode::Darken => namui::BlendMode::Darken.into(),
        BlendMode::Multiply => namui::BlendMode::Multiply.into(),
        BlendMode::ColorBurn => namui::BlendMode::ColorBurn.into(),
        BlendMode::LinearBurn => namui::Blender::arithmetic(0.0, 1.0, 1.0, -1.0),
        BlendMode::DarkerColor => {
            let sksl = r#"
                vec4 BRIGHTNESS_MAP = vec4(0.299, 0.587, 0.114, 0.0);
                vec4 main(vec4 src, vec4 dst) {
                    float src_brightness, dst_brightness;
                    vec4 new_src;

                    src_brightness = dot(src, BRIGHTNESS_MAP);
                    dst_brightness = dot(dst, BRIGHTNESS_MAP);
                    new_src = vec4(src_brightness > dst_brightness ? dst.rgb : src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        BlendMode::Lighten => namui::BlendMode::Lighten.into(),
        BlendMode::Screen => namui::BlendMode::Screen.into(),
        BlendMode::ColorDodge => namui::BlendMode::ColorDodge.into(),
        BlendMode::LinearDodge => namui::Blender::arithmetic(0.0, 1.0, 1.0, 0.0),
        BlendMode::LighterColor => {
            let sksl = r#"
                vec4 BRIGHTNESS_MAP = vec4(0.299, 0.587, 0.114, 0.0);
                vec4 main(vec4 src, vec4 dst) {
                    float src_brightness, dst_brightness;
                    vec4 new_src;

                    src_brightness = dot(src, BRIGHTNESS_MAP);
                    dst_brightness = dot(dst, BRIGHTNESS_MAP);
                    new_src = vec4(src_brightness > dst_brightness ? src.rgb : dst.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        BlendMode::Overlay => namui::BlendMode::Overlay.into(),
        BlendMode::SoftLight => namui::BlendMode::SoftLight.into(),
        BlendMode::HardLight => namui::BlendMode::HardLight.into(),
        BlendMode::VividLight => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    for (int i = 0; i < 3; i++) {
                        if (src[i] <= 0.5) {
                            new_src[i] = max(0, 1 - (1 - dst[i]) / (2 * src[i]));
                        } else {
                            new_src[i] = min(1, dst[i] / (2 * (1 - src[i])));
                        }
                    }
                    new_src.a = src.a;

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        BlendMode::LinearLight => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    for (int i = 0; i < 3; i++) {
                        if (src[i] <= 0.5) {
                            new_src[i] = dst[i] + 2 * src[i] - 1;
                        } else {
                            new_src[i] = dst[i] + 2 * (src[i] - 0.5);
                        }
                    }
                    new_src.a = src.a;

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        BlendMode::PinLight => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    for (int i = 0; i < 3; i++) {
                        if (src[i] > 0.5) {
                            new_src[i] = max(dst[i], 2 * (src[i] - 0.5));
                        } else {
                            new_src[i] = min(dst[i], 2 * src[i]);
                        }
                    }
                    new_src.a = src.a;

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        BlendMode::HardMix => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(min(floor(src.rgb + dst.rgb), 1), src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        BlendMode::Difference => namui::BlendMode::Difference.into(),
        BlendMode::Exclusion => namui::BlendMode::Exclusion.into(),
        BlendMode::Subtract => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(dst.rgb - src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        BlendMode::Divide => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(dst.rgb / src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        BlendMode::Hue => namui::BlendMode::Hue.into(),
        BlendMode::Saturation => namui::BlendMode::Saturation.into(),
        BlendMode::Color => namui::BlendMode::Color.into(),
        BlendMode::Luminosity => namui::BlendMode::Luminosity.into(),
        _ => namui::BlendMode::SrcOver.into(),
    }
}
