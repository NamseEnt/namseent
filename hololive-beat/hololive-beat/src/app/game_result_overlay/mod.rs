use namui::prelude::*;

#[component]
pub struct GameResultOverlay {
    pub wh: Wh<Px>,
}
impl Component for GameResultOverlay {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh: _ } = self;

        // TODO

        ctx.done()
    }
}
