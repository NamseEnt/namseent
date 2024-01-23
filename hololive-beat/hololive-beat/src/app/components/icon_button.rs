use crate::app::theme::THEME;
use namui::prelude::*;
use namui_prebuilt::typography::adjust_font_size;

#[component]
pub struct IconButton<'a> {
    pub wh: Wh<Px>,
    pub text: String,
    pub on_click: &'a dyn Fn(),
}
impl Component for IconButton<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, text, on_click } = self;

        ctx.component(
            namui::text(TextParam {
                text,
                x: wh.width / 2,
                y: wh.height / 2,
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font: Font {
                    size: adjust_font_size(wh.height),
                    name: THEME.icon_font_name.to_string(),
                },
                style: TextStyle {
                    color: THEME.text.with_alpha(216),
                    ..Default::default()
                },
                max_width: None,
            })
            .attach_event(|event| {
                let Event::MouseDown { event } = event else {
                    return;
                };
                if !event.is_local_xy_in() {
                    return;
                }
                on_click();
            }),
        );

        ctx.done()
    }
}
