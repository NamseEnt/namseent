use namui::prelude::*;

#[component]
pub struct EventTrap<C>
where
    C: Component + Sized,
{
    component: C,
}
impl<C> Component for EventTrap<C>
where
    C: Component + Sized,
{
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self { component } = self;

        ctx.component(component.attach_event(|event| {
            match event {
                Event::MouseDown { event } => {
                    event.stop_propagation();
                }
                Event::MouseMove { event } => {
                    event.stop_propagation();
                }
                Event::MouseUp { event } => {
                    event.stop_propagation();
                }
                Event::Wheel { event } => {
                    event.stop_propagation();
                }
                _ => {} // below don't support stop_propagation
                        // .on_key_down(|event: KeyboardEvent| event.stop_propagation())
                        // .on_key_up(|event: KeyboardEvent| event.stop_propagation())
            }
        }));
        ctx.done()
    }
}

#[component]
pub struct EventTrapMouse<C>
where
    C: Component + Sized,
{
    component: C,
}
impl<C> Component for EventTrapMouse<C>
where
    C: Component + Sized,
{
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self { component } = self;

        ctx.component(component.attach_event(|event| {
            match event {
                Event::MouseDown { event } => {
                    if event.is_local_xy_in() {
                        event.stop_propagation();
                    }
                }
                Event::MouseMove { event } => {
                    if event.is_local_xy_in() {
                        event.stop_propagation();
                    }
                }
                Event::MouseUp { event } => {
                    if event.is_local_xy_in() {
                        event.stop_propagation();
                    }
                }
                Event::Wheel { event } => {
                    event.stop_propagation();
                }
                _ => {} // below don't support stop_propagation
                        // .on_key_down(|event: KeyboardEvent| event.stop_propagation())
                        // .on_key_up(|event: KeyboardEvent| event.stop_propagation())
            }
        }));
        ctx.done()
    }
}
