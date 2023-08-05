use super::*;
use crate::{Matrix3x3, RenderingTree};
use namui_type::*;

pub struct RenderCtx {
    pub(crate) instance: Arc<ComponentInstance>,
    pub(crate) state_index: AtomicUsize,
    pub(crate) effect_index: AtomicUsize,
    pub(crate) memo_index: AtomicUsize,
    pub(crate) track_eq_index: AtomicUsize,
    pub(crate) updated_sigs: Mutex<HashSet<SigId>>,
    tree_ctx: Arc<TreeContext>,
    children: Arc<Mutex<Vec<RenderingTree>>>,
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

    pub fn add(
        &'a self,
        add: impl Component, // Name 'add' is to prevent showing 'child' text on rust-analyzer with vscode
    ) -> AddingCtx {
        let matrix = Matrix3x3::identity(); // TODO
        let mut ctx = AddingCtx::new(self.tree_ctx.clone(), self.children.clone(), matrix);
        ctx.add(add);
        ctx
    }

    pub(crate) fn add_rendering_tree(&'a self, rendering_tree: RenderingTree) {
        self.children.lock().unwrap().push(rendering_tree);
    }

    pub fn try_add<C: Component>(&'a self, try_add: Option<C>) -> &'a Self {
        if let Some(component) = try_add {
            self.add(component);
        }
        self
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

    pub(crate) fn done(self) -> RenderDone {
        RenderDone {
            rendering_tree: RenderingTree::Children(
                Arc::into_inner(self.children)
                    .unwrap()
                    .into_inner()
                    .unwrap(),
            ),
        }
    }

    pub fn test_bounding_box(
        &self,
        component: impl Component,
    ) -> (Option<Rect<Px>>, Arc<ComponentInstance>) {
        todo!()
    }

    pub fn translate(&self, xy: Xy<Px>) -> MatrixCtx {
        MatrixCtx {
            ctx: self,
            matrix: Matrix3x3::from_translate(xy.x.as_f32(), xy.y.as_f32()),
        }
    }

    pub fn later_once(&'a self, later_once: impl 'a + FnOnce(&Self)) -> impl 'a + Component {
        RenderBox::new(later_once)
    }

    pub fn clip(&self, path: crate::PathBuilder, clip_op: crate::ClipOp) -> MatrixCtx {
        todo!()
    }
}

#[derive(Clone)]
pub struct MatrixCtx<'a> {
    pub(crate) ctx: &'a RenderCtx,
    pub(crate) matrix: Matrix3x3,
}

impl<'a> MatrixCtx<'a> {
    pub fn translate(&mut self, xy: Xy<Px>) -> &mut Self {
        self.matrix.translate(xy.x.as_f32(), xy.y.as_f32());
        self
    }

    pub fn branch(&mut self, branch: impl FnOnce(&mut Self)) -> &mut Self {
        branch(&mut self.clone());
        self
    }

    pub fn add(&mut self, add: impl Component) -> AddingCtx {
        let mut ctx = self.create_adding_ctx();
        ctx.add(add);
        ctx
    }

    pub fn add_with_instance(
        &self,
        component: impl Component,
        instance: Arc<ComponentInstance>,
    ) -> AddingCtx {
        let mut ctx = self.create_adding_ctx();
        ctx.add_with_instance(component, instance);
        ctx
    }

    fn create_adding_ctx(&self) -> AddingCtx {
        AddingCtx::new(
            self.ctx.tree_ctx.clone(),
            self.ctx.children.clone(),
            self.matrix,
        )
    }
}
