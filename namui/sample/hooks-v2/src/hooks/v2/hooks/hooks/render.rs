use super::*;

pub fn use_render<'a, C: Component + 'a>(render: impl 'a + FnOnce() -> C) -> RenderDone {
    handle_render_with_component(|| render())
}

pub fn use_render_with_event<'a, C: Component + 'a, Event: 'static + Send + Sync>(
    on_event: impl 'a + FnOnce(&Event),
    render: impl 'a + FnOnce(EventContext<Event>) -> C,
) -> RenderDone {
    handle_render_with_component(|| render(EventContext::new(ctx().instance.component_id)))
}

fn handle_render_with_component<'a, C: Component + 'a>(child: impl FnOnce() -> C) -> RenderDone {
    let ctx = ctx();
    let component_instance = ctx.instance.clone();
    let child = child();
    RenderDone {
        component_holder: ComponentHolder {
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
        component_holder: ComponentHolder {
            component_instance,
            children: vec![],
            rendering_tree: Some(rendering_tree),
        },
    }
}
