use super::*;

pub fn use_render(render: impl FnOnce(&mut ChildrenContext)) -> RenderDone {
    let mut children_ctx = ChildrenContext::new();
    render(&mut children_ctx);
    children_ctx.done(None)
}

pub fn use_render_with_event<Event: 'static + Send + Sync>(
    on_event: impl FnOnce(&Event),
    render: impl FnOnce(&mut ChildrenEventContext<Event>),
) -> RenderDone {
    let ctx = ctx();
    if let ContextFor::Event { event_callback, .. } = &ctx.context_for {
        if event_callback.component_id == ctx.instance.component_id {
            on_event(event_callback.event.as_ref().downcast_ref().unwrap());
        }
    }

    let component_id = ctx.instance.component_id;

    let mut render_ctx = ChildrenEventContext::new(component_id);
    render(&mut render_ctx);
    render_ctx.done(None)
}

pub fn use_render_with_rendering_tree(
    render: impl FnOnce(&mut ChildrenContext),
    fn_rendering_tree: impl 'static + Fn(Vec<RenderingTree>) -> RenderingTree,
) -> RenderDone {
    let mut children_ctx = ChildrenContext::new();
    render(&mut children_ctx);
    children_ctx.done(Some(Arc::new(move |children| fn_rendering_tree(children))))
}

pub struct ChildrenContext {
    children: Vec<ComponentTree>,
}

impl ChildrenContext {
    fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn add<'a>(
        &'a mut self,
        add: impl Component + 'a, // Name 'add' is to prevent showing 'child' text on rust-analyzer with vscode
    ) -> &'a mut Self {
        let child = add;
        let ctx = ctx();

        let child_tree: ComponentTree = match &ctx.context_for {
            ContextFor::Mount => mount_visit(&child),
            ContextFor::Event {
                event_callback,
                children_tree,
            } => {
                // TODO: It's ok to stop visit children if the event is for this component.

                let child_index = self.children.len();
                let child_tree = children_tree.get(child_index).unwrap();

                event_visit(&child, child_tree.clone(), event_callback.clone())
            }
            ContextFor::SetState {
                set_state_item,
                updated_sigs,
                children_tree,
            } => {
                let child_index = self.children.len();

                let child_tree = match children_tree.get(child_index) {
                    Some(child_tree)
                        if child_tree.component_instance.component_type_id
                            == child.static_type_id() =>
                    {
                        child_tree.clone()
                    }
                    _ => ComponentTree::new(&child),
                };
                set_state_visit(
                    &child,
                    child_tree,
                    set_state_item.clone(),
                    updated_sigs.clone(),
                )
            }
        };

        self.children.push(child_tree);

        self
    }

    fn done(self, fn_rendering_tree: Option<FnRenderingTree>) -> RenderDone {
        let ctx = take_ctx_and_clear_up();

        RenderDone {
            component_tree: ComponentTree {
                component_instance: ctx.instance,
                children: self.children,
                fn_rendering_tree,
            },
        }
    }
}

pub struct ChildrenEventContext<Event: 'static> {
    component_id: usize,
    inner: ChildrenContext,
    _event: std::marker::PhantomData<Event>,
}

impl<Event: 'static + Send + Sync> ChildrenEventContext<Event> {
    fn new(component_id: usize) -> Self {
        Self {
            component_id,
            inner: ChildrenContext::new(),
            _event: std::marker::PhantomData,
        }
    }
    pub fn event(&self, event: Event) -> EventCallback {
        EventCallback {
            component_id: self.component_id,
            event: Arc::new(event),
        }
    }

    pub fn add(&mut self, add: impl Component) -> &mut Self {
        self.inner.add(add);
        self
    }

    fn done(self, fn_rendering_tree: Option<FnRenderingTree>) -> RenderDone {
        self.inner.done(fn_rendering_tree)
    }
}
