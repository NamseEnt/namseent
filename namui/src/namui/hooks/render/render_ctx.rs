use super::*;
use crate::{Matrix3x3, RenderingTree};

pub struct RenderCtx {
    pub(crate) instance: Arc<ComponentInstance>,
    pub(crate) state_index: AtomicUsize,
    pub(crate) effect_index: AtomicUsize,
    pub(crate) memo_index: AtomicUsize,
    pub(crate) track_eq_index: AtomicUsize,
    pub(crate) updated_sigs: Mutex<HashSet<SigId>>,
    tree_ctx: Arc<TreeContext>,
    children: Arc<Mutex<Vec<RenderingTree>>>,
    pub(crate) matrix: Mutex<Matrix3x3>,
    force_render_index: AtomicUsize,
}

impl<'a> RenderCtx {
    pub(crate) fn new(
        instance: Arc<ComponentInstance>,
        updated_sigs: HashSet<SigId>,
        tree_ctx: Arc<TreeContext>,
    ) -> Self {
        Self {
            instance,
            state_index: Default::default(),
            effect_index: Default::default(),
            memo_index: Default::default(),
            track_eq_index: Default::default(),
            updated_sigs: Mutex::new(updated_sigs),
            tree_ctx,
            children: Default::default(),
            matrix: Default::default(),
            force_render_index: Default::default(),
        }
    }

    pub fn atom_init<T: Debug + Send + Sync + 'static>(
        &'a self,
        atom: &'static Atom<T>,
        init: impl 'a + FnOnce() -> T,
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
        memo: impl 'a + FnOnce() -> T,
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

    pub(crate) fn add(&'a self, key: String, component: impl Component) {
        let rendering_tree = self.render(key, component);
        self.push_rendering_tree(rendering_tree);
    }

    pub(crate) fn push_rendering_tree(&'a self, rendering_tree: RenderingTree) {
        self.children.lock().unwrap().push(rendering_tree);
    }

    pub(crate) fn is_sig_updated(&self, sig_id: &SigId) -> bool {
        self.updated_sigs.lock().unwrap().contains(sig_id)
    }

    pub(crate) fn add_sig_updated(&self, sig_id: SigId) {
        self.updated_sigs.lock().unwrap().insert(sig_id);
    }

    pub fn web_event(&'a self, web_event: impl 'a + FnOnce(&crate::web::WebEvent)) {
        if let RenderEvent::WebEvent { web_event: event } = get_render_event().as_ref() {
            web_event(event);
        }
    }

    pub fn arc<T: 'a>(&'a self, value: T) -> Arc<T> {
        Arc::new(value)
    }

    pub fn return_(&self, component: impl Component) -> RenderDone {
        self.add("".to_string(), component);
        self.return_internal()
    }

    pub fn return_no(&self) -> RenderDone {
        RenderDone {
            rendering_tree: RenderingTree::Empty,
        }
    }

    pub(crate) fn return_internal(&self) -> RenderDone {
        RenderDone {
            rendering_tree: RenderingTree::Children(std::mem::take(
                &mut self.children.lock().unwrap(),
            )),
        }
    }

    pub fn ghost_render(&self, component: impl Component) -> RenderingTree {
        let index = self
            .force_render_index
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let key = format!("force_render_{index}");
        self.render(key, component)
    }

    fn render(&self, key: String, component: impl Component) -> RenderingTree {
        let child_instance = self.instance.get_or_create_child_instance(key, &component);
        self.tree_ctx.render(
            component,
            child_instance,
            self.updated_sigs.lock().unwrap().clone(),
        )
    }
}

// #[derive(Clone)]
// pub struct MatrixCtx<'a> {
//     pub(crate) ctx: &'a RenderCtx,
//     pub(crate) matrix: Matrix3x3,
// }

// impl<'a> MatrixCtx<'a> {
//     pub fn translate(&mut self, xy: Xy<Px>) -> &mut Self {
//         self.matrix.translate(xy.x.as_f32(), xy.y.as_f32());
//         self
//     }

//     pub fn branch(&mut self, branch: impl FnOnce(&mut Self)) -> &mut Self {
//         branch(&mut self.clone());
//         self
//     }

//     pub fn add(&mut self, add: impl Component) -> AddingCtx {
//         let mut ctx = self.create_adding_ctx();
//         ctx.add(add);
//         ctx
//     }

//     pub fn add_with_instance(
//         &self,
//         component: impl Component,
//         instance: Arc<ComponentInstance>,
//     ) -> AddingCtx {
//         let mut ctx = self.create_adding_ctx();
//         ctx.add_with_instance(component, instance);
//         ctx
//     }

//     fn create_adding_ctx(&self) -> AddingCtx {
//         AddingCtx::new(
//             self.ctx.tree_ctx.clone(),
//             self.ctx.children.clone(),
//             self.matrix,
//         )
//     }
// }
