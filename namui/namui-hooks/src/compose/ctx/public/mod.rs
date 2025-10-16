mod event;
mod stack;

use super::*;
use crate::*;
use std::sync::atomic::Ordering;

impl ComposeCtx<'_, '_> {
    pub fn compose(&self, compose: impl FnOnce(ComposeCtx)) -> &Self {
        self.compose_with_key(None, compose)
    }
    pub fn compose_2(&self, title: &str, compose: impl FnOnce(ComposeCtx)) -> &Self {
        let _ = title;
        self.compose_with_key(None, compose)
    }
    pub fn compose_with_key(
        &self,
        key: impl Into<AddKey>,
        compose: impl FnOnce(ComposeCtx),
    ) -> &Self {
        let rt_container = self.ghost_impl(key, compose);
        self.add_rt_container(rt_container);

        self
    }
    pub fn ghost_compose(
        &self,
        key: impl Into<AddKey>,
        compose: impl FnOnce(ComposeCtx),
    ) -> RenderingTree {
        self.ghost_impl(key, compose).into()
    }
    fn ghost_impl(&self, key: impl Into<AddKey>, compose: impl FnOnce(ComposeCtx)) -> RtContainer {
        let child_key = match key.into() {
            AddKey::String(key) => ChildKey::String(key),
            AddKey::U128(uuid) => ChildKey::U128(uuid),
            AddKey::Incremental => ChildKey::IncrementalCompose {
                index: self.composer.get_next_compose_index(),
            },
        };
        let child_composer = self.world.get_or_create_composer(self.composer, child_key);

        let rt_container = RtContainer::new();

        let ctx = ComposeCtx::new(
            self.world,
            child_composer,
            &rt_container,
            Cow::Borrowed(self.full_stack.as_ref()),
        );
        compose(ctx);

        rt_container
    }
    pub fn add(&self, component: impl Component) -> &Self {
        self.add_with_key(None, component)
    }
    pub fn add_with_key(&self, key: impl Into<AddKey>, component: impl Component) -> &Self {
        let rendering_tree = self.ghost_add(key, component);
        self.add_rendering_tree(rendering_tree);

        self
    }
    pub fn ghost_add(&self, key: impl Into<AddKey>, component: impl Component) -> RenderingTree {
        let component = match component.direct_rendering_tree() {
            Ok(rendering_tree) => {
                return rendering_tree;
            }
            Err(component) => component,
        };

        let child_key = match key.into() {
            AddKey::String(key) => ChildKey::String(key),
            AddKey::U128(uuid) => ChildKey::U128(uuid),
            AddKey::Incremental => ChildKey::IncrementalComponent {
                index: self.composer.get_next_component_index(),
                type_name: std::any::type_name_of_val(&component).to_string(),
            },
        };

        let (child_composer, child_instance) =
            self.world.get_or_create_instance(self.composer, child_key);

        render_ctx::run(
            self.world,
            component,
            child_composer,
            child_instance,
            Cow::Borrowed(self.full_stack.as_ref()),
        )
    }
    pub fn set_event_propagation(&self, propagate: bool) -> bool {
        self.world
            .is_stop_event_propagation
            .swap(!propagate, Ordering::Relaxed)
    }
}
