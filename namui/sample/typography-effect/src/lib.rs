use namui::prelude::*;

pub fn main() {
    namui::start(|| TypographyEffectExample)
}

#[namui::component]
struct TypographyEffectExample;

impl Component for TypographyEffectExample {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let screen_wh = namui::screen::size();

        ctx.component(namui_prebuilt::typography::effect::glow(
            "Hello world!",
            Font {
                size: 64.int_px(),
                name: "NotoSansKR-Regular".to_string(),
            },
            screen_wh.as_xy().into_type::<Px>() / 2.0,
            Paint::new(Color::WHITE),
            TextAlign::Center,
            TextBaseline::Middle,
            Blur::Normal { sigma: 24.0 },
            24.px(),
            Color::from_u8(255, 128, 0, 255),
        ));

        ctx.component(rect(RectParam {
            rect: Rect::Xywh {
                x: screen_wh.width.into_px() / 4.0,
                y: screen_wh.height.into_px() / 4.0,
                width: screen_wh.width.into_px() / 2.0,
                height: screen_wh.height.into_px() / 2.0,
            },
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: Color::BLACK,
                    width: 1.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill {
                    color: Color::BLACK,
                }),
                round: None,
            },
        }));

        ctx.done()
    }
}
