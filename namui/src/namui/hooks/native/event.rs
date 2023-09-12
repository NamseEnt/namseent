use super::*;
use crate::*;

pub(crate) fn attach_event<'a, C: 'a + Component>(
    component: C,
    on_event: impl 'a + FnOnce(Event),
) -> AttachEvent<'a, C> {
    AttachEvent {
        component,
        on_event: Mutex::new(Some(Box::new(on_event))),
    }
}

type OnEvent<'a> = Box<dyn 'a + FnOnce(Event)>;
pub struct AttachEvent<'a, C: Component> {
    component: C,
    on_event: Mutex<Option<OnEvent<'a>>>,
}

impl<'a, C: 'a + Component> StaticType for AttachEvent<'a, C> {}
impl<'a, C: 'a + Component> Debug for AttachEvent<'a, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttachEvent")
            .field("component", &self.component)
            .finish()
    }
}
impl<'b, C: 'b + Component> Component for AttachEvent<'b, C> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        ctx.component(self.component);
        let done = ctx.done();

        ctx.on_raw_event(|raw_event| {
            let on_event = self.on_event.lock().unwrap().take().unwrap();
            invoke_on_event(
                &ctx.tree_ctx,
                on_event,
                raw_event,
                ctx.inverse_matrix(),
                &done.rendering_tree,
                ctx,
            );
        });

        done
    }
}
