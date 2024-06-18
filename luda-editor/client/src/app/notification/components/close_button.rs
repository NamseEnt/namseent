use namui::*;
use namui_prebuilt::simple_rect;

#[component]
pub struct CloseButton {
    pub wh: Wh<Px>,
    pub color: Color,
}
impl Component for CloseButton {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh, color } = self;
        ctx.compose(|ctx| {
            namui_prebuilt::table::padding(wh.height / 6.0, |wh, ctx| {
                let path = Path::new()
                    .move_to(0.px(), 0.px())
                    .line_to(wh.width, wh.height)
                    .move_to(wh.width, 0.px())
                    .line_to(0.px(), wh.height);
                let paint = Paint::new()
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(wh.height / 8.0)
                    .set_color(color);

                ctx.add(namui::path(path, paint));
                ctx.add(simple_rect(
                    wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::TRANSPARENT,
                ));
            })(wh, ctx);
        });

        
    }
}
