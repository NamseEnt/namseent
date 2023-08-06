use super::*;
use crate::{Matrix3x3, RenderingTree};
use namui_type::Px;

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
    component_index: AtomicUsize,
}

impl<'a> RenderCtx {
    pub(crate) fn new(
        instance: Arc<ComponentInstance>,
        updated_sigs: HashSet<SigId>,
        tree_ctx: Arc<TreeContext>,
        matrix: Matrix3x3,
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
            matrix: Mutex::new(matrix),
            force_render_index: Default::default(),
            component_index: Default::default(),
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
        let rendering_tree = self.render_children(key, component);
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

    // pub fn return_(&self, component: impl Component) -> RenderDone {
    //     self.add("".to_string(), component);
    //     self.return_internal()
    // }

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
        self.render_children(key, component)
    }

    fn render_children(&self, key: String, component: impl Component) -> RenderingTree {
        let child_instance = self.instance.get_or_create_child_instance(key, &component);
        self.tree_ctx.render(
            component,
            child_instance,
            self.updated_sigs.lock().unwrap().clone(),
            self.matrix.lock().unwrap().clone(),
        )
    }

    pub fn component(&self, component: impl Component) -> &Self {
        let index = self
            .component_index
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let key = format!("component_{index}");
        self.add(key, component);
        self
    }

    pub fn done(&self) -> RenderDone {
        todo!()
    }

    pub fn component_group(&self, func: impl FnOnce(GroupCtx)) -> &Self {
        todo!()
    }

    pub fn component_branch(&self, func: impl FnOnce(BranchCtx)) -> &Self {
        todo!()
    }

    pub fn on_top(&self) -> MatrixCtx {
        todo!()
    }

    pub fn absolute(&self, x: namui_type::Px, y: namui_type::Px) -> MatrixCtx {
        todo!()
    }

    pub fn as_compose(&self) -> ComposeCtx {
        todo!()
    }

    pub fn compose(&self, ctx: impl FnOnce(ComposeCtx)) -> &Self {
        todo!()
    }
}

pub struct GroupCtx {}

impl GroupCtx {
    pub fn add(&self, key: impl AsRef<str>, component: impl Component) {
        todo!()
    }

    pub fn translate(self, x: namui_type::Px, y: namui_type::Px) -> Self {
        todo!()
    }

    pub fn clip(self, height: crate::PathBuilder, intersect: crate::ClipOp) -> Self {
        todo!()
    }

    pub fn as_branch(self) -> BranchCtx {
        todo!()
    }
}

pub struct BranchCtx {}

impl BranchCtx {
    pub fn add(self, component: impl Component) {
        todo!()
    }
}

pub struct MatrixCtx {}

impl MatrixCtx {
    pub fn on_top(self) -> Self {
        todo!()
    }

    pub fn absolute(self, x: namui_type::Px, y: namui_type::Px) -> Self {
        todo!()
    }

    pub fn component(self, component: impl Component) -> Self {
        todo!()
    }

    pub fn component_group(&self, func: impl FnOnce(GroupCtx)) -> Self {
        todo!()
    }

    pub fn attach_event(&self, attach_event: impl Fn(Event<'_>)) {
        todo!()
    }
}

pub struct ComposeCtx {}
impl ComposeCtx {
    pub fn translate(&self, x: Px, y: Px) -> Self {
        todo!()
    }

    pub fn clip(&self, height: crate::PathBuilder, intersect: crate::ClipOp) -> Self {
        todo!()
    }

    pub fn group_by(&self, key: String) -> Self {
        todo!()
    }

    pub fn ghost_render(&self, component: impl Component) -> RenderingTree {
        todo!()
    }

    pub fn add(&self, component: impl Component) -> &Self {
        todo!()
    }
}
