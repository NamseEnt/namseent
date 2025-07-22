use crate::{
    game_state::{is_boss_stage, use_game_state},
    icon::{Icon, IconKind, IconSize},
    l10n::ui::TopBarText,
    palette,
    theme::typography::headline,
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
                once(table::fixed(px(64.), |wh, ctx| {
                    ctx.add(
                        headline(format!(
                            "{} {stage}",
                            game_state.text().ui(TopBarText::Stage)
                        ))
                        .size(crate::theme::typography::FontSize::Medium)
                        .align(crate::theme::typography::TextAlign::Center { wh })
                        .build(),
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
