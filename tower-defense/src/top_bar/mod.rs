use crate::{
    game_state::{is_boss_stage, mutate_game_state, use_game_state},
    palette,
};
use namui::*;
use namui_prebuilt::{button, simple_rect, table, typography};
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
                        level: game_state.level,
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

        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(
                    1,
                    table::horizontal([
                        table::fixed(px(64.), |wh, ctx| {
                            ctx.add(typography::body::center(
                                wh,
                                format!("HP {:.0}", hp * 100.0),
                                palette::ON_SURFACE,
                            ));
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
                            ctx.add(typography::body::center(
                                wh,
                                format!("Gold"),
                                palette::ON_SURFACE,
                            ));
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(typography::body::right(
                                wh,
                                format!("{}", gold),
                                palette::ON_SURFACE,
                            ));
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

        ctx.compose(|ctx| {
            table::horizontal(
                once(table::fixed(px(64.), |wh, ctx| {
                    ctx.add(typography::body::center(
                        wh,
                        format!("Stage {}", stage),
                        palette::ON_SURFACE,
                    ));
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
    level: NonZero<usize>,
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

        let can_upgrade = level < NonZero::new(10).unwrap() && gold >= level_up_cost;

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
                    ctx.add(typography::body::center(
                        wh,
                        format!("Level {}", level),
                        palette::ON_SURFACE,
                    ));
                }),
                table::ratio(
                    1,
                    table::padding(PADDING, |wh, ctx| {
                        ctx.add(button::TextButton {
                            rect: wh.to_rect(),
                            text: format!("레벨업 {}", level_up_cost),
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
                        });
                    }),
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
