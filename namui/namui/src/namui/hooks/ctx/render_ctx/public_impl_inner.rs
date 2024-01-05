use super::*;
use std::cell::OnceCell;

impl RenderCtxInner {
    pub fn done(&mut self) -> RenderDone {
        let children = std::mem::take(&mut self.children);
        let rendering_tree = crate::render(children);
        self.rendered = Some(rendering_tree);

        // NOTE: inspect is disabled yet
        // let bounding_box = rendering_tree
        //     .bounding_box()
        //     .map(|bounding_box| self.matrix.transform_rect(bounding_box));
        // self.instance.set_debug_bounding_box(bounding_box);

        RenderDone { _private: () }
    }

    /// Get RenderingTree but don't add it to the children.
    pub fn ghost_compose(
        &mut self,
        compose: impl FnOnce(&mut ComposeCtx),
        GhostComposeOption {
            enable_event_handling,
        }: GhostComposeOption,
    ) -> RenderingTree {
        let lazy: Rc<OnceCell<LazyRenderingTree>> = Default::default();
        {
            let mut compose_ctx = ComposeCtx::new(
                KeyVec::new_child(self.get_next_component_index()),
                self.matrix,
                self.renderer(),
                lazy.clone(),
                self.raw_event,
                self.clippings.clone(),
            );

            let prev_enable_event =
                tree_ctx_mut().swap_enable_event_handling(enable_event_handling);

            compose(&mut compose_ctx);

            tree_ctx_mut().swap_enable_event_handling(prev_enable_event);
        }

        Rc::into_inner(lazy)
            .unwrap()
            .take()
            .unwrap()
            .into_rendering_tree()
    }

    /// Get RenderingTree but don't add it to the children.
    pub fn ghost_component(
        &mut self,
        component: impl Component,
        GhostComposeOption {
            enable_event_handling,
        }: GhostComposeOption,
    ) -> RenderingTree {
        let key = KeyVec::new_child(self.get_next_component_index());

        let prev_enable_event = tree_ctx_mut().swap_enable_event_handling(enable_event_handling);

        let rendering_tree = self.render_children(key, component);

        tree_ctx_mut().swap_enable_event_handling(prev_enable_event);

        rendering_tree
    }

    pub fn compose(&mut self, compose: impl FnOnce(&mut ComposeCtx)) -> &Self {
        let rendering_tree = self.ghost_compose(
            compose,
            GhostComposeOption {
                enable_event_handling: true,
            },
        );
        self.children.push(rendering_tree);

        self
    }
    pub fn component(&mut self, component: impl Component) -> &Self {
        let rendering_tree = self.ghost_component(
            component,
            GhostComposeOption {
                enable_event_handling: true,
            },
        );
        self.children.push(rendering_tree);

        self
    }
}
