use super::*;
use crate::{AsXyPx, Matrix3x3, RenderingTree};

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
    component_index: AtomicUsize,
}

impl Drop for RenderCtx {
    fn drop(&mut self) {
        self.instance.after_render();
    }
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
        let mut vec = vec![];
        std::mem::swap(self.children.lock().unwrap().as_mut(), &mut vec);

        RenderDone {
            rendering_tree: crate::render(vec),
        }
    }

    pub fn ghost_render(&self, component: impl Component) -> RenderingTree {
        // TODO: Prevent event handling for ghost render
        self.render_children(
            KeyVec::new_child(self.get_next_component_index()),
            component,
        )
    }

    fn renderer(&self) -> Renderer {
        Renderer {
            instance: self.instance.clone(),
            updated_sigs: self.updated_sigs.lock().unwrap().clone(),
            tree_ctx: self.tree_ctx.clone(),
        }
    }

    fn render_children(&self, key_vec: KeyVec, component: impl Component) -> RenderingTree {
        self.renderer()
            .render(key_vec, component, self.matrix.lock().unwrap().clone())
    }

    fn get_next_component_index(&self) -> usize {
        self.component_index
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn done(&self) -> RenderDone {
        self.return_internal()
    }

    pub fn ghost_render_with_ctx<Func: FnOnce(&mut ComposeCtx)>(
        &self,
        content: Func,
    ) -> RenderingTree {
        todo!()
    }
    pub(crate) fn add(&'a self, key: KeyVec, component: impl Component) {
        let rendering_tree = self.render_children(key, component);
        self.children.lock().unwrap().push(rendering_tree);
    }
}

impl RenderCtx {
    pub fn component(&self, component: impl Component) -> &Self {
        self.add(
            KeyVec::new_child(self.get_next_component_index()),
            component,
        );
        self
    }
    pub fn compose(&self, compose: impl FnOnce(&mut ComposeCtx)) -> &Self {
        crate::log!(
            "compose before len: {}",
            self.children.lock().unwrap().len()
        );
        let mut compose_ctx = ComposeCtx::new(
            KeyVec::new_child(self.get_next_component_index()),
            self.matrix.lock().unwrap().clone(),
            self.renderer(),
            self.children.clone(),
        );

        compose(&mut compose_ctx);
        drop(compose_ctx);
        crate::log!(
            "compose after len: {}\n\n",
            self.children.lock().unwrap().len()
        );

        self
    }
}

#[derive(Clone)]
struct Renderer {
    instance: Arc<ComponentInstance>,
    updated_sigs: HashSet<SigId>,
    tree_ctx: Arc<TreeContext>,
}

impl Renderer {
    fn render(
        &self,
        key_vec: KeyVec,
        component: impl Component,
        matrix: Matrix3x3,
    ) -> RenderingTree {
        let child_instance = self
            .instance
            .get_or_create_child_instance(key_vec, &component);
        self.tree_ctx.render(
            component,
            child_instance.clone(),
            self.updated_sigs.clone(),
            matrix,
        )
    }
}

pub struct ComposeCtx {
    matrix: Matrix3x3,
    children_index: usize,
    pre_key_vec: KeyVec,
    renderer: Renderer,
    children: Arc<Mutex<Vec<RenderingTree>>>,
    push_on_drop: Arc<Mutex<Vec<RenderingTree>>>,
}
impl Drop for ComposeCtx {
    fn drop(&mut self) {
        let mut children = vec![];
        std::mem::swap(self.children.lock().unwrap().as_mut(), &mut children);

        let rendering_tree = {
            if children.len() == 0 {
                crate::log!("ComposeCtx Drop No Len");
                return;
            } else if children.len() == 1 {
                children.remove(0)
            } else {
                RenderingTree::Children(children)
            }
        };

        self.push_on_drop.lock().unwrap().push(rendering_tree);
    }
}
impl ComposeCtx {
    fn new(
        pre_key_vec: KeyVec,
        matrix: Matrix3x3,
        renderer: Renderer,
        push_on_drop: Arc<Mutex<Vec<RenderingTree>>>,
    ) -> Self {
        ComposeCtx {
            matrix,
            children_index: Default::default(),
            pre_key_vec,
            renderer,
            children: Default::default(),
            push_on_drop,
        }
    }
    fn next_children_index(&mut self) -> usize {
        let index = self.children_index;
        self.children_index += 1;
        index
    }
    fn next_child_key_vec(&mut self) -> KeyVec {
        let index = self.next_children_index();
        self.pre_key_vec.child(index)
    }

