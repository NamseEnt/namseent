use namui::prelude::*;

#[component]
pub struct EventTrap;

impl Component for EventTrap {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        ctx.on_raw_event(|event| {
            let event_ext: &dyn EventExt = match event {
                RawEvent::MouseDown { event } => event,
                RawEvent::MouseMove { event } => event,
                RawEvent::MouseUp { event } => event,
                RawEvent::Wheel { event } => event,
                RawEvent::KeyDown { event } => event,
                RawEvent::KeyUp { event } => event,
                RawEvent::TextInputKeyDown { event } => event,
                _ => return,
            };
            event_ext.stop_propagation();
        });
        ctx.done()
    }
}
