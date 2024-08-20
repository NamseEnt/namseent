use crate::color;
use namui::*;

#[derive(Clone)]
pub struct ToolTip {
    pub global_xy: Xy<Px>,
    pub text: String,
}

impl Component for ToolTip {
    fn render(self, ctx: &RenderCtx) {
        let Self { global_xy, text } = self;

        const OFFSET: Px = px(16.0);

        let tooltip = namui::text(TextParam {
            text: text.clone(),
            x: 0.px(),
            y: 0.px(),
            align: TextAlign::Left,
            baseline: TextBaseline::Top,
            font: Font {
                size: 12.int_px(),
                name: "NotoSansKR-Regular".to_string(),
            },
            style: TextStyle {
                border: None,
                drop_shadow: None,
                color: color::STROKE_NORMAL,
                background: Some(TextStyleBackground {
                    color: color::BACKGROUND,
                    margin: Some(Ltrb {
                        left: 4.px(),
                        top: 4.px(),
                        right: 4.px(),
                        bottom: 4.px(),
                    }),
                }),
                line_height_percent: 100.percent(),
                underline: None,
            },
            max_width: None,
        });

        let Some(tooltip_bounding_box) = namui::bounding_box(&tooltip) else {
            return;
        };

        let screen_size = screen::size();
        let max_xy = (screen_size - tooltip_bounding_box.wh()).as_xy();

        ctx.component(on_top(absolute(
            (global_xy.x + OFFSET).min(max_xy.x).max(0.px()),
            (global_xy.y + OFFSET).min(max_xy.y).max(0.px()),
            tooltip,
        )))
        .done()
    }
}
