use namui::prelude::*;

#[component]
pub(super) struct ImageList {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
}

impl Component for ImageList {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        // TODO: Draw List
        ctx.done()
    }
}
