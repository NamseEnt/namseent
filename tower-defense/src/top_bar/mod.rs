mod level;
mod stage;

use crate::game_state::use_game_state;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::{
    icon::{Icon, IconKind, IconSize},
    palette,
    theme::typography::{FontSize, memoized_text},
};
use namui::*;
use namui_prebuilt::table;

const TOP_BAR_HEIGHT: Px = px(48.);
const ITEM_WIDTH: Px = px(128.);
const PADDING: Px = px(8.);

const BG_OVERSIZE_H: Px = px(4.);
const BG_OVERSIZE_V: Px = px(4.);

pub struct TopBar {
    pub wh: Wh<Px>,
}
impl Component for TopBar {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);

        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed_no_clip(ITEM_WIDTH, |wh, ctx| {
                    ctx.add(crate::top_bar::level::LevelIndicator {
                        wh,
                        level: game_state.level.get(),
                    });
                }),
                table::fixed_no_clip(PADDING, |_, _| {}),
                table::fixed_no_clip(ITEM_WIDTH, |wh, ctx| {
                    ctx.compose(|ctx| {
                        let hp_pct = (game_state.hp / 100.0).clamp(0.0, 1.0);
                        table::horizontal([
                            table::fixed_no_clip(48.px(), |wh, ctx| {
                                ctx.add(Icon::new(IconKind::Health).size(IconSize::Large).wh(wh));
                            }),
                            table::fixed_no_clip(32.px(), |wh, ctx| {
                                ctx.add(memoized_text(&hp_pct, |mut builder| {
                                    builder
                                        .headline()
                                        .size(FontSize::Medium)
                                        .text(format!("{:.0}", hp_pct * 100.0))
                                        .render_center(wh)
                                }));
                            }),
                            table::ratio(1, |_, _| {}),
                        ])(wh, ctx);
                    });
                }),
                table::fixed_no_clip(PADDING, |_, _| {}),
                table::fixed_no_clip(ITEM_WIDTH, |wh, ctx| {
                    ctx.compose(|ctx| {
                        table::horizontal([
                            table::fixed_no_clip(48.px(), |wh, ctx| {
                                ctx.add(Icon::new(IconKind::Gold).size(IconSize::Large).wh(wh));
                            }),
                            table::fixed_no_clip(32.px(), |wh, ctx| {
                                ctx.add(memoized_text(&game_state.gold, |mut builder| {
                                    builder
                                        .headline()
                                        .size(FontSize::Medium)
                                        .text(format!("{}", game_state.gold))
                                        .render_center(wh)
                                }));
                            }),
                            table::ratio(1, |_, _| {}),
                        ])(wh, ctx);
                    });
                }),
                table::ratio(1, |wh, ctx| {
                    ctx.add(crate::top_bar::stage::StageIndicator {
                        wh,
                        stage: game_state.stage,
                    });
                }),
            ])(Wh::new(wh.width, TOP_BAR_HEIGHT), ctx);
        });

        ctx.translate((-BG_OVERSIZE_H, -BG_OVERSIZE_V))
            .add(PaperContainerBackground {
                width: wh.width + BG_OVERSIZE_H * 2.0,
                height: TOP_BAR_HEIGHT + BG_OVERSIZE_V * 2.0,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Sticky,
                color: palette::SURFACE_CONTAINER,
                shadow: true,
                arrow: None,
            });
    }
}
