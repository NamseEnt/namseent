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

    pub fn effect<CleanUp: EffectCleanUp>(
        &'a self,
        title: impl AsRef<str>,
        effect: impl FnOnce() -> CleanUp,
    ) {
        handle_effect(self, title, effect)
    }

    pub fn interval(
        &'a self,
        title: impl AsRef<str>,
        duration: Duration,
        job: impl FnOnce(Duration),
    ) {
        handle_interval(self, title, duration, job)
    }

    pub fn on_raw_event(&'a self, on_raw_event: impl FnOnce(&crate::RawEvent)) {
        if let Some(raw_event) = global_state::raw_event() {
            on_raw_event(raw_event);
        }
    }

    pub fn stop_event_propagation(&'a self) {
        global_state::tree_ctx()
            .is_stop_event_propagation
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn done(&self) -> RenderDone {
        let vec: Vec<RenderingTree> = std::mem::take(self.children.lock().unwrap().as_mut());
        let rendering_tree = crate::render(vec);

        #[cfg(target_family = "wasm")]
        {
            let bounding_box = rendering_tree
                .bounding_box()
                .map(|bounding_box| self.matrix.lock().unwrap().transform_rect(bounding_box));
            *self.instance.debug_bounding_box.lock().unwrap() = bounding_box;
        }

        RenderDone { rendering_tree }
    }

    /// Get RenderingTree but don't add it to the children.
    pub fn ghost_compose(
        &self,
        compose: impl FnOnce(&mut ComposeCtx),
        GhostComposeOption {
            enable_event_handling,
        }: GhostComposeOption,
    ) -> RenderingTree {
        let lazy: LazyShared = Default::default();
        {
            let mut compose_ctx = ComposeCtx::new(
                KeyVec::new_child(self.get_next_component_index()),
                self.renderer(),
                lazy.clone(),
                global_state::no_op(),
            );

            let prev_enable_event =
                global_state::tree_ctx().enable_event_handling(enable_event_handling);

            compose(&mut compose_ctx);

            global_state::tree_ctx().enable_event_handling(prev_enable_event);
        }

        lazy.get_rendering_tree()
    }

    /// Get RenderingTree but don't add it to the children.
    pub fn ghost_component(
        &self,
        component: impl Component,
        GhostComposeOption {
            enable_event_handling,
        }: GhostComposeOption,
    ) -> RenderingTree {
        let key = KeyVec::new_child(self.get_next_component_index());

        let prev_enable_event =
            global_state::tree_ctx().enable_event_handling(enable_event_handling);

        let rendering_tree = self.render_children(key, component);

        global_state::tree_ctx().enable_event_handling(prev_enable_event);

        rendering_tree
    }

    pub fn compose(&self, compose: impl FnOnce(&mut ComposeCtx)) -> &Self {
        let rendering_tree = self.ghost_compose(
            compose,
            GhostComposeOption {
                enable_event_handling: true,
            },
        );
        self.children.lock().unwrap().push(rendering_tree);

        self
    }
    pub fn component(&self, component: impl Component) -> &Self {
        let rendering_tree = self.ghost_component(
            component,
            GhostComposeOption {
                enable_event_handling: true,
            },
        );
        self.children.lock().unwrap().push(rendering_tree);
        self
    }
    pub fn global_xy(&self, local_xy: Xy<Px>) -> Xy<Px> {
        let local_xy = TransformMatrix::from_translate(local_xy.x.as_f32(), local_xy.y.as_f32());
        let global_xy = global_state::matrix() * local_xy;
        Xy::new(global_xy.x().px(), global_xy.y().px())
    }
}

pub struct GhostComposeOption {
    pub enable_event_handling: bool,
}
