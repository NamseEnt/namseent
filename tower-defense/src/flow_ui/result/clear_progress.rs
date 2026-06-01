use crate::{
    animation::with_spring,
    theme::{palette, typography, typography::memoized_text},
};
use namui::*;

pub struct ClearProgress {
    pub wh: Wh<Px>,
    pub clear_rate: f32,
}
impl Component for ClearProgress {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, clear_rate } = self;

        let fill_width = wh.width * (clear_rate / 100.0);
        let fill_width_with_spring =
            with_spring(ctx, fill_width, 0.px(), |px| px.as_f32(), || 0.px());

        let progress_bar_height = wh.height;
        const PROGRESS_BAR_BG_COLOR: Color = palette::SURFACE_CONTAINER_HIGH;
        const PROGRESS_BAR_FILL_COLOR: Color = palette::PRIMARY;
        const PROGRESS_BAR_BORDER_COLOR: Color = palette::OUTLINE;

        ctx.add(memoized_text(&clear_rate, |mut builder| {
            builder
                .headline()
                .size(typography::FontSize::Medium)
                .bold()
                .color(palette::WHITE)
                .stroke(2.px(), palette::DARK_CHARCOAL)
                .text(format!("{:.2}%", clear_rate))
                .render_center(wh)
        }));

        ctx.add(rect(RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: (wh.height - progress_bar_height) * 0.5,
                width: wh.width,
                height: progress_bar_height,
            },
            style: RectStyle {
                fill: None,
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: px(2.0),
                    border_position: BorderPosition::Outside,
                }),
                round: Some(RectRound { radius: px(4.0) }),
            },
        }));

        if fill_width_with_spring > px(0.0) {
            ctx.add(rect(RectParam {
                rect: Rect::Xywh {
                    x: px(0.0),
                    y: (wh.height - progress_bar_height) * 0.5,
                    width: fill_width_with_spring,
                    height: progress_bar_height,
                },
                style: RectStyle {
                    fill: Some(RectFill {
                        color: PROGRESS_BAR_FILL_COLOR,
                    }),
                    stroke: None,
                    round: Some(RectRound { radius: px(4.0) }),
                },
            }));
        }

        ctx.add(rect(RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: (wh.height - progress_bar_height) * 0.5,
                width: wh.width,
                height: progress_bar_height,
            },
            style: RectStyle {
                fill: Some(RectFill {
                    color: PROGRESS_BAR_BG_COLOR,
                }),
                stroke: Some(RectStroke {
                    color: PROGRESS_BAR_BORDER_COLOR,
                    width: px(1.0),
                    border_position: BorderPosition::Inside,
                }),
                round: Some(RectRound { radius: px(4.0) }),
            },
        }));
    }
}
