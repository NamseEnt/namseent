use crate::icon::Icon;
use namui::*;

impl Component for Icon {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(self.to_rendering_tree());
    }
}
