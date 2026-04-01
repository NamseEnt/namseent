use crate::{
    game_state::use_game_state,
    l10n::ui::TopBarText,
    theme::typography::{self, memoized_text},
};
use namui::*;

pub struct RunIndicator {
    pub wh: Wh<Px>,
    pub stage: usize,
}
impl Component for RunIndicator {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, stage } = self;
        let game_state = use_game_state(ctx);

        ctx.add(memoized_text(&stage, |mut builder| {
            builder
                .headline()
                .size(typography::FontSize::Medium)
                .text(format!("{} {stage}", game_state.text().ui(TopBarText::Run)))
                .render_center(wh)
        }));
    }
}
