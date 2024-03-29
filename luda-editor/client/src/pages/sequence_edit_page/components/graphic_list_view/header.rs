use crate::color;
use namui::*;
use namui_prebuilt::{simple_rect, table};

#[component]
pub struct Header {
    pub wh: Wh<Px>,
}
impl Component for Header {
    fn render(self, ctx: &RenderCtx)  {
        const PADDING: Wh<Px> = Wh {
            width: px(8.0),
            height: px(4.0),
        };

        let Self { wh } = self;

        ctx.compose(|ctx| {
            table::hooks::vertical_padding(PADDING.height, |wh, ctx| {
                table::hooks::horizontal_padding(PADDING.width, |wh, ctx| {
                    ctx.add(text(TextParam {
                        text: "Graphic List".to_string(),
                        x: 0.px(),
                        y: wh.height / 2.0,
                        align: TextAlign::Left,
                        baseline: TextBaseline::Middle,
                        font: Font {
                            size: 12.int_px(),
                            name: "NotoSansKR-Regular".to_string(),
                        },
                        style: TextStyle {
                            color: color::STROKE_NORMAL,
                            ..Default::default()
                        },
                        max_width: Some(wh.width),
                    }));
                })(wh, ctx);
            })(wh, ctx);
        });

        ctx.component(simple_rect(
            wh,
            color::STROKE_NORMAL,
            1.px(),
            color::BACKGROUND,
        ));

        
    }
}
