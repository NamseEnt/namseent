use namui::prelude::*;

#[component]
pub(super) struct CgList {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
}

impl Component for CgList {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        // TODO: Draw List
        ctx.done()
    }
}
