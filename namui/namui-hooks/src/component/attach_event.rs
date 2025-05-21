use super::*;
use crate::*;

pub(crate) fn attach_event<'a, C: 'a + Component>(
    component: C,
    on_event: impl 'a + FnOnce(Event),
) -> AttachEvent<'a, C> {
    AttachEvent {
        component,
        on_event: Box::new(on_event),
    }
}

type OnEvent<'a> = Box<dyn 'a + FnOnce(Event)>;
pub struct AttachEvent<'a, C: Component> {
    component: C,
    on_event: OnEvent<'a>,
}

impl<'a, C: 'a + Component> Component for AttachEvent<'a, C> {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(self.component).attach_event(self.on_event);
    }
}
