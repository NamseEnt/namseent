mod clear_progress;
mod event_list;

use crate::flow_ui::result::clear_progress::ClearProgress;
use crate::flow_ui::result::event_list::EventList;
use crate::game_state::flow::GameFlow;
use crate::game_state::use_game_state;
use crate::game_state::{restart_game, set_modal};
use crate::l10n::ui::ResultModalText;
use crate::theme::button::{Button, ButtonColor, ButtonVariant};
use crate::theme::{
    palette,
    typography::{self, memoized_text},
};
use namui::*;
use namui_prebuilt::{simple_rect, table};

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(16.);
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

        // 규칙에 따라 modal 크기 계산
        // 세로: 스크린 세로의 60%, 최소 640, 최대 720
        let height = (screen_wh.height * 0.6).clamp(px(640.0), px(720.0));
        // 가로: 스크린 가로의 80%, 최소 480, 최대 720
        let width = (screen_wh.width * 0.8).clamp(px(480.0), px(720.0));

        let result_modal_wh = Wh { width, height };

        let modal_xy = ((screen_wh - result_modal_wh) * 0.5).to_xy();

        ctx.compose(|ctx| {
            let ctx = ctx.translate(modal_xy);

            ctx.compose(|ctx| {
                table::padding(
                    PADDING,
                    table::vertical([
                        table::fixed(TITLE_HEIGHT, |wh, ctx| {
                            ctx.add(memoized_text((), |builder| {
                                builder
                                    .headline()
                                    .size(typography::FontSize::Large)
                                    .text(game_state.text().result_modal(ResultModalText::Title))
                                    .render_center(wh)
                            }));
                        }),
                        table::fixed(PADDING, |_wh, _ctx| {}),
                        table::fixed(PROGRESS_BAR_HEIGHT, |wh, ctx| {
                            ctx.add(ClearProgress { wh, clear_rate });
                        }),
                        table::fixed(PADDING, |_wh, _ctx| {}),
                        table::ratio(1, |wh, ctx| {
                            // 이벤트 리스트
                            ctx.add(EventList {
                                wh,
                                events: &game_state.play_history.events,
                            });
                        }),
                        table::fixed(PADDING, |_, _| {}),
                        table::fixed(48.px(), |wh, ctx| {
                            ctx.add(
                                Button::new(
                                    wh,
                                    &|| {
                                        restart_game();
                                        set_modal(None);
                                    },
                                    &|wh, text_color, ctx| {
                                        ctx.add(memoized_text(&text_color, |builder| {
                                            builder
                                                .headline()
                                                .size(typography::FontSize::Medium)
                                                .color(text_color)
                                                .text(
                                                    game_state.text().result_modal(
                                                        ResultModalText::RestartButton,
                                                    ),
                                                )
                                                .render_center(wh)
                                        }));
                                    },
                                )
                                .long_press_time(2.sec())
                                .color(ButtonColor::Primary)
                                .variant(ButtonVariant::Contained),
                            );
                        }),
                    ]),
                )(result_modal_wh, ctx);
            });

            ctx.add(rect(RectParam {
                rect: result_modal_wh.to_rect(),
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
            Color::BLACK.with_alpha(225),
        ))
        .attach_event(|event| {
            match event {
                Event::MouseDown { event }
                | Event::MouseUp { event }
                | Event::MouseMove { event } => {
                    event.stop_propagation();
                }
                _ => {}
            };
        });
    }
}
