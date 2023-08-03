use super::*;
use crate::RenderingTree;

pub struct RenderCtx {
    pub(crate) instance: Arc<ComponentInstance>,
    pub(crate) state_index: AtomicUsize,
    pub(crate) effect_index: AtomicUsize,
    pub(crate) memo_index: AtomicUsize,
    pub(crate) track_eq_index: AtomicUsize,
    tree_ctx: TreeContext,
    direct_children: Mutex<Vec<Box<dyn Component>>>,
}

impl<'a> RenderCtx {
    pub(crate) fn new(instance: Arc<ComponentInstance>, tree_ctx: TreeContext) -> Self {
        Self {
            instance,
            state_index: Default::default(),
            effect_index: Default::default(),
            memo_index: Default::default(),
            track_eq_index: Default::default(),
            tree_ctx,
            direct_children: Default::default(),
        }
    }

    pub fn atom_init<T: Debug + Send + Sync + 'static>(
        &'a self,
        atom: &'static Atom<T>,
        init: impl FnOnce() -> T,
    ) -> (Sig<'a, T>, SetState<T>) {
        handle_atom_init(atom, init)
    }

    pub fn atom<T: Debug + Send + Sync + 'static>(
        &'a self,
        atom: &'static Atom<T>,
    ) -> (Sig<'a, T>, SetState<T>) {
        handle_atom(atom)
    }

    pub fn state<T: 'static + Debug + Send + Sync>(
        &'a self,
        init: impl FnOnce() -> T,
    ) -> (Sig<'a, T>, SetState<T>) {
        handle_state(self, init)
    }

    pub fn memo<T: 'static + Debug + Send + Sync>(
        &'a self,
        memo: impl FnOnce() -> T,
    ) -> Sig<'a, T> {
        handle_memo(self, memo)
    }

    pub fn track_eq<T: 'static + Debug + Send + Sync + PartialEq + Clone>(
        &'a self,
        track_eq: &T,
    ) -> Sig<'a, T> {
        handle_track_eq(self, track_eq)
    }

    pub fn effect(&'a self, title: &'static str, effect: impl FnOnce()) {
        handle_effect(self, title, effect)
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

    pub fn done(&'a self) -> RenderDone {
        self.done_inner(None)
    }

    pub fn done_with_rendering_tree(
        &'a self,
        fn_rendering_tree: impl 'a + FnOnce(Vec<RenderingTree>) -> RenderingTree,
    ) -> RenderDone {
        let fn_rendering_tree = unsafe {
            std::mem::transmute::<
                Box<dyn FnOnce(Vec<RenderingTree>) -> RenderingTree>,
                Box<dyn FnOnce(Vec<RenderingTree>) -> RenderingTree>,
            >(Box::new(fn_rendering_tree))
        };
        self.done_inner(Some(fn_rendering_tree))
    }

    fn done_inner(&'a self, fn_rendering_tree: Option<FnRenderingTree>) -> RenderDone {
        self.tree_ctx
            .clone()
            .end_up_one_component_rendering(ComponentRenderResult {
                children: std::mem::take(&mut self.direct_children.lock().unwrap()),
                component_instance: self.instance.clone(),
                fn_rendering_tree: fn_rendering_tree,
            });

        RenderDone {
            tree_ctx: self.tree_ctx.clone(),
        }
    }

    pub(crate) fn is_sig_updated(&self, sig_id: &SigId) -> bool {
        self.tree_ctx.is_sig_updated(sig_id)
    }

    pub(crate) fn add_sig_updated(&self, sig_id: SigId) {
        self.tree_ctx.add_sig_updated(sig_id)
    }

    pub fn web_event(&'a self, web_event: impl 'a + Fn(&crate::web::WebEvent)) {
        let unsafe_casted = unsafe {
            std::mem::transmute::<
                Box<dyn Fn(&crate::web::WebEvent)>,
                Box<dyn Fn(&crate::web::WebEvent)>,
            >(Box::new(web_event))
        };
        *self.instance.web_event_listener.lock().unwrap() = Some(unsafe_casted);
    }
}
