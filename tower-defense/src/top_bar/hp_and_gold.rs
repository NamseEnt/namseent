use crate::{
    game_state::use_game_state, l10n::ui::TopBarText, palette, theme::typography::headline,
};
use namui::*;
use namui_prebuilt::{simple_rect, table};

const PADDING: Px = px(8.);

pub struct HPAndGoldIndicator {
    pub wh: Wh<Px>,
    pub hp: f32,
    pub gold: usize,
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
                            ctx.add(
                                headline(format!(
                                    "{} {:.0}",
                                    game_state.text().ui(TopBarText::Hp),
                                    hp * 100.0
                                ))
                                .size(crate::theme::typography::FontSize::Medium)
                                .align(crate::theme::typography::TextAlign::Center { wh })
                                .build(),
                            );
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
                            ctx.add(
                                headline(game_state.text().ui(TopBarText::Gold).to_string())
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::Center { wh })
                                    .build(),
                            );
                        }),
                        table::ratio(1, |wh, ctx| {
                            ctx.add(
                                headline(format!("{gold}"))
                                    .size(crate::theme::typography::FontSize::Medium)
                                    .align(crate::theme::typography::TextAlign::RightTop {
                                        width: wh.width,
                                    })
                                    .build(),
                            );
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
