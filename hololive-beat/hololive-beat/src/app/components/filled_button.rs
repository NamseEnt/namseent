use crate::app::theme::THEME;
use namui::prelude::*;
use namui_prebuilt::typography;

#[component]
pub struct FilledButton<'a> {
    pub wh: Wh<Px>,
    pub text: String,
    pub on_click: &'a dyn Fn(),
}
impl Component for FilledButton<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, text, on_click } = self;

        let center_xy = wh / 2;

        ctx.component(namui::text(TextParam {
            text,
            x: center_xy.width,
            y: center_xy.height,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font: Font {
                size: typography::adjust_font_size(wh.height),
                name: THEME.font_name.to_string(),
            },
            style: TextStyle {
                color: THEME.text,
                ..Default::default()
            },
            max_width: None,
        }));

        ctx.component(
            path(
                Path::new().add_rect(Rect::zero_wh(wh)),
                Paint::new(THEME.primary.dark.with_alpha(25)).set_blend_mode(BlendMode::Screen),
            )
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
