use namui::prelude::*;

#[component]
pub struct EventTrap;

impl Component for EventTrap {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        ctx.stop_event_propagation();
        ctx.done()
    }
}
