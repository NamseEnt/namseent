use crate::theme::{
    palette,
    typography::{self, TextAlign, headline},
};
use namui::*;

pub struct ClearProgress {
    pub wh: Wh<Px>,
    pub clear_rate: f32,
}
impl Component for ClearProgress {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, clear_rate } = self;

        // 프로그레스바
        let progress_bar_height = wh.height;
        const PROGRESS_BAR_BG_COLOR: Color = palette::SURFACE_CONTAINER_HIGH;
        const PROGRESS_BAR_FILL_COLOR: Color = palette::PRIMARY;
        const PROGRESS_BAR_BORDER_COLOR: Color = palette::OUTLINE;

        // 클리어율 텍스트
        ctx.add(
            headline(format!("{:.2}%", clear_rate))
                .align(TextAlign::Center { wh })
                .size(typography::FontSize::Medium)
                .color(palette::ON_SURFACE)
                .stroke(1.px(), palette::ON_PRIMARY)
                .build(),
        );

        // 진행률 바
        let fill_width = wh.width * (clear_rate / 100.0);
        if fill_width > px(0.0) {
            ctx.add(rect(RectParam {
                rect: Rect::Xywh {
                    x: px(0.0),
                    y: (wh.height - progress_bar_height) * 0.5,
                    width: fill_width,
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

        // 배경
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
