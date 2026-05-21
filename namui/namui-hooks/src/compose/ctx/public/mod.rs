mod event;
mod stack;

use super::*;
use crate::*;
use std::sync::atomic::Ordering;

impl<'a, 'rt> ComposeCtx<'a, 'rt> {
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
        self.ghost_impl(key, compose).into_rendering_tree()
    }
    fn ghost_impl(
        &self,
        key: impl Into<AddKey>,
        compose: impl FnOnce(ComposeCtx),
    ) -> RtContainer<'a> {
        let child_key = match key.into() {
            AddKey::String(key) => ChildKey::string(key),
            AddKey::U128(uuid) => ChildKey::u128(uuid),
            AddKey::Incremental => {
                ChildKey::incremental_compose(self.composer.get_next_compose_index())
            }
        };
        let child_composer = self.world.get_or_create_composer(self.composer, child_key);

        let rt_container = RtContainer::new(self.world);

        let ctx = ComposeCtx::new(self.world, child_composer, &rt_container, self.full_stack);
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
            AddKey::String(key) => ChildKey::string(key),
            AddKey::U128(uuid) => ChildKey::u128(uuid),
            AddKey::Incremental => ChildKey::incremental_component(
                self.composer.get_next_component_index(),
                std::any::type_name_of_val(&component),
            ),
        };

        let (child_composer, child_instance) =
            self.world.get_or_create_instance(self.composer, child_key);

        render_ctx::run(
            self.world,
            component,
            child_composer,
            child_instance,
            self.full_stack,
        )
    }
    pub fn set_event_propagation(&self, propagate: bool) -> bool {
        self.world
            .is_stop_event_propagation
            .swap(!propagate, Ordering::Relaxed)
    }
}
