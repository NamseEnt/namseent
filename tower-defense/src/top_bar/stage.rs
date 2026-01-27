use crate::{
    game_state::{is_boss_stage, use_game_state},
    icon::{Icon, IconKind, IconSize},
    l10n::ui::TopBarText,
    palette,
    theme::typography::{self},
};
use namui::*;
use namui_prebuilt::{simple_rect, table};
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
            table::horizontal(
                once(table::fixed(px(128.), |wh, ctx| {
                    let text = format!(
                        "{} {stage}",
                        game_state.text().ui(TopBarText::Stage)
                    );
                    ctx.add(
                        typography::headline()
                            .size(typography::FontSize::Medium)
                            .text(&text)
                            .center(wh),
                    );
                }))
                .chain((0..5).map(|offset| {
                    table::fixed(
                        wh.height,
                        table::padding(PADDING, move |wh, ctx| {
                            let kind = match is_boss_stage(stage + offset as usize) {
                                true => IconKind::EnemyBoss,
                                false => IconKind::EnemyNormal,
                            };
                            ctx.add(Icon::new(kind).size(IconSize::Large).wh(wh));
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
