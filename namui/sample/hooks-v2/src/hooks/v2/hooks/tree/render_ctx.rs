use super::*;

pub struct RenderCtx {
    pub(crate) context_for: ContextFor,
    pub(crate) instance: Arc<ComponentInstance>,
    pub(crate) state_index: AtomicUsize,
    pub(crate) effect_index: AtomicUsize,
    pub(crate) memo_index: AtomicUsize,
    pub(crate) as_index: AtomicUsize,
    tree_ctx: TreeContext,
}

impl<'a> RenderCtx {
    pub(crate) fn new(
        context_for: ContextFor,
        instance: Arc<ComponentInstance>,
        tree_ctx: TreeContext,
    ) -> Self {
        Self {
            context_for,
            instance,
            state_index: Default::default(),
            effect_index: Default::default(),
            memo_index: Default::default(),
            as_index: Default::default(),
            tree_ctx,
        }
    }

    // pub fn use_atom_init< T: Debug + Send + Sync + 'static>(
    //     &'a self,
    //     atom: &'static Atom<T>,
    //     init: impl FnOnce() -> T,
    // ) -> (Sig<'a, T>, SetState<T>) {
    //     todo!()
    // }

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

    pub fn use_children(
        &'a self,
        use_children: impl 'a + FnOnce(ChildrenContext) -> ChildrenDone,
    ) -> RenderDone {
        let children_ctx = ChildrenContext::new(self.tree_ctx.clone(), self.instance.clone(), None);
        use_children(children_ctx);

        RenderDone {}
    }

    pub fn use_children_with_event<Event: 'static + Debug + Send + Sync>(
        &'a self,
        on_event: impl FnOnce(Event),
        children: impl FnOnce(ChildrenEventContext<Event>) -> ChildrenDone,
    ) -> RenderDone {
        let children_ctx =
            ChildrenEventContext::new(self.tree_ctx.clone(), self.instance.clone(), None);
        children(children_ctx);

        RenderDone {}
    }

    pub fn use_children_with_rendering_tree(
        &'a self,
        children: impl 'a + FnOnce(ChildrenContext) -> ChildrenDone,
        fn_rendering_tree: impl 'a + FnOnce(Vec<RenderingTree>) -> RenderingTree,
    ) -> RenderDone {
        let fn_rendering_tree = unsafe {
            std::mem::transmute::<
                Box<dyn FnOnce(Vec<RenderingTree>) -> RenderingTree>,
                Box<dyn FnOnce(Vec<RenderingTree>) -> RenderingTree>,
            >(Box::new(fn_rendering_tree))
        };
        let children_ctx = ChildrenContext::new(
            self.tree_ctx.clone(),
            self.instance.clone(),
            Some(fn_rendering_tree),
        );
        children(children_ctx);

        RenderDone {}
    }
}

pub struct ChildrenDone {}

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
