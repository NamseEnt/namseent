use crate::{
    game_state::{is_boss_stage, use_game_state},
    palette,
};
use namui::*;
use namui_prebuilt::{
    simple_rect,
    table::{self},
    typography,
};
use std::iter::once;

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
                    ctx.add(HPBar {
                        wh,
                        hp: (game_state.hp / 100.0).clamp(0.0, 1.0),
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
                    ctx.add(GoldIndicator {
                        wh,
                        gold: game_state.gold,
                    });
                }),
                table::ratio(1, |_, _| {}),
            ])(Wh::new(screen_wh.width, TOP_BAR_HEIGHT), ctx);
        });
    }
}

pub struct HPBar {
    wh: Wh<Px>,
    hp: f32,
}
impl Component for HPBar {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, hp } = self;

        ctx.compose(|ctx| {
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

struct GoldIndicator {
    wh: Wh<Px>,
    gold: usize,
}
impl Component for GoldIndicator {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, gold } = self;

        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed(px(64.), |wh, ctx| {
                    ctx.add(typography::body::center(
                        wh,
                        format!("Gold"),
                        palette::ON_SURFACE,
                    ));
                }),
                table::ratio(
                    1,
                    table::padding(PADDING, |wh, ctx| {
                        ctx.add(typography::body::right(
                            wh,
                            format!("{}", gold),
                            palette::ON_SURFACE,
                        ));
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
