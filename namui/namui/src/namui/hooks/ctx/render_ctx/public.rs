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
        handle_state(self.inner(), init_state)
    }

    pub fn memo<T: 'static + Debug + Send + Sync>(
        &'a self,
        memo: impl 'a + FnOnce() -> T,
    ) -> Sig<'a, T> {
        handle_memo(self.inner(), memo)
    }

    pub fn track_eq<T: 'static + Debug + Send + Sync + PartialEq + Clone>(
        &'a self,
        track_eq: &T,
    ) -> Sig<'a, T> {
        handle_track_eq(self.inner(), track_eq)
    }

    pub fn effect<CleanUp: EffectCleanUp>(
        &'a self,
        title: impl AsRef<str>,
        effect: impl FnOnce() -> CleanUp,
    ) {
        handle_effect(self.inner(), title, effect)
    }

    pub fn on_raw_event(&'a self, on_raw_event: impl 'a + FnOnce(&crate::RawEvent)) {
        if let Some(raw_event) = self.inner().raw_event.as_ref() {
            on_raw_event(raw_event);
        }
    }

    pub fn stop_event_propagation(&'a self) {
        tree_ctx_mut().is_stop_event_propagation = true;
    }

    pub fn done(&self) -> RenderDone {
        self.inner().done()
    }

    /// Get RenderingTree but don't add it to the children.
    pub fn ghost_compose(
        &self,
        compose: impl FnOnce(&mut ComposeCtx),
        option: GhostComposeOption,
    ) -> RenderingTree {
        self.inner().ghost_compose(compose, option)
    }

    /// Get RenderingTree but don't add it to the children.
    pub fn ghost_component(
        &self,
        component: impl Component,
        option: GhostComposeOption,
    ) -> RenderingTree {
        self.inner().ghost_component(component, option)
    }

    pub fn compose(&self, compose: impl FnOnce(&mut ComposeCtx)) -> &Self {
        self.inner().compose(compose);

        self
    }
    pub fn component(&self, component: impl Component) -> &Self {
        self.inner().component(component);

        self
    }
    pub fn global_xy(&self, local_xy: Xy<Px>) -> Xy<Px> {
        let local_xy = Matrix3x3::from_translate(local_xy.x.as_f32(), local_xy.y.as_f32());
        let global_xy = self.inner().matrix * local_xy;
        Xy::new(global_xy.x().px(), global_xy.y().px())
    }
}

pub struct GhostComposeOption {
    pub enable_event_handling: bool,
}
