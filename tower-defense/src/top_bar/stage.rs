use crate::{
    game_state::{is_boss_stage, use_game_state},
    icon::{Icon, IconKind, IconSize},
    l10n::ui::TopBarText,
    theme::typography::{self, memoized_text},
};
use namui::*;
use namui_prebuilt::table;
use std::iter::once;

const PADDING: Px = px(8.);

pub struct StageIndicator {
    pub wh: Wh<Px>,
    pub stage: usize,
}
impl Component for StageIndicator {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, stage } = self;
        let game_state = use_game_state(ctx);
        ctx.compose(|ctx| {
            let text_width = px(128.);
            let available = (wh.width - text_width).max(0.px());
            let icon_count = (available.as_f32() / wh.height.as_f32()).floor() as usize;

            table::horizontal(
                once(table::fixed(text_width, |wh, ctx| {
                    ctx.add(memoized_text(&stage, |mut builder| {
                        builder
                            .headline()
                            .size(typography::FontSize::Medium)
                            .text(format!(
                                "{} {stage}",
                                game_state.text().ui(TopBarText::Stage)
                            ))
                            .render_center(wh)
                    }));
                }))
                .chain((0..icon_count).map(|offset| {
                    table::fixed(
                        wh.height,
                        table::padding(PADDING, move |wh, ctx| {
                            let kind = match is_boss_stage(stage + offset) {
                                true => IconKind::EnemyBoss,
                                false => IconKind::EnemyNormal,
                            };
                            ctx.add(Icon::new(kind).size(IconSize::Large).wh(wh));
                        }),
                    )
                })),
            )(wh, ctx);
        });
    }
}
