use anyhow::{Ok, Result};
use image::ImageBuffer;
use psd::BlendMode;
use skia_safe::{Blender, Data, Image, RuntimeEffect};
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

pub fn photoshop_blend_mode_into_blender(blend_mode: psd::BlendMode) -> skia_safe::Blender {
    match blend_mode {
        // BlendMode::PassThrough => todo!(),
        // BlendMode::Dissolve => todo!(),
        BlendMode::Normal => skia_safe::BlendMode::SrcOver.into(),
        BlendMode::Darken => skia_safe::BlendMode::Darken.into(),
        BlendMode::Multiply => skia_safe::BlendMode::Multiply.into(),
        BlendMode::ColorBurn => skia_safe::BlendMode::ColorBurn.into(),
        BlendMode::LinearBurn => Blender::arithmetic(0.0, 1.0, 1.0, -1.0, false).unwrap(),
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
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            blender.unwrap()
        }
        BlendMode::Lighten => skia_safe::BlendMode::Lighten.into(),
        BlendMode::Screen => skia_safe::BlendMode::Screen.into(),
        BlendMode::ColorDodge => skia_safe::BlendMode::ColorDodge.into(),
        BlendMode::LinearDodge => Blender::arithmetic(0.0, 1.0, 1.0, 0.0, false).unwrap(),
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
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            blender.unwrap()
        }
        BlendMode::Overlay => skia_safe::BlendMode::Overlay.into(),
        BlendMode::SoftLight => skia_safe::BlendMode::SoftLight.into(),
        BlendMode::HardLight => skia_safe::BlendMode::HardLight.into(),
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
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            blender.unwrap()
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
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            blender.unwrap()
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
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            blender.unwrap()
        }
        BlendMode::HardMix => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(min(floor(src.rgb + dst.rgb), 1), src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            blender.unwrap()
        }
        BlendMode::Difference => skia_safe::BlendMode::Difference.into(),
        BlendMode::Exclusion => skia_safe::BlendMode::Exclusion.into(),
        BlendMode::Subtract => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(dst.rgb - src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            blender.unwrap()
        }
        BlendMode::Divide => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(dst.rgb / src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            let effect = RuntimeEffect::make_for_blender(sksl, None).unwrap();
            let blender = effect.make_blender(Data::new_empty(), None);
            blender.unwrap()
        }
        BlendMode::Hue => skia_safe::BlendMode::Hue.into(),
        BlendMode::Saturation => skia_safe::BlendMode::Saturation.into(),
        BlendMode::Color => skia_safe::BlendMode::Color.into(),
        BlendMode::Luminosity => skia_safe::BlendMode::Luminosity.into(),
        _ => skia_safe::BlendMode::SrcOver.into(),
    }
}
