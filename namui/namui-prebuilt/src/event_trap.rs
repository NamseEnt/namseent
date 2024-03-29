use namui::*;

#[component]
pub struct EventTrap;

impl Component for EventTrap {
    fn render(self, ctx: &RenderCtx)  {
        ctx.set_event_propagation(false);
        
    }
}
