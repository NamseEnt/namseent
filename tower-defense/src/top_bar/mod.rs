use crate::{
    game_state::{is_boss_stage, level_rarity_weight, mutate_game_state, use_game_state},
    l10n::ui::TopBarText,
    palette,
    theme::typography::{self, Headline, Paragraph},
};
use namui::*;
use namui_prebuilt::{button, simple_rect, table};
use std::{iter::once, num::NonZero};

const TOP_BAR_HEIGHT: Px = px(48.);
const ITEM_WIDTH: Px = px(256.);
const PADDING: Px = px(8.);

pub struct TopBar {
    pub screen_wh: Wh<Px>,
}
impl Component for TopBar {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh } = self;

        let game_state = use_game_state(ctx);

        ctx.compose(|ctx| {
            table::horizontal([
                table::ratio(1, |_, _| {}),
                table::fixed(ITEM_WIDTH, |wh, ctx| {
                    ctx.add(HPAndGoldIndicator {
                        wh,
                        hp: (game_state.hp / 100.0).clamp(0.0, 1.0),
                        gold: game_state.gold,
                    });
                }),
                table::fixed(PADDING, |_, _| {}),
                table::fixed(ITEM_WIDTH, |wh, ctx| {
                    ctx.add(StageIndicator {
                        wh,
                        stage: game_state.stage,
                    });
                }),
                table::fixed(PADDING, |_, _| {}),
                table::fixed(ITEM_WIDTH, |wh, ctx| {
                    ctx.add(LevelIndicator {
                        wh,
                        level: game_state.level.get(),
                        level_up_cost: game_state.level_up_cost(),
                        gold: game_state.gold,
                    });
                }),
                table::ratio(1, |_, _| {}),
            ])(Wh::new(screen_wh.width, TOP_BAR_HEIGHT), ctx);
        });
    }
}

pub struct HPAndGoldIndicator {
    wh: Wh<Px>,
    hp: f32,
    gold: usize,
}
impl Component for HPAndGoldIndicator {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, hp, gold } = self;
        let game_state = use_game_state(ctx);
        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(
                    1,
                    table::horizontal([
                        table::fixed(px(64.), |wh, ctx| {
                            ctx.add(Headline {
                                text: format!(
                                    "{} {:.0}",
                                    game_state.text().ui(TopBarText::Hp),
                                    hp * 100.0
                                ),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                        table::ratio(
                            1,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.add(simple_rect(
                                    Wh::new(wh.width * (hp).clamp(0.0, 1.0), wh.height),
                                    Color::TRANSPARENT,
                                    0.px(),
                                    palette::PRIMARY,
                                ));

                                ctx.add(simple_rect(
                                    wh,
                                    Color::TRANSPARENT,
                                    0.px(),
                                    palette::SURFACE,
                                ));
                            }),
                        ),
                    ]),
                ),
                table::ratio(
                    1,
                    table::horizontal([
                        table::fixed(px(64.), |wh, ctx| {
                            ctx.add(Headline {
                                text: game_state.text().ui(TopBarText::Gold).to_string(),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Headline {
                                text: format!("{gold}"),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::RightTop { width: wh.width },
                                max_width: None,
                            });
                        }),
                        table::fixed(PADDING, |_, _| {}),
                    ]),
                ),
            ])(wh, ctx);
        });

        ctx.add(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            palette::SURFACE_CONTAINER,
        ));
    }
}

pub struct StageIndicator {
    wh: Wh<Px>,
    stage: usize,
}
impl Component for StageIndicator {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, stage } = self;
        let game_state = use_game_state(ctx);
        ctx.compose(|ctx| {
            table::horizontal(
                once(table::fixed(px(64.), |wh, ctx| {
                    ctx.add(Headline {
                        text: format!("{} {stage}", game_state.text().ui(TopBarText::Stage)),
                        font_size: typography::FontSize::Medium,
                        text_align: typography::TextAlign::Center { wh },
                        max_width: None,
                    });
                }))
                .chain((0..5).map(|offset| {
                    table::fixed(
                        wh.height,
                        table::padding(PADDING, move |wh, ctx| {
                            let path = Path::new().add_oval(wh.to_rect());
                            let paint = Paint::new(match is_boss_stage(stage + offset as usize) {
                                true => palette::COMMON,
                                false => palette::EPIC,
                            });
                            ctx.add(namui::path(path, paint));
                        }),
                    )
                })),
            )(wh, ctx);
        });

        ctx.add(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            palette::SURFACE_CONTAINER,
        ));
    }
}

