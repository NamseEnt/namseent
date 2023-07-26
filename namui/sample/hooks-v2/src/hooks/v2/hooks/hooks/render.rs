use super::*;

pub fn use_render<'a, C: Component + 'a>(render: impl 'a + FnOnce() -> C) -> RenderDone {
    handle_render_with_component(|| render())
}

pub fn use_render_with_event<'a, C: Component + 'a, Event: 'static + Send + Sync>(
    on_event: impl 'a + FnOnce(&Event),
    render: impl 'a + FnOnce(EventContext<Event>) -> C,
) -> RenderDone {
    let ctx = ctx();
    if let ContextFor::Event { event_callback } = &ctx.context_for {
        if event_callback.component_id == ctx.instance.component_id {
            on_event(event_callback.event.as_ref().downcast_ref().unwrap());
        }
    }

    handle_render_with_component(|| render(EventContext::new(ctx.instance.component_id)))
}

fn handle_render_with_component<'a, C: Component + 'a>(child: impl FnOnce() -> C) -> RenderDone {
    let ctx = ctx();
    let component_instance = ctx.instance.clone();
    let child = child();
    RenderDone {
        component_tree: ComponentTree {
            component_instance,
            children: vec![mount_visit(&child)],
            rendering_tree: None,
        },
    }
}

pub fn use_render_with_rendering_tree(rendering_tree: RenderingTree) -> RenderDone {
    let ctx = ctx();
    let component_instance = ctx.instance.clone();
    RenderDone {
        component_tree: ComponentTree {
            component_instance,
            children: vec![],
            rendering_tree: Some(rendering_tree),
        },
    }
}

pub struct EventContext<Event: 'static> {
    component_id: usize,
    _event: std::marker::PhantomData<Event>,
}

impl<Event: 'static + Send + Sync> EventContext<Event> {
    fn new(component_id: usize) -> Self {
        Self {
            component_id,
            _event: std::marker::PhantomData,
        }
    }
    pub fn event(&self, event: Event) -> EventCallback {
        EventCallback {
            component_id: self.component_id,
            event: Arc::new(event),
        }
    }
}
