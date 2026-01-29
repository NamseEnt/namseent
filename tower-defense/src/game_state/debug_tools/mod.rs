mod add_tower_card;
mod add_upgrade;
mod auto_setup;
pub mod monster_hp_balance;
mod route_length_info;
pub mod state_snapshot;

use crate::game_state::{effect::Effect, item::Item, mutate_game_state, set_modal};
use crate::icon::{Icon, IconKind, IconSize};
use crate::rarity::Rarity;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::{
    palette,
    typography::{self, memoized_text},
};
use add_tower_card::AddTowerCardTool;
use add_upgrade::AddUpgradeTool;
use auto_setup::AutoSetupButton;
use route_length_info::RouteLengthInfoTool;
use state_snapshot_tool::StateSnapshotTool;
mod spiral_place;
mod state_snapshot_tool;
use monster_hp_balance::MonsterHpBalanceButton;
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};
use spiral_place::PlaceSelectedTowerInSpiralButton;

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(16.);
const GAP: Px = px(8.);

pub struct DebugToolsModal;

impl Component for DebugToolsModal {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();

        let modal_wh = Wh::new(720.px(), 720.px());
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
                                ctx.add(memoized_text(
                                    (),
                                    |builder| {
                                        builder
                                            .headline()
                                            .size(typography::FontSize::Medium)
                                            .text("Debug Tools")
                                            .render_left_center(wh.height)
                                    },
                                ));
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
                                                ctx.add(MonsterHpBalanceButton {
                                                    width: _wh.width - PADDING * 2.0,
                                                });
                                            }),
                                            table::fixed(GAP, |_, _| {}),
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(AutoSetupButton {
                                                    width: _wh.width - PADDING * 2.0,
                                                });
                                            }),
                                            table::fixed(GAP, |_, _| {}),
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(AddTowerCardTool {
                                                    width: _wh.width - PADDING * 2.0,
                                                });
                                            }),
                                            table::fixed(GAP, |_, _| {}),
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(AddUpgradeTool {
                                                    width: _wh.width - PADDING * 2.0,
                                                });
                                            }),
                                            table::fixed(GAP, |_, _| {}),
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(RouteLengthInfoTool {
                                                    width: _wh.width - PADDING * 2.0,
                                                });
                                            }),
                                            table::fixed(GAP, |_, _| {}),
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(PlaceSelectedTowerInSpiralButton {
                                                    width: _wh.width - PADDING * 2.0,
                                                });
                                            }),
                                            table::fixed(GAP, |_, _| {}),
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(StateSnapshotTool {
                                                    width: _wh.width - PADDING * 2.0,
                                                });
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
                                                                typography::paragraph()
                                                                    .color(text_color)
                                                                    .text("Add Shop Reroll Item")
                                                                    .render_center(wh),
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
                                                                typography::paragraph()
                                                                    .color(text_color)
                                                                    .text("Add Hand Reroll Item")
                                                                    .render_center(wh),
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
                style: palette::title_background_style(),
            }));

            // 모달 배경
            ctx.add(rect(RectParam {
                rect: modal_wh.to_rect(),
                style: palette::modal_box_style(),
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
