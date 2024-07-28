use anyhow::{Ok, Result};
use image::ImageBuffer;
use psd::BlendMode;
use skia_safe::{Blender, Data, Image, Paint, RuntimeEffect};
use std::io::Cursor;

pub fn sk_image_to_webp(image: &Image) -> Result<Vec<u8>> {
    let row_bytes = image.width() as usize * 4;
    let mut pixels = vec![0; image.height() as usize * row_bytes];
    image.read_pixels(
        &image.image_info(),
        &mut pixels,
        row_bytes,
        (0, 0),
        skia_safe::image::CachingHint::Disallow,
    );
    let image_buffer = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_vec(
        image.width() as _,
        image.height() as _,
        pixels,
    )
    .ok_or(anyhow::anyhow!("Failed to create image buffer from layer"))?;
    let mut webp_bytes: Vec<u8> = Vec::new();
    image_buffer.write_to(&mut Cursor::new(&mut webp_bytes), image::ImageFormat::WebP)?;
    Ok(webp_bytes)
}

pub struct AutoRestoreCanvas<'canvas> {
    canvas: &'canvas skia_safe::Canvas,
    save_count: usize,
}
impl<'canvas> AutoRestoreCanvas<'canvas> {
    pub fn new(canvas: &'canvas skia_safe::Canvas) -> Self {
        let save_count = canvas.save_count();
        Self { canvas, save_count }
    }
}
impl Drop for AutoRestoreCanvas<'_> {
    fn drop(&mut self) {
        self.canvas.restore_to_count(self.save_count);
    }
}

pub fn set_photoshop_blend_mode(paint: &mut Paint, blend_mode: BlendMode) {
    match blend_mode {
        // BlendMode::PassThrough => todo!(),
        BlendMode::Normal => paint.set_blend_mode(skia_safe::BlendMode::SrcOver),
        // BlendMode::Dissolve => todo!(),
        BlendMode::Darken => paint.set_blend_mode(skia_safe::BlendMode::Darken),
        BlendMode::Multiply => paint.set_blend_mode(skia_safe::BlendMode::Multiply),
        BlendMode::ColorBurn => paint.set_blend_mode(skia_safe::BlendMode::ColorBurn),
        BlendMode::LinearBurn => {
            let blender = Blender::arithmetic(0.0, 1.0, 1.0, -1.0, false);
            paint.set_blender(blender)
        }
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
            paint.set_blender(blender)
        }
        BlendMode::Lighten => paint.set_blend_mode(skia_safe::BlendMode::Lighten),
        BlendMode::Screen => paint.set_blend_mode(skia_safe::BlendMode::Screen),
        BlendMode::ColorDodge => paint.set_blend_mode(skia_safe::BlendMode::ColorDodge),
        BlendMode::LinearDodge => {
            let blender = Blender::arithmetic(0.0, 1.0, 1.0, 0.0, false);
            paint.set_blender(blender)
        }
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
            paint.set_blender(blender)
        }
        BlendMode::Overlay => paint.set_blend_mode(skia_safe::BlendMode::Overlay),
        BlendMode::SoftLight => paint.set_blend_mode(skia_safe::BlendMode::SoftLight),
        BlendMode::HardLight => paint.set_blend_mode(skia_safe::BlendMode::HardLight),
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
            paint.set_blender(blender)
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
            paint.set_blender(blender)
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
            paint.set_blender(blender)
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
            paint.set_blender(blender)
        }
        BlendMode::Difference => paint.set_blend_mode(skia_safe::BlendMode::Difference),
        BlendMode::Exclusion => paint.set_blend_mode(skia_safe::BlendMode::Exclusion),
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
            paint.set_blender(blender)
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
            paint.set_blender(blender)
        }
        BlendMode::Hue => paint.set_blend_mode(skia_safe::BlendMode::Hue),
        BlendMode::Saturation => paint.set_blend_mode(skia_safe::BlendMode::Saturation),
        BlendMode::Color => paint.set_blend_mode(skia_safe::BlendMode::Color),
        BlendMode::Luminosity => paint.set_blend_mode(skia_safe::BlendMode::Luminosity),
        // TODO: implement other blend modes
        _ => &mut paint.set_blend_mode(skia_safe::BlendMode::SrcOver),
    };
}
