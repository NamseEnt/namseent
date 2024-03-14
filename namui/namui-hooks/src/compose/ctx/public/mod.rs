mod event;
mod stack;

use super::*;
use crate::*;

impl<'a, 'rt> ComposeCtx<'a, 'rt> {
    pub fn compose(&self, compose: impl FnOnce(&ComposeCtx)) -> &Self {
        self.compose_with_key(
            ChildKey::IncrementalCompose {
                index: self.composer.get_next_compose_index(),
            },
            compose,
        )
    }
    pub fn compose_with_key(
        &self,
        key: impl Into<ChildKey>,
        compose: impl FnOnce(&ComposeCtx),
    ) -> &Self {
        let child_composer = self.world.get_or_create_composer(self.composer, key.into());

        let rt_container = RtContainer::new();

        let ctx = ComposeCtx::new(
            self.world,
            child_composer,
            &rt_container,
            Cow::Borrowed(self.full_stack.as_ref()),
        );
        compose(&ctx);

        self.add_rt_container(rt_container);

        self
    }
    pub fn ghost_compose(
        &self,
        key: impl Into<ChildKey>,
        compose: impl FnOnce(&ComposeCtx),
    ) -> RenderingTree {
        let child_composer = self.world.get_or_create_composer(self.composer, key.into());

        let rt_container = RtContainer::new();

        let ctx = ComposeCtx::new(
            self.world,
            child_composer,
            &rt_container,
            Cow::Borrowed(self.full_stack.as_ref()),
        );
        compose(&ctx);

        rt_container.into()
    }
    pub fn add(&self, component: impl Component) -> &Self {
        self.add_with_key(
            ChildKey::IncrementalComponent {
                index: self.composer.get_next_component_index(),
                type_name: component.static_type_name(),
            },
            component,
        )
    }
    pub fn add_with_key(&self, key: impl Into<ChildKey>, component: impl Component) -> &Self {
        let component = match component.direct_rendering_tree() {
            Ok(rendering_tree) => {
                self.add_rendering_tree(rendering_tree);
                return self;
            }
            Err(component) => component,
        };

        let (child_composer, child_instance) =
            self.world.get_or_create_instance(self.composer, key.into());

        let rendering_tree = render_ctx::run(
            self.world,
            component,
            child_composer,
            child_instance,
            Cow::Borrowed(self.full_stack.as_ref()),
        );
        self.add_rendering_tree(rendering_tree);

        self
    }
    pub fn ghost_add(&self, key: impl Into<ChildKey>, component: impl Component) -> RenderingTree {
        let component = match component.direct_rendering_tree() {
            Ok(rendering_tree) => {
                return rendering_tree;
            }
            Err(component) => component,
        };

        let (child_composer, child_instance) =
            self.world.get_or_create_instance(self.composer, key.into());

        render_ctx::run(
            self.world,
            component,
            child_composer,
            child_instance,
            Cow::Borrowed(self.full_stack.as_ref()),
        )
    }
}
