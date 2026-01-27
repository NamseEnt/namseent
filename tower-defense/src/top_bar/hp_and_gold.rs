use crate::icon::{Icon, IconKind, IconSize};
use crate::theme::typography::{self, FontSize};
use crate::{palette};
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
        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(
                    1,
                    table::horizontal([
                        table::fixed(px(32.), |wh, ctx| {
                            ctx.add(Icon::new(IconKind::Health).size(IconSize::Medium).wh(wh));
                        }),
                        table::fixed(48.px(), |wh, ctx| {
                            let hp_text = format!("{:.0}", hp * 100.0);
                            ctx.add(
                                typography::headline()
                                    .text(&hp_text)
                                    .size(FontSize::Medium)
                                    .center(wh),
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
                        table::fixed(32.px(), |wh, ctx| {
                            ctx.add(Icon::new(IconKind::Gold).size(IconSize::Medium).wh(wh));
                        }),
                        table::ratio(1, |wh, ctx| {
                            let gold_text = format!("{gold}");
                            ctx.add(
                                typography::headline()
                                    .text(&gold_text)
                                    .size(FontSize::Medium)
                                    .right_top(wh.width),
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
