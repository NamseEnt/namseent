use crate::icon::{Icon, IconKind, IconSize};
use crate::theme::button::Button;
use crate::{
    game_state::{mutate_game_state, upgrade::Upgrade, use_game_state},
    l10n::upgrade::UpgradeKindText,
    palette,
    theme::typography::{FontSize, TextAlign, headline, paragraph},
};
use namui::*;
use namui_prebuilt::table::{self, ratio_no_clip};

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
    pub upgrades: &'a [Upgrade],
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

        let on_upgrade_select = |upgrade: Upgrade| {
            mutate_game_state(move |state| {
                state.upgrade(upgrade);
                state.goto_selecting_tower();
            });
        };

        let offset = ((screen_wh - UPGRADE_SELECT_WH) * 0.5).to_xy();

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
                .add(Button::new(
                    UPGRADE_SELECT_BUTTON_WH,
                    &|| {
                        toggle_open();
                    },
                    &|wh, text_color, ctx| {
                        ctx.add(namui::text(TextParam {
                            text: format!("강화 선택 {}", if opened { "🔼" } else { "🔽" }),
                            x: wh.width / 2.0,
                            y: wh.height / 2.0,
                            align: namui::TextAlign::Center,
                            baseline: TextBaseline::Middle,
                            font: Font {
                                size: 14.int_px(),
                                name: "NotoSansKR-Regular".to_string(),
                            },
                            style: TextStyle {
                                color: text_color,
                                background: None,
                                border: None,
                                drop_shadow: None,
                                line_height_percent: 100.percent(),
                                underline: None,
                            },
                            max_width: None,
                        }));
                    },
                ));
        });
    }
}

struct UpgradeSelect<'a> {
    upgrades: &'a [Upgrade],
    on_upgrade_select: &'a dyn Fn(Upgrade),
}
impl Component for UpgradeSelect<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            upgrades,
            on_upgrade_select,
        } = self;

        ctx.compose(|ctx| {
            table::padding_no_clip(
                PADDING,
                table::horizontal(upgrades.iter().map(|&upgrade| {
                    ratio_no_clip(1, move |wh, ctx| {
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
    upgrade: Upgrade,
    on_upgrade_select: &'a dyn Fn(Upgrade),
}
impl Component for UpgradeSelectItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            upgrade,
            on_upgrade_select,
        } = self;
        let game_state = use_game_state(ctx);
        ctx.compose(|ctx| {
            table::padding_no_clip(PADDING, |wh, ctx| {
                ctx.compose(|ctx| {
                    table::vertical([
                        table::fixed_no_clip(
                            wh.width,
                            table::padding_no_clip(PADDING, |wh, ctx| {
                                ctx.translate(((wh.width - IconSize::Large.px()) * 0.5, -PADDING))
                                    .add(
                                        Icon::new(IconKind::Rarity {
                                            rarity: upgrade.rarity,
                                        })
                                        .size(IconSize::Large)
                                        .wh(Wh::new(IconSize::Large.px(), PADDING)),
                                    );

                                ctx.compose(|ctx| {
                                    table::padding(PADDING, |wh, ctx| {
                                        ctx.add(upgrade.kind.thumbnail(wh));
                                    })(wh, ctx);
                                });

                                ctx.add(rect(RectParam {
                                    rect: wh.to_rect(),
                                    style: RectStyle {
                                        stroke: None,
                                        fill: Some(RectFill {
                                            color: palette::SURFACE_CONTAINER_LOWEST,
                                        }),
                                        round: Some(RectRound {
                                            radius: palette::ROUND,
                                        }),
                                    },
                                }));
                            }),
                        ),
                        table::ratio(
                            1,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.compose(|ctx| {
                                    table::padding(PADDING, |wh, ctx| {
                                        table::vertical([
                                            table::fit(table::FitAlign::LeftTop, |ctx| {
                                                ctx.add(
                                                    headline(game_state.text().upgrade_kind(
                                                        UpgradeKindText::Name(&upgrade.kind),
                                                    ))
                                                    .size(FontSize::Small)
                                                    .align(TextAlign::LeftTop)
                                                    .max_width(wh.width)
                                                    .build(),
                                                );
                                            }),
                                            table::fixed(PADDING, |_, _| {}),
                                            table::ratio(1, |_wh, ctx| {
                                                ctx.add(
                                                    paragraph(game_state.text().upgrade_kind(
                                                        UpgradeKindText::Description(&upgrade.kind),
                                                    ))
                                                    .size(FontSize::Medium)
                                                    .align(TextAlign::LeftTop)
                                                    .max_width(wh.width)
                                                    .build(),
                                                );
                                            }),
                                        ])(wh, ctx);
                                    })(wh, ctx);
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
