use crate::storage::get_project_cg_part_variant_image_url;
use namui::prelude::*;
use rpc::data::*;

pub struct CgRenderProps {
    pub rect: Rect<Px>,
    pub project_id: Uuid,
    pub cg_id: Uuid,
}

pub fn render_cg(props: CgRenderProps, screen_cg: &ScreenCg, cg_file: &CgFile) -> RenderingTree {
    render(screen_cg.parts.iter().rev().map(|screen_part| {
        try_render(|| {
            let cg_part = cg_file
                .parts
                .iter()
                .find(|part| part.name == screen_part.name())?;
            Some(render_cg_part(&props, screen_part, cg_part))
        })
    }))
}

fn render_cg_part(
    props: &CgRenderProps,
    part: &rpc::data::ScreenCgPart,
    cg_part: &CgPart,
) -> RenderingTree {
    match part {
        rpc::data::ScreenCgPart::Single { variant_name, .. } => try_render(|| {
            let variant_name = variant_name.as_ref()?;

            let variant = cg_part
                .variants
                .iter()
                .find(|variant| &variant.name == variant_name)?;

            Some(render_cg_variant(props, variant))
        }),
        rpc::data::ScreenCgPart::Multi { variant_names, .. } => render(
            cg_part
                .variants
                .iter()
                .filter(|variant| variant_names.contains(&variant.name))
                .map(|variant| render_cg_variant(props, variant)),
        ),
        rpc::data::ScreenCgPart::AlwaysOn { .. } => render(
            cg_part
                .variants
                .iter()
                .map(|variant| render_cg_variant(props, variant)),
        ),
    }
}

fn render_cg_variant(props: &CgRenderProps, variant: &rpc::data::CgPartVariant) -> RenderingTree {
    try_render(|| {
        let rect = Rect::Xywh {
            x: props.rect.x() + props.rect.width() * variant.rect.x(),
            y: props.rect.y() + props.rect.height() * variant.rect.y(),
            width: props.rect.width() * variant.rect.width(),
            height: props.rect.height() * variant.rect.height(),
        };

        let url = get_project_cg_part_variant_image_url(props.project_id, props.cg_id, variant.id)
            .unwrap();

        let image = namui::image::try_load_url(&url)?;
        let image_info = image.get_image_info();
        let src_wh = Wh::new(image_info.width, image_info.height);

        let paint_builder = get_straight_alpha_image_paint_builder(&image, variant.blend_mode);
        let path_builder = PathBuilder::new().add_rect(src_wh.to_rect());
        Some(translate(
            rect.x(),
            rect.y(),
            scale(
                rect.width() / src_wh.width,
                rect.height() / src_wh.height,
                path(path_builder, paint_builder),
            ),
        ))
    })
}

/// Skia uses premultiplied alpha, but cg part variant webp and photoshop use straight alpha.
/// So we need to convert premultiplied alpha to straight alpha.
fn get_straight_alpha_image_paint_builder(
    image: &Image,
    cg_part_variant_blend_mode: CgPartVariantBlendMode,
) -> PaintBuilder {
    let paint = PaintBuilder::new();
    namui::shader!(StraightAlphaShader, {
        uniform shader image;

        half4 main(float2 coord) {
            half4 evaluated = image.eval(coord);
            return evaluated.rgba / evaluated.aaa1;
        }
    });

    let image_shader = image.get_default_shader();
    let paint = paint.set_shader(StraightAlphaShader::new(image_shader).make());

    // TODO: blend modes of skia is implemented for premultiplied alpha.
    // We may need to implement blend modes for straight alpha with custom blender.
    // Custom blender is not supported yet on canvas-kit. https://groups.google.com/g/skia-discuss/c/6QdgoxoYnv8
    match cg_part_variant_blend_mode {
        CgPartVariantBlendMode::Normal => paint.set_blend_mode(BlendMode::SrcOver),
        CgPartVariantBlendMode::Darken => paint.set_blend_mode(BlendMode::Darken),
        CgPartVariantBlendMode::Multiply => paint.set_blend_mode(BlendMode::Multiply),
        CgPartVariantBlendMode::ColorBurn => paint.set_blend_mode(BlendMode::ColorBurn),
        CgPartVariantBlendMode::Lighten => paint.set_blend_mode(BlendMode::Lighten),
        CgPartVariantBlendMode::Screen => paint.set_blend_mode(BlendMode::Screen),
        CgPartVariantBlendMode::ColorDodge => paint.set_blend_mode(BlendMode::ColorDodge),
        CgPartVariantBlendMode::Overlay => paint.set_blend_mode(BlendMode::Overlay),
        CgPartVariantBlendMode::SoftLight => paint.set_blend_mode(BlendMode::SoftLight),
        CgPartVariantBlendMode::HardLight => paint.set_blend_mode(BlendMode::HardLight),
        CgPartVariantBlendMode::Difference => paint.set_blend_mode(BlendMode::Difference),
        CgPartVariantBlendMode::Exclusion => paint.set_blend_mode(BlendMode::Exclusion),
        CgPartVariantBlendMode::Hue => paint.set_blend_mode(BlendMode::Hue),
        CgPartVariantBlendMode::Saturation => paint.set_blend_mode(BlendMode::Saturation),
        CgPartVariantBlendMode::Color => paint.set_blend_mode(BlendMode::Color),
        CgPartVariantBlendMode::Luminosity => paint.set_blend_mode(BlendMode::Luminosity),
        _ => paint,
        // TODO: Implement these blend modes
        // CgPartVariantBlendMode::PassThrough => unimplemented!("PassThrough not supported"),
        // CgPartVariantBlendMode::Dissolve => unimplemented!("Dissolve not supported"),
        // CgPartVariantBlendMode::LinearBurn => unimplemented!("LinearBurn not supported"),
        // CgPartVariantBlendMode::DarkerColor => unimplemented!("DarkerColor not supported"),
        // CgPartVariantBlendMode::LinearDodge => unimplemented!("LinearDodge not supported"),
        // CgPartVariantBlendMode::LighterColor => unimplemented!("LighterColor not supported"),
        // CgPartVariantBlendMode::VividLight => unimplemented!("VividLight not supported"),
        // CgPartVariantBlendMode::LinearLight => unimplemented!("LinearLight not supported"),
        // CgPartVariantBlendMode::PinLight => unimplemented!("PinLight not supported"),
        // CgPartVariantBlendMode::HardMix => unimplemented!("HardMix not supported"),
        // CgPartVariantBlendMode::Subtract => unimplemented!("Subtract not supported"),
        // CgPartVariantBlendMode::Divide => unimplemented!("Divide not supported"),
    }
}
