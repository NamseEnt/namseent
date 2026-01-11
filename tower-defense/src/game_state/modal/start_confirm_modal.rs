use crate::game_state::{force_start, set_modal, use_game_state};
use crate::icon::{Icon, IconKind, IconSize};
use crate::l10n::ui::StartConfirmModalText;
use crate::theme::button::{Button, ButtonColor, ButtonVariant};
use crate::theme::{
    palette,
    typography::{self, headline, paragraph},
};
use namui::*;
use namui_prebuilt::{simple_rect, table};

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(16.);

pub struct StartConfirmModal;

impl Component for StartConfirmModal {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();
        let game_state = use_game_state(ctx);

        let modal_wh = Wh::new(320.px(), 180.px());
        let modal_xy = ((screen_wh - modal_wh) * 0.5).to_xy();

        ctx.compose(|ctx| {
            // 모달 창
            let ctx = ctx.translate(modal_xy);
            ctx.compose(|ctx| {
                table::vertical([
                    table::fixed(
                        TITLE_HEIGHT,
                        table::horizontal([
                            table::fixed(PADDING, |_, _| {}),
                            table::ratio(1, |wh, ctx| {
                                ctx.add(
                                    headline(
                                        game_state
                                            .text()
                                            .start_confirm_modal(StartConfirmModalText::Title),
                                    )
                                    .size(typography::FontSize::Medium)
                                    .align(typography::TextAlign::LeftCenter { height: wh.height })
                                    .build(),
                                );
                            }),
                            table::fixed(48.px(), |wh, ctx| {
                                ctx.add(
                                    Button::new(
                                        wh,
                                        &|| set_modal(None),
                                        &|wh, _text_color, ctx| {
                                            ctx.add(
                                                Icon::new(IconKind::Reject)
                                                    .size(IconSize::Large)
                                                    .wh(wh),
                                            );
                                        },
                                    )
                                    .variant(ButtonVariant::Text),
                                );
                            }),
                        ]),
                    ),
                    table::ratio(
                        1,
                        table::padding(PADDING, |wh, ctx| {
                            ctx.add(
                                paragraph(
                                    game_state
                                        .text()
                                        .start_confirm_modal(StartConfirmModalText::Message),
                                )
                                .align(typography::TextAlign::Center { wh })
                                .build(),
                            );
                        }),
                    ),
                    table::fixed(
                        64.px(),
                        table::padding(
                            PADDING,
                            table::horizontal([
                                table::ratio(1, |wh, ctx| {
                                    ctx.add(
                                        Button::new(
                                            wh,
                                            &|| set_modal(None),
                                            &|wh, text_color, ctx| {
                                                ctx.add(
                                                    paragraph(
                                                        game_state.text().start_confirm_modal(
                                                            StartConfirmModalText::No,
                                                        ),
                                                    )
                                                    .color(text_color)
                                                    .align(typography::TextAlign::Center { wh })
                                                    .build(),
                                                );
                                            },
                                        )
                                        .variant(ButtonVariant::Outlined),
                                    );
                                }),
                                table::fixed(PADDING, |_, _| {}),
                                table::ratio(1, |wh, ctx| {
                                    ctx.add(
                                        Button::new(
                                            wh,
                                            &|| {
                                                set_modal(None);
                                                force_start();
                                            },
                                            &|wh, text_color, ctx| {
                                                ctx.add(
                                                    paragraph(
                                                        game_state.text().start_confirm_modal(
                                                            StartConfirmModalText::Yes,
                                                        ),
                                                    )
                                                    .color(text_color)
                                                    .align(typography::TextAlign::Center { wh })
                                                    .build(),
                                                );
                                            },
                                        )
                                        .variant(ButtonVariant::Contained)
                                        .color(ButtonColor::Primary),
                                    );
                                }),
                            ]),
                        ),
                    ),
                ])(modal_wh, ctx);
            });

            // 타이틀 배경
            ctx.add(rect(RectParam {
                rect: Wh::new(modal_wh.width, TITLE_HEIGHT).to_rect(),
                style: RectStyle {
                    stroke: None,
                    fill: Some(RectFill {
                        color: palette::SURFACE_CONTAINER,
                    }),
                    round: Some(RectRound {
                        radius: palette::ROUND,
                    }),
                },
            }));

            // 모달 배경
            ctx.add(rect(RectParam {
                rect: modal_wh.to_rect(),
                style: RectStyle {
                    stroke: None,
                    fill: Some(RectFill {
                        color: palette::SURFACE,
                    }),
                    round: Some(RectRound {
                        radius: palette::ROUND,
                    }),
                },
            }));
        })
        .attach_event(|event| {
            match event {
                Event::MouseDown { event }
                | Event::MouseMove { event }
                | Event::MouseUp { event } => {
                    if !event.is_local_xy_in() {
                        return;
                    }
                    event.stop_propagation();
                }
                Event::Wheel { event } => {
                    if !event.is_local_xy_in() {
                        return;
                    }
                    event.stop_propagation();
                }
                _ => {}
            };
        });

        // 배경 오버레이
        ctx.add(
            simple_rect(
                screen_wh,
                Color::TRANSPARENT,
                0.px(),
                Color::from_u8(0, 0, 0, 128),
            )
            .attach_event(|event| {
                let Event::MouseDown { event } = event else {
                    return;
                };
                set_modal(None);
                event.stop_propagation();
            }),
        );
    }
}
