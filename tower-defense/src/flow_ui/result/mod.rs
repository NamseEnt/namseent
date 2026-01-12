mod clear_progress;

use crate::flow_ui::result::clear_progress::ClearProgress;
use crate::game_state::flow::GameFlow;
use crate::game_state::set_modal;
use crate::game_state::use_game_state;
use crate::theme::button::{Button, ButtonColor, ButtonVariant};
use crate::theme::typography::TextAlign;
use crate::theme::{
    palette,
    typography::{self, headline},
};
use namui::*;
use namui_prebuilt::{simple_rect, table};

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(16.);
const RESULT_MODAL_WH: Wh<Px> = Wh {
    width: px(640.0),
    height: px(480.0),
};
const PROGRESS_BAR_HEIGHT: Px = px(24.);

pub struct ResultModal;

impl Component for ResultModal {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);

        let clear_rate = match &game_state.flow {
            GameFlow::Result { clear_rate } => *clear_rate,
            _ => 0.0, // Result 상태가 아닐 경우 기본값
        };

        let screen_wh = screen::size().into_type::<Px>();

        let modal_xy = ((screen_wh - RESULT_MODAL_WH) * 0.5).to_xy();

        ctx.compose(|ctx| {
            let ctx = ctx.translate(modal_xy);

            ctx.compose(|ctx| {
                table::padding(
                    PADDING,
                    table::vertical([
                        table::fixed(TITLE_HEIGHT, |wh, ctx| {
                            ctx.add(
                                headline("게임 결과")
                                    .align(TextAlign::Center { wh })
                                    .size(typography::FontSize::Medium)
                                    .align(typography::TextAlign::LeftCenter { height: wh.height })
                                    .build(),
                            );
                        }),
                        table::fixed(PADDING, |_wh, _ctx| {}),
                        table::fixed(PROGRESS_BAR_HEIGHT, |wh, ctx| {
                            ctx.add(ClearProgress { wh, clear_rate });
                        }),
                        table::fixed(PADDING, |_wh, _ctx| {}),
                        table::ratio(1, |_wh, _ctx| {
                            // 내용 영역 - placeholder
                        }),
                        table::fixed(PADDING, |_, _| {}),
                        table::fixed(48.px(), |wh, ctx| {
                            ctx.add(
                                Button::new(
                                    wh,
                                    &|| {
                                        set_modal(None);
                                        // TODO: 다음 스테이지로 이동하거나 메인 메뉴로 돌아가기
                                    },
                                    &|_wh, text_color, ctx| {
                                        ctx.add(
                                            headline("확인")
                                                .align(TextAlign::Center { wh })
                                                .size(typography::FontSize::Medium)
                                                .color(text_color)
                                                .build(),
                                        );
                                    },
                                )
                                .color(ButtonColor::Primary)
                                .variant(ButtonVariant::Contained),
                            );
                        }),
                    ]),
                )(RESULT_MODAL_WH, ctx);
            });

            ctx.add(rect(RectParam {
                rect: RESULT_MODAL_WH.to_rect(),
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: palette::OUTLINE,
                        width: 1.px(),
                        border_position: BorderPosition::Inside,
                    }),
                    fill: Some(RectFill {
                        color: palette::SURFACE_CONTAINER,
                    }),
                    round: Some(RectRound {
                        radius: palette::ROUND,
                    }),
                },
            }));
        });

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK.with_alpha(200),
        ));
    }
}