    pub fn debug(&self) {
        // crate::log!(
        //     "Debug ComposeCtx: children: {:#?}",
        //     self.children.lock().unwrap()
        // );
    }
}
// Nesting
impl ComposeCtx {
    pub fn translate(&mut self, xy: impl AsXyPx) -> Self {
        let xy = xy.as_xy_px();
        let matrix = self.matrix * Matrix3x3::from_translate(xy.x.as_f32(), xy.y.as_f32());
        ComposeCtx::new(
            self.next_child_key_vec(),
            matrix,
            self.renderer.clone(),
            self.children.clone(),
        )
    }
    pub fn absolute(&mut self, xy: impl AsXyPx) -> Self {
        let xy = xy.as_xy_px();
        let matrix = Matrix3x3::from_translate(xy.x.as_f32(), xy.y.as_f32());
        ComposeCtx::new(
            self.next_child_key_vec(),
            matrix,
            self.renderer.clone(),
            self.children.clone(),
        )
    }
    pub fn clip(&mut self, height: crate::PathBuilder, intersect: crate::ClipOp) -> Self {
        // TODO: Cliping

        ComposeCtx::new(
            self.next_child_key_vec(),
            self.matrix,
            self.renderer.clone(),
            self.children.clone(),
        )
    }
    pub fn group_by(&mut self, key: impl Into<Key>) -> Self {
        ComposeCtx::new(
            self.pre_key_vec.group(key),
            self.matrix,
            self.renderer.clone(),
            self.children.clone(),
        )
    }
    pub fn on_top(&mut self) -> Self {
        let matrix = Matrix3x3::identity();
        ComposeCtx::new(
            self.next_child_key_vec(),
            matrix,
            self.renderer.clone(),
            self.children.clone(),
        )
    }
}
impl ComposeCtx {
    pub fn ghost_render(&self, component: impl Component) -> RenderingTree {
        todo!()
    }

    pub fn add(&mut self, component: impl Component) -> &mut Self {
        let key_vec = self.next_child_key_vec();
        let rendering_tree = self.renderer.render(key_vec, component, self.matrix);
        self.children.lock().unwrap().push(rendering_tree);
        self
    }

    pub fn add_with_key(&mut self, key: impl Into<Key>, component: impl Component) -> &mut Self {
        let key_vec = self.pre_key_vec.custom_key(key);
        let rendering_tree = self.renderer.render(key_vec, component, self.matrix);
        self.children.lock().unwrap().push(rendering_tree);
        self
    }

    pub fn compose(&mut self, compose: impl FnOnce(&mut ComposeCtx)) -> &mut Self {
        let key_vec = self.next_child_key_vec();
        let mut child_compose_ctx = ComposeCtx::new(
            key_vec,
            self.matrix,
            self.renderer.clone(),
            self.children.clone(),
        );
        compose(&mut child_compose_ctx);
        drop(child_compose_ctx);

        self
    }

    pub fn compose_with_key(
        &mut self,
        key: impl Into<Key>,
        compose: impl FnOnce(&mut ComposeCtx),
    ) -> &mut Self {
        let key_vec = self.pre_key_vec.custom_key(key);
        let mut child_compose_ctx = ComposeCtx::new(
            key_vec,
            self.matrix,
            self.renderer.clone(),
            self.children.clone(),
        );
        compose(&mut child_compose_ctx);
        drop(child_compose_ctx);

        self
    }

    pub fn attach_event(&self, attach_event: impl Fn(Event<'_>)) -> &Self {
        todo!()
    }
}
