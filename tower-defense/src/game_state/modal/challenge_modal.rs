use crate::game_state::{mutate_game_state, set_modal, use_game_state};
use crate::icon::{Icon, IconKind, IconSize};
use crate::theme::typography::TextAlign;
use crate::theme::{
    button::{Button, ButtonColor, ButtonVariant},
    palette,
    typography::{self, headline, paragraph},
};
use namui::*;
use namui_prebuilt::{simple_rect, table};
use std::iter::once;

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(8.);
const LIST_ITEM_HEIGHT: Px = px(64.);
const MODAL_WIDTH: Px = px(360.);
const MODAL_HEIGHT: Px = px(256.);

pub struct ChallengeModal;

impl Component for ChallengeModal {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();
        let game_state = use_game_state(ctx);

        let choices = &game_state.monster_spawn_state.challenge_choices;
        let selected = game_state.monster_spawn_state.challenge_selected;

        let close_modal = move || {
            set_modal(None);
        };

        let modal_wh = Wh::new(MODAL_WIDTH, MODAL_HEIGHT);
        let modal_xy = ((screen_wh - modal_wh) * 0.5).to_xy();

        ctx.compose(|ctx| {
            // Modal container
            let ctx = ctx.translate(modal_xy);

            ctx.compose(|ctx| {
                table::vertical([
                    table::fixed(
                        TITLE_HEIGHT,
                        table::horizontal([
                            table::fixed(PADDING, |_, _| {}),
                            table::ratio(1, |wh, ctx| {
                                ctx.add(
                                    headline("도전")
                                        .size(typography::FontSize::Medium)
                                        .align(typography::TextAlign::LeftCenter {
                                            height: wh.height,
                                        })
                                        .build(),
                                );
                            }),
                            table::fixed(48.px(), |wh, ctx| {
                                ctx.add(
                                    Button::new(wh, &close_modal, &|wh, _text_color, ctx| {
                                        ctx.add(
                                            Icon::new(IconKind::Reject)
                                                .size(IconSize::Large)
                                                .wh(wh),
                                        );
                                    })
                                    .variant(ButtonVariant::Text),
                                );
                            }),
                        ]),
                    ),
                    table::ratio(
                        1,
                        table::vertical(
                            once(table::ratio(1, |_, _| {}))
                                .chain((0..3).map(|index| {
                                    table::fixed(
                                        LIST_ITEM_HEIGHT,
                                        table::padding(PADDING, move |wh, ctx| {
                                            let is_selected = selected[index];
                                            let monster = choices[index].kind;
                                            let label = format!(
                                                "{} {}",
                                                monster.emoji(),
                                                monster.display_name()
                                            );
                                            let variant = if is_selected {
                                                ButtonVariant::Contained
                                            } else {
                                                ButtonVariant::Outlined
                                            };

                                            ctx.add(
                                                Button::new(
                                                    wh,
                                                    &move || {
                                                        mutate_game_state(move |game_state| {
                                                            game_state
                                                                .monster_spawn_state
                                                                .toggle_challenge_selection(index);
                                                        });
                                                    },
                                                    &|wh, text_color, ctx| {
                                                        ctx.compose(|ctx| {
                                                            table::horizontal([
                                                                table::fixed(
                                                                    wh.height,
                                                                    |wh, ctx| {
                                                                        ctx.add(
                                                                        Icon::new(
                                                                            IconKind::EnemyNamed,
                                                                        )
                                                                        .size(IconSize::Large)
                                                                        .wh(wh),
                                                                    );
                                                                    },
                                                                ),
                                                                table::fixed(PADDING, |_, _| {}),
                                                                table::ratio(1, |wh, ctx| {
                                                                    ctx.add(
                                                                        paragraph(label.clone())
                                                                            .color(text_color)
                                                                            .align(TextAlign::LeftCenter { height: wh.height })
                                                                            .build(),
                                                                    );
                                                                }),
                                                                table::fixed(
                                                                    wh.height,
                                                                    |wh, ctx| {
                                                                        let check_icon =
                                                                            match is_selected {
                                                                                true => {
                                                                                    IconKind::Accept
                                                                                }
                                                                                false => {
                                                                                    IconKind::Reject
                                                                                }
                                                                            };
                                                                        ctx.add(
                                                                            Icon::new(check_icon)
                                                                                .size(
                                                                                    IconSize::Large,
                                                                                )
                                                                                .wh(wh),
                                                                        );
                                                                    },
                                                                ),
                                                            ])(
                                                                wh, ctx
                                                            );
                                                        });
                                                    },
                                                )
                                                .variant(variant)
                                                .color(ButtonColor::Primary),
                                            );
                                        }),
                                    )
                                }))
                                .chain(once(table::ratio(1, |_, _| {}))),
                        ),
                    ),
                ])(modal_wh, ctx);
            });

            // Title background (added after content to render behind)
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

            // Modal background added last so it sits at the very back
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

        // Overlay background closes modal on click
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
                close_modal();
                event.stop_propagation();
            }),
        );
    }
}