struct LevelIndicator {
    wh: Wh<Px>,
    level: usize,
    level_up_cost: usize,
    gold: usize,
}
impl Component for LevelIndicator {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            level,
            level_up_cost,
            gold,
        } = self;
        let game_state = use_game_state(ctx);

        let (mouse_hovering, set_mouse_hovering) = ctx.state(|| false);

        let can_upgrade = level < 10 && gold >= level_up_cost;

        let level_up = || {
            mutate_game_state(move |game_state| {
                game_state.level = game_state.level.checked_add(1).expect("Level overflow");
                println!("Level up to {}", game_state.level);
                game_state.gold -= level_up_cost;
            });
        };

        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed(px(64.), |wh, ctx| {
                    ctx.add(Headline {
                        text: format!("{} {level}", game_state.text().ui(TopBarText::Level)),
                        font_size: typography::FontSize::Medium,
                        text_align: typography::TextAlign::Center { wh },
                        max_width: None,
                    });
                }),
                table::ratio(
                    1,
                    table::padding(PADDING, |wh, ctx| {
                        ctx.add(button::TextButton {
                            rect: wh.to_rect(),
                            text: format!(
                                "{} {level_up_cost}",
                                game_state.text().ui(TopBarText::LevelUp)
                            ),
                            text_color: match can_upgrade {
                                true => palette::ON_PRIMARY,
                                false => palette::ON_SURFACE,
                            },
                            stroke_color: palette::OUTLINE,
                            stroke_width: 1.px(),
                            fill_color: match can_upgrade {
                                true => palette::PRIMARY,
                                false => palette::SURFACE_CONTAINER_HIGH,
                            },
                            mouse_buttons: vec![MouseButton::Left],
                            on_mouse_up_in: |_| {
                                if !can_upgrade {
                                    return;
                                }
                                level_up();
                            },
                        })
                        .attach_event(|event| {
                            let Event::MouseMove { event } = event else {
                                return;
                            };

                            let mouse_move_is_local_xy_in = event.is_local_xy_in();
                            if *mouse_hovering != mouse_move_is_local_xy_in {
                                set_mouse_hovering.set(mouse_move_is_local_xy_in);
                            }
                        });
                    }),
                ),
            ])(wh, ctx);
        });

        ctx.compose(|ctx| {
            if !*mouse_hovering {
                return;
            }

            ctx.translate((0.px(), wh.height))
                .on_top()
                .add(LevelUpDetails {
                    width: wh.width,
                    current_level: level,
                });
        });

        ctx.add(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            palette::SURFACE_CONTAINER,
        ));
    }
}

struct LevelUpDetails {
    width: Px,
    current_level: usize,
}
impl Component for LevelUpDetails {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            width,
            current_level,
        } = self;
        let game_state = crate::game_state::use_game_state(ctx);

        const LINE_HEIGHT: Px = px(32.);
        const CONTAINER_HEIGHT: Px = px(128.);
        const RARITY_LABEL_WIDTH: Px = px(64.);

        let current_level = ctx.track_eq(&current_level);
        let weights = ctx.memo(|| {
            let current_level = NonZero::new(*current_level).expect("Level must be non-zero");
            let next_level = current_level
                .checked_add(1)
                .unwrap()
                .min(NonZero::new(10).unwrap());
            let mut current_weights = level_rarity_weight(current_level);
            let current_total_weight: usize = current_weights.iter().sum();
            current_weights.iter_mut().for_each(|weight| {
                *weight = (*weight as f32 / current_total_weight as f32 * 100.0).round() as usize;
            });
            let mut next_weights = level_rarity_weight(next_level);
            let next_total_weight: usize = next_weights.iter().sum();
            next_weights.iter_mut().for_each(|weight| {
                *weight = (*weight as f32 / next_total_weight as f32 * 100.0).round() as usize;
            });
            [
                [current_weights[0], next_weights[0]],
                [current_weights[1], next_weights[1]],
                [current_weights[2], next_weights[2]],
                [current_weights[3], next_weights[3]],
            ]
        });

        let wh = Wh::new(width, CONTAINER_HEIGHT);

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(
                    LINE_HEIGHT,
                    table::horizontal([
                        table::fixed(PADDING, |_, _| {}),
                        table::fixed(RARITY_LABEL_WIDTH, |wh, ctx| {
                            ctx.add(Headline {
                                text: game_state.text().ui(TopBarText::RarityCommon).to_string(),
                                font_size: typography::FontSize::Small,
                                text_align: typography::TextAlign::LeftCenter { height: wh.height },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: format!("{}%", weights[0][0]),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: ">>>".to_string(),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: format!("{}%", weights[0][1]),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                    ]),
                ),
                table::fixed(
                    LINE_HEIGHT,
                    table::horizontal([
                        table::fixed(PADDING, |_, _| {}),
                        table::fixed(RARITY_LABEL_WIDTH, |wh, ctx| {
                            ctx.add(Headline {
                                text: game_state.text().ui(TopBarText::RarityRare).to_string(),
                                font_size: typography::FontSize::Small,
                                text_align: typography::TextAlign::LeftCenter { height: wh.height },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: format!("{}%", weights[1][0]),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: ">>>".to_string(),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: format!("{}%", weights[1][1]),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                    ]),
                ),
                table::fixed(
                    LINE_HEIGHT,
                    table::horizontal([
                        table::fixed(PADDING, |_, _| {}),
                        table::fixed(RARITY_LABEL_WIDTH, |wh, ctx| {
                            ctx.add(Headline {
                                text: game_state.text().ui(TopBarText::RarityEpic).to_string(),
                                font_size: typography::FontSize::Small,
                                text_align: typography::TextAlign::LeftCenter { height: wh.height },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: format!("{}%", weights[2][0]),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: ">>>".to_string(),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: format!("{}%", weights[2][1]),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                    ]),
                ),
                table::fixed(
                    LINE_HEIGHT,
                    table::horizontal([
                        table::fixed(PADDING, |_, _| {}),
                        table::fixed(RARITY_LABEL_WIDTH, |wh, ctx| {
                            ctx.add(Headline {
                                text: game_state.text().ui(TopBarText::RarityLegendary).to_string(),
                                font_size: typography::FontSize::Small,
                                text_align: typography::TextAlign::LeftCenter { height: wh.height },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: format!("{}%", weights[3][0]),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: ">>>".to_string(),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(Paragraph {
                                text: format!("{}%", weights[3][1]),
                                font_size: typography::FontSize::Medium,
                                text_align: typography::TextAlign::Center { wh },
                                max_width: None,
                            });
                        }),
                    ]),
                ),
            ])(wh, ctx);
        });
    }
}
