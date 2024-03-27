use crate::storage::get_project_cg_part_variant_image_url;
use namui::prelude::*;
use rpc::data::*;

#[component]
pub struct CgRender<'a> {
    pub rect: Rect<Px>,
    pub project_id: Uuid,
    pub screen_cg: &'a ScreenCg,
    pub cg_file: &'a CgFile,
}

impl Component for CgRender<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let render_cg_variant = |ctx: &mut ComposeCtx, variant: &rpc::data::CgPartVariant| {
            let rect = Rect::Xywh {
                x: self.rect.x() + self.rect.width() * variant.rect.x(),
                y: self.rect.y() + self.rect.height() * variant.rect.y(),
                width: self.rect.width() * variant.rect.width(),
                height: self.rect.height() * variant.rect.height(),
            };

            let url = get_project_cg_part_variant_image_url(
                self.project_id,
                self.screen_cg.id,
                variant.id,
            )
            .unwrap();

            ctx.clip(Path::new().add_rect(self.rect), ClipOp::Intersect)
                .add(namui::image(ImageParam {
                    rect,
                    source: ImageSource::Url { url },
                    style: ImageStyle {
                        fit: ImageFit::Fill,
                        paint: None,
                    },
                }));
        };
        let render_cg_part =
            |ctx: &mut ComposeCtx, part: &rpc::data::ScreenCgPart, cg_part: &CgPart| match part {
                rpc::data::ScreenCgPart::Single { variant_name, .. } => {
                    let Some(variant_name) = variant_name.as_ref() else {
                        return;
                    };

                    let Some(variant) = cg_part
                        .variants
                        .iter()
                        .find(|variant| &variant.name == variant_name)
                    else {
                        return;
                    };

                    render_cg_variant(ctx, variant)
                }
                rpc::data::ScreenCgPart::Multi { variant_names, .. } => cg_part
                    .variants
                    .iter()
                    .filter(|variant| variant_names.contains(&variant.name))
                    .for_each(|variant| render_cg_variant(ctx, variant)),
                rpc::data::ScreenCgPart::AlwaysOn { .. } => cg_part
                    .variants
                    .iter()
                    .for_each(|variant| render_cg_variant(ctx, variant)),
            };

        ctx.compose(|ctx| {
            ctx.compose_with_key(self.screen_cg.id.to_string(), |ctx| {
                for screen_part in self.screen_cg.parts.iter() {
                    let Some(cg_part) = self
                        .cg_file
                        .parts
                        .iter()
                        .find(|part| part.name == screen_part.name())
                    else {
                        continue;
                    };

                    ctx.compose_with_key(&cg_part.name, |ctx| {
                        render_cg_part(ctx, screen_part, cg_part);
                    });
                }
            });
        })
        .done()
    }
}
