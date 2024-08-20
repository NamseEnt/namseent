pub(crate) fn photoshop_blend_mode_into_blender(blend_mode: psd::BlendMode) -> namui::Blender {
    match blend_mode {
        // BlendMode::PassThrough => todo!(),
        // BlendMode::Dissolve => todo!(),
        psd::BlendMode::Normal => namui::BlendMode::SrcOver.into(),
        psd::BlendMode::Darken => namui::BlendMode::Darken.into(),
        psd::BlendMode::Multiply => namui::BlendMode::Multiply.into(),
        psd::BlendMode::ColorBurn => namui::BlendMode::ColorBurn.into(),
        psd::BlendMode::LinearBurn => namui::Blender::arithmetic(0.0, 1.0, 1.0, -1.0),
        psd::BlendMode::DarkerColor => {
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
        psd::BlendMode::Lighten => namui::BlendMode::Lighten.into(),
        psd::BlendMode::Screen => namui::BlendMode::Screen.into(),
        psd::BlendMode::ColorDodge => namui::BlendMode::ColorDodge.into(),
        psd::BlendMode::LinearDodge => namui::Blender::arithmetic(0.0, 1.0, 1.0, 0.0),
        psd::BlendMode::LighterColor => {
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
        psd::BlendMode::Overlay => namui::BlendMode::Overlay.into(),
        psd::BlendMode::SoftLight => namui::BlendMode::SoftLight.into(),
        psd::BlendMode::HardLight => namui::BlendMode::HardLight.into(),
        psd::BlendMode::VividLight => {
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
        psd::BlendMode::LinearLight => {
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
        psd::BlendMode::PinLight => {
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
        psd::BlendMode::HardMix => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(min(floor(src.rgb + dst.rgb), 1), src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        psd::BlendMode::Difference => namui::BlendMode::Difference.into(),
        psd::BlendMode::Exclusion => namui::BlendMode::Exclusion.into(),
        psd::BlendMode::Subtract => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(dst.rgb - src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        psd::BlendMode::Divide => {
            let sksl = r#"
                vec4 main(vec4 src, vec4 dst) {
                    vec4 new_src;

                    new_src = vec4(dst.rgb / src.rgb, src.a);

                    return new_src + (1 - new_src.a) * dst;
                }
            "#;
            namui::Blender::Sksl(sksl.to_string())
        }
        psd::BlendMode::Hue => namui::BlendMode::Hue.into(),
        psd::BlendMode::Saturation => namui::BlendMode::Saturation.into(),
        psd::BlendMode::Color => namui::BlendMode::Color.into(),
        psd::BlendMode::Luminosity => namui::BlendMode::Luminosity.into(),
        _ => namui::BlendMode::SrcOver.into(),
    }
}
