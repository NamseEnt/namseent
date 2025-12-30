mod add_tower_card;
mod add_upgrade;
pub mod state_snapshot;

use crate::game_state::{effect::Effect, item::Item, mutate_game_state, set_modal};
use crate::icon::{Icon, IconKind, IconSize};
use crate::rarity::Rarity;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::{
    palette,
    typography::{self, headline, paragraph},
};
use add_tower_card::AddTowerCardTool;
use add_upgrade::AddUpgradeTool;
use state_snapshot_tool::StateSnapshotTool;
mod state_snapshot_tool;
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(16.);
const GAP: Px = px(8.);

pub struct DebugToolsModal;

impl Component for DebugToolsModal {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();

        let modal_wh = Wh::new(600.px(), 400.px());
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
                                    headline("Debug Tools")
                                        .size(typography::FontSize::Medium)
                                        .align(typography::TextAlign::LeftCenter {
                                            height: wh.height,
                                        })
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
                        table::padding(PADDING, |_wh, ctx| {
                            ctx.add(AutoScrollViewWithCtx {
                                wh: _wh,
                                scroll_bar_width: PADDING,
                                content: |scroll_ctx| {
                                    scroll_ctx.compose(|ctx| {
                                        table::vertical([
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(AddTowerCardTool { width: _wh.width - PADDING * 2.0 });
                                            }),
                                            table::fixed(GAP, |_, _| {}),
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(AddUpgradeTool { width: _wh.width - PADDING * 2.0 });
                                            }),
                                            table::fixed(GAP, |_, _| {}),
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(StateSnapshotTool { width: _wh.width - PADDING * 2.0 });
                                            }),
                                            table::fixed(GAP, |_, _| {}),
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(
                                                    Button::new(
                                                        Wh::new(_wh.width - PADDING * 2.0, 40.px()),
                                                        &|| {
                                                            mutate_game_state(|gs| {
                                                                gs.items.push(Item {
                                                                    effect: Effect::ExtraShopReroll,
                                                                    rarity: Rarity::Common,
                                                                    value: 0.0.into(),
                                                                });
                                                            });
                                                        },
                                                        &|wh, text_color, ctx| {
                                                            ctx.add(
                                                                paragraph("Add Shop Reroll Item")
                                                                    .color(text_color)
                                                                    .align(typography::TextAlign::Center {
                                                                        wh,
                                                                    })
                                                                    .build(),
                                                            );
                                                        },
                                                    )
                                                    .variant(ButtonVariant::Outlined),
                                                );
                                            }),
                                            table::fixed(GAP, |_, _| {}),
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(
                                                    Button::new(
                                                        Wh::new(_wh.width - PADDING * 2.0, 40.px()),
                                                        &|| {
                                                            mutate_game_state(|gs| {
                                                                gs.items.push(Item {
                                                                    effect: Effect::ExtraReroll,
                                                                    rarity: Rarity::Common,
                                                                    value: 0.0.into(),
                                                                });
                                                            });
                                                        },
                                                        &|wh, text_color, ctx| {
                                                            ctx.add(
                                                                paragraph("Add Hand Reroll Item")
                                                                    .color(text_color)
                                                                    .align(typography::TextAlign::Center {
                                                                        wh,
                                                                    })
                                                                    .build(),
                                                            );
                                                        },
                                                    )
                                                    .variant(ButtonVariant::Outlined),
                                                );
                                            }),
                                        ])(_wh, ctx);
                                    });
                                },
                            });
                        }),
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
