use super::*;
use crate::RenderingTree;
use std::marker::PhantomData;

pub struct RenderCtx<'a> {
    _a: PhantomData<&'a ()>,
    pub(crate) instance: Arc<ComponentInstance>,
    pub(crate) state_index: AtomicUsize,
    pub(crate) effect_index: AtomicUsize,
    pub(crate) memo_index: AtomicUsize,
    pub(crate) as_index: AtomicUsize,
    tree_ctx: TreeContext,
    direct_children: Mutex<Vec<Box<dyn Component>>>,
}

impl<'a> RenderCtx<'a> {
    pub(crate) fn new(instance: Arc<ComponentInstance>, tree_ctx: TreeContext) -> Self {
        Self {
            _a: Default::default(),
            instance,
            state_index: Default::default(),
            effect_index: Default::default(),
            memo_index: Default::default(),
            as_index: Default::default(),
            tree_ctx,
            direct_children: Default::default(),
        }
    }

    pub fn use_atom_init<T: Debug + Send + Sync + 'static>(
        &'a self,
        atom: &'static Atom<T>,
        init: impl FnOnce() -> T,
    ) -> (Sig<'a, T>, SetState<T>) {
        handle_use_atom_init(atom, init)
    }

    pub fn use_atom<T: Debug + Send + Sync + 'static>(
        &'a self,
        atom: &'static Atom<T>,
    ) -> (Sig<'a, T>, SetState<T>) {
        handle_use_atom(atom)
    }

    pub fn use_state<T: 'static + Debug + Send + Sync>(
        &'a self,
        init: impl FnOnce() -> T,
    ) -> (Sig<'a, T>, SetState<T>) {
        handle_use_state(self, init)
    }

    pub fn use_memo<T: 'static + Debug + Send + Sync>(
        &'a self,
        use_memo: impl FnOnce() -> T,
    ) -> Sig<'a, T> {
        handle_use_memo(self, use_memo)
    }

    pub fn use_effect(&'a self, title: &'static str, effect: impl FnOnce()) {
        handle_use_effect(self, title, effect)
    }

    pub fn add(
        &'a self,
        add: impl Component + 'a, // Name 'add' is to prevent showing 'child' text on rust-analyzer with vscode
    ) {
        self.direct_children.lock().unwrap().push(unsafe {
            std::mem::transmute::<Box<dyn Component>, Box<dyn Component>>(Box::new(add))
        })
    }

    pub fn try_add<C: Component + 'a>(&'a self, try_add: Option<C>) {
        if let Some(component) = try_add {
            self.add(component)
        }
    }

    pub fn done(self) -> RenderDone {
        self.tree_ctx
            .clone()
            .end_up_one_component_rendering(ComponentRenderResult {
                children: self.direct_children.into_inner().unwrap(),
                component_instance: self.instance,
                fn_rendering_tree: None,
            });

        RenderDone {
            tree_ctx: self.tree_ctx,
        }
    }

    pub fn done_with_rendering_tree(
        self,
        fn_rendering_tree: impl 'a + FnOnce(Vec<RenderingTree>) -> RenderingTree,
    ) -> RenderDone {
        let fn_rendering_tree = unsafe {
            std::mem::transmute::<
                Box<dyn FnOnce(Vec<RenderingTree>) -> RenderingTree>,
                Box<dyn FnOnce(Vec<RenderingTree>) -> RenderingTree>,
            >(Box::new(fn_rendering_tree))
        };

        self.tree_ctx
            .clone()
            .end_up_one_component_rendering(ComponentRenderResult {
                children: self.direct_children.into_inner().unwrap(),
                component_instance: self.instance,
                fn_rendering_tree: Some(fn_rendering_tree),
            });

        RenderDone {
            tree_ctx: self.tree_ctx,
        }
    }

    pub(crate) fn is_sig_updated(&self, sig_id: &SigId) -> bool {
        self.tree_ctx.is_sig_updated(sig_id)
    }

    pub(crate) fn add_sig_updated(&self, sig_id: SigId) {
        self.tree_ctx.add_sig_updated(sig_id)
    }

    pub fn use_web_event(&'a self, use_web_event: impl 'a + Fn(&crate::web::WebEvent)) {
        let unsafe_casted = unsafe {
            std::mem::transmute::<
                Box<dyn Fn(&crate::web::WebEvent)>,
                Box<dyn Fn(&crate::web::WebEvent)>,
            >(Box::new(use_web_event))
        };
        *self.instance.web_event_listener.lock().unwrap() = Some(unsafe_casted);
    }
}

pub struct ChildrenDone {
    pub(crate) tree_ctx: TreeContext,
}
impl ChildrenDone {
    fn to_render_done(&self) -> RenderDone {
        RenderDone {
            tree_ctx: self.tree_ctx.clone(),
        }
    }
}

pub(crate) enum ContextFor {
    Mount,
    // Event {
    //     event_callback: EventCallback,
    //     children_tree: Vec<ComponentTree>,
    // },
    // SetState {
    //     set_state_item: SetStateItem,
    //     updated_sigs: Arc<Mutex<HashSet<SigId>>>,
    //     children_tree: VecDeque<ComponentTree>,
    // },
}

impl Debug for ContextFor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextFor::Mount => write!(f, "ContextFor::Mount"),
            // ContextFor::Event {
            //     event_callback,
            //     children_tree: children,
            // } => write!(
            //     f,
            //     "ContextFor::Event {{ event_callback: {:?}, children: {:?} }}",
            //     event_callback, children
            // ),
            // ContextFor::SetState {
            //     updated_sigs,
            //     set_state_item,
            //     children_tree: children,
            // } => write!(
            //     f,
            //     "ContextFor::SetState {{ updated_sigs: {:?}, set_state_item: {:?}, children: {:?} }}",
            //     updated_sigs.lock().unwrap(),
            //     set_state_item,
            //     children,
            // ),
        }
    }
}
