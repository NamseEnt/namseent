use super::*;

impl<'a> RenderCtx {
    pub fn atom_init<T: Debug + Send + Sync + 'static>(
        &'a self,
        atom: &'static Atom<T>,
        atom_init: impl 'a + FnOnce() -> T,
    ) -> (Sig<'a, T>, SetState<T>) {
        handle_atom_init(atom, atom_init)
    }

    pub fn atom<T: Debug + Send + Sync + 'static>(
        &'a self,
        atom: &'static Atom<T>,
    ) -> (Sig<'a, T>, SetState<T>) {
        handle_atom(atom)
    }

    pub fn state<T: 'static + Debug + Send + Sync>(
        &'a self,
        init_state: impl FnOnce() -> T,
    ) -> (Sig<'a, T>, SetState<T>) {
        handle_state(self, init_state)
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

    pub fn effect<CleanUp: EffectCleanUp>(
        &'a self,
        title: impl AsRef<str>,
        effect: impl FnOnce() -> CleanUp,
    ) {
        handle_effect(self, title, effect)
    }

    pub fn on_raw_event(&'a self, on_raw_event: impl 'a + FnOnce(&crate::RawEvent)) {
        if let Some(raw_event) = self.raw_event.lock().unwrap().clone() {
            on_raw_event(raw_event.as_ref());
        }
    }

    pub fn ghost_render(&self, component: impl Component) -> RenderingTree {
        self.disable_event_handling();
        let rendering_tree = self.render_children(
            KeyVec::new_child(self.get_next_component_index()),
            component,
        );
        self.enable_event_handling();
        rendering_tree
    }

    pub fn done(&self) -> RenderDone {
        let vec: Vec<RenderingTree> = std::mem::take(self.children.lock().unwrap().as_mut());
        let rendering_tree = crate::render(vec);

        let bounding_box = rendering_tree
            .bounding_box()
            .map(|bounding_box| self.matrix.lock().unwrap().transform_rect(bounding_box));
        *self.instance.debug_bounding_box.lock().unwrap() = bounding_box;

        RenderDone { rendering_tree }
    }

    pub fn ghost_compose(&self, compose: impl FnOnce(&mut ComposeCtx)) -> RenderingTree {
        self.disable_event_handling();
        let rendering_tree = self.compose_inner(compose);
        self.enable_event_handling();
        rendering_tree
    }
    pub fn component(&self, component: impl Component) -> &Self {
        self.add(
            KeyVec::new_child(self.get_next_component_index()),
            component,
        );
        self
    }
    pub fn compose(&self, compose: impl FnOnce(&mut ComposeCtx)) -> &Self {
        let rendering_tree = self.compose_inner(compose);
        self.children.lock().unwrap().push(rendering_tree);

        self
    }
}
