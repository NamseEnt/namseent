use crate::{
    icon::{Icon, IconKind, IconSize},
    theme::typography::{FontSize, memoized_text},
};
use namui::*;
use namui_prebuilt::table;

pub struct LevelIndicator {
    pub wh: Wh<Px>,
    pub level: usize,
}
impl Component for LevelIndicator {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, level } = self;

        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed(48.px(), |wh, ctx| {
                    ctx.add(Icon::new(IconKind::Level).size(IconSize::Large).wh(wh));
                }),
                table::fixed(32.px(), |wh, ctx| {
                    ctx.add(memoized_text(&level, |mut builder| {
                        builder
                            .headline()
                            .size(FontSize::Medium)
                            .text(format!("{level}"))
                            .render_center(wh)
                    }));
                }),
                table::ratio(1, |_, _| {}),
            ])(wh, ctx);
        });
    }
}
