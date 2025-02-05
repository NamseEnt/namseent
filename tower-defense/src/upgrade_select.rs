use crate::{
    game_state::{flow::GameFlow, mutate_game_state},
    palette,
    upgrade::{merge_or_append_upgrade, Upgrade},
};
use namui::*;
use namui_prebuilt::{
    button::TextButton,
    table::{self, ratio},
    typography,
};

const PADDING: Px = px(4.0);
const UPGRADE_SELECT_WH: Wh<Px> = Wh {
    width: px(640.0),
    height: px(480.0),
};
const UPGRADE_SELECT_BUTTON_WH: Wh<Px> = Wh {
    width: px(64.0),
    height: px(36.0),
};

pub struct UpgradeSelectModal<'a> {
    pub screen_wh: Wh<Px>,
    pub upgrades: &'a [Upgrade; 3],
}
impl Component for UpgradeSelectModal<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            screen_wh,
            upgrades,
        } = self;

        let (opened, set_opened) = ctx.state(|| true);

        let toggle_open = || {
            set_opened.mutate(|opened| *opened = !*opened);
        };

        let on_upgrade_select = |upgrade: &Upgrade| {
            let upgrade = upgrade.clone();
            mutate_game_state(|state| {
                merge_or_append_upgrade(&mut state.upgrades, upgrade);
                state.flow = GameFlow::SelectingTower;
            });
        };

        let offset = ((screen_wh - UPGRADE_SELECT_WH) * 0.5).as_xy();

        ctx.compose(|ctx| {
            ctx.translate(offset).add(UpgradeSelectOpenButton {
                opened: *opened,
                toggle_open: &toggle_open,
            });
        });

        ctx.compose(|ctx| {
            if !*opened {
                return;
            }
            ctx.translate(offset).add(UpgradeSelect {
                upgrades,
                on_upgrade_select: &on_upgrade_select,
            });
        });
    }
}

struct UpgradeSelectOpenButton<'a> {
    opened: bool,
    toggle_open: &'a dyn Fn(),
}
impl Component for UpgradeSelectOpenButton<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            opened,
            toggle_open,
        } = self;

        ctx.compose(|ctx| {
            ctx.translate((0.px(), UPGRADE_SELECT_BUTTON_WH.height))
                .add(TextButton {
                    rect: UPGRADE_SELECT_BUTTON_WH.to_rect(),
                    text: format!("강화 선택 {}", if opened { "🔼" } else { "🔽" }),
                    text_color: palette::ON_SURFACE,
                    stroke_color: palette::OUTLINE,
                    stroke_width: 1.px(),
                    fill_color: palette::SURFACE_CONTAINER,
                    mouse_buttons: vec![MouseButton::Left],
                    on_mouse_up_in: |_| {
                        toggle_open();
                    },
                });
        });
    }
}

struct UpgradeSelect<'a> {
    upgrades: &'a [Upgrade; 3],
    on_upgrade_select: &'a dyn Fn(&Upgrade),
}
impl Component for UpgradeSelect<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            upgrades,
            on_upgrade_select,
        } = self;

        ctx.compose(|ctx| {
            table::padding(
                PADDING,
                table::horizontal(upgrades.iter().map(|upgrade| {
                    ratio(1, |wh, ctx| {
                        ctx.add(UpgradeSelectItem {
                            wh,
                            upgrade,
                            on_upgrade_select,
                        });
                    })
                })),
            )(UPGRADE_SELECT_WH, ctx);
        });
    }
}

struct UpgradeSelectItem<'a> {
    wh: Wh<Px>,
    upgrade: &'a Upgrade,
    on_upgrade_select: &'a dyn Fn(&Upgrade),
}
impl Component for UpgradeSelectItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            upgrade,
            on_upgrade_select,
        } = self;

        ctx.compose(|ctx| {
            table::padding(PADDING, |wh, ctx| {
                ctx.compose(|ctx| {
                    table::vertical([
                        table::fixed(
                            wh.width,
                            table::padding(PADDING, |_wh, _ctx| {
                                // TODO: Icons
                            }),
                        ),
                        table::ratio(
                            1,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.compose(|ctx| {
                                    table::padding(
                                        PADDING,
                                        table::vertical([
                                            table::fixed(36.px(), |_wh, ctx| {
                                                ctx.add(typography::body::left_top(
                                                    upgrade.name(),
                                                    palette::ON_SURFACE,
                                                ));
                                            }),
                                            table::fixed(PADDING, |_, _| {}),
                                            table::ratio(1, |_wh, ctx| {
                                                ctx.add(typography::body::left_top(
                                                    upgrade.description(),
                                                    palette::ON_SURFACE_VARIANT,
                                                ));
                                            }),
                                        ]),
                                    )(wh, ctx);
                                });

                                ctx.add(rect(RectParam {
                                    rect: wh.to_rect(),
                                    style: RectStyle {
                                        stroke: Some(RectStroke {
                                            color: palette::OUTLINE,
                                            width: 1.px(),
                                            border_position: BorderPosition::Inside,
                                        }),
                                        fill: Some(RectFill {
                                            color: palette::SURFACE,
                                        }),
                                        round: Some(RectRound {
                                            radius: palette::ROUND,
                                        }),
                                    },
                                }));
                            }),
                        ),
                    ])(wh, ctx);
                });

                ctx.add(
                    rect(RectParam {
                        rect: wh.to_rect(),
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
                    })
                    .attach_event(|event| {
                        let Event::MouseDown { event } = event else {
                            return;
                        };
                        if !event.is_local_xy_in() {
                            return;
                        }
                        on_upgrade_select(upgrade);
                    }),
                );
            })(wh, ctx);
        });
    }
}
