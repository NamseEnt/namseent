use super::*;
use namui::ClosurePtr;
use std::cell::RefCell;

pub struct ChildrenContext {
    direct_children: RefCell<Vec<Box<dyn Component>>>,
    // children: Vec<ComponentTree>,
    tree_ctx: TreeContext,
    component_instance: Arc<ComponentInstance>,
    fn_rendering_tree: Option<FnRenderingTree>,
}

impl<'a> ChildrenContext {
    pub(crate) fn new(
        tree_ctx: TreeContext,
        component_instance: Arc<ComponentInstance>,
        fn_rendering_tree: Option<FnRenderingTree>,
    ) -> Self {
        Self {
            direct_children: Default::default(),
            tree_ctx,
            component_instance,
            fn_rendering_tree,
        }
    }

    pub fn add(
        &'a self,
        add: impl Component + 'a, // Name 'add' is to prevent showing 'child' text on rust-analyzer with vscode
    ) {
        self.direct_children.borrow_mut().push(unsafe {
            std::mem::transmute::<Box<dyn Component>, Box<dyn Component>>(Box::new(add))
        })
    }

    pub fn done(self) -> ChildrenDone {
        self.tree_ctx
            .clone()
            .end_up_one_component_rendering(ComponentRenderResult {
                children: self.direct_children.into_inner(),
                component_instance: self.component_instance,
                fn_rendering_tree: self.fn_rendering_tree,
            });

        ChildrenDone {
            tree_ctx: self.tree_ctx,
        }
    }

    pub(crate) fn closure<T: 'static>(&'a self, call: impl 'a + Fn(T)) -> ClosurePtr<T, ()> {
        ClosurePtr::new(unsafe {
            std::mem::transmute::<Box<dyn Fn(T)>, Box<dyn Fn(T)>>(Box::new(call))
        })
    }
}

pub struct ChildrenEventContext<Event: 'static> {
    component_id: usize,
    inner: ChildrenContext,
    _event: std::marker::PhantomData<Event>,
}

impl<Event: 'static + Send + Sync> ChildrenEventContext<Event> {
    pub(crate) fn new(
        tree_ctx: TreeContext,
        component_instance: Arc<ComponentInstance>,
        fn_rendering_tree: Option<FnRenderingTree>,
    ) -> Self {
        Self {
            component_id: component_instance.component_id,
            inner: ChildrenContext::new(tree_ctx, component_instance, fn_rendering_tree),
            _event: std::marker::PhantomData,
        }
    }
    pub fn event(&self, event: Event) -> EventCallback {
        EventCallback {
            component_id: self.component_id,
            event: Arc::new(event),
        }
    }
    pub fn event_with_param<Param>(
        &self,
        event_with_param: impl 'static + Send + Sync + Fn(Param) -> Option<Event>,
    ) -> EventCallbackWithParam<Param> {
        EventCallbackWithParam {
            component_id: self.component_id,
            closure: Arc::new(move |param: Param| {
                let event = event_with_param(param);
                event.map(|event| {
                    Arc::new(event) as Arc<(dyn std::any::Any + Send + Sync + 'static)>
                })
            }),
        }
    }
    pub fn add<'a>(
        &'a self,
        add: impl Component + 'a, // Name 'add' is to prevent showing 'child' text on rust-analyzer with vscode
    ) {
        self.inner.add(add);
    }
    pub fn done(self) -> ChildrenDone {
        self.inner.done()
    }
}
