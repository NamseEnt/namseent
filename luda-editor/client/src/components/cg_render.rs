use crate::storage::get_project_cg_part_variant_image_url;
use namui::prelude::*;
use rpc::data::*;

pub struct CgRenderProps {
    pub rect: Rect<Px>,
    pub project_id: Uuid,
    pub cg_id: Uuid,
}

pub fn render_cg(props: CgRenderProps, screen_cg: &ScreenCg, cg_file: &CgFile) -> RenderingTree {
    render(screen_cg.parts.iter().map(|screen_part| {
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
    let rect = Rect::Xywh {
        x: props.rect.x() + props.rect.width() * variant.rect.x(),
        y: props.rect.y() + props.rect.height() * variant.rect.y(),
        width: props.rect.width() * variant.rect.width(),
        height: props.rect.height() * variant.rect.height(),
    };

    let url =
        get_project_cg_part_variant_image_url(props.project_id, props.cg_id, variant.id).unwrap();

    namui::image(ImageParam {
        rect,
        source: ImageSource::Url { url },
        style: ImageStyle {
            fit: ImageFit::Fill,
            paint: None,
        },
    })
}
