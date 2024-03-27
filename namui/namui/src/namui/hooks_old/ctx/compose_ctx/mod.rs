mod clip_in;
mod nesting;

use self::global_state::GlobalStatePop;
use super::{hook_tree::HookNodeWrapper, *};
use crate::{hooks::key::Key, *};

pub struct ComposeCtx {
    hook_node: HookNodeWrapper,
    _global_state_pop: GlobalStatePop,
    next_child_index: usize,
}

impl ComposeCtx {
    pub(super) fn new(hook_node: HookNodeWrapper, global_state_pop: GlobalStatePop) -> Self {
        ComposeCtx {
            hook_node,
            _global_state_pop: global_state_pop,
            next_child_index: 0,
        }
    }

    pub fn add(&mut self, component: impl Component) -> &mut Self {
        let rendering_tree = match component.direct_rendering_tree() {
            Ok(rendering_tree) => rendering_tree,
            Err(component) => self.render(None, component),
        };

        global_state::add_child(rendering_tree);

        self
    }

    pub fn add_with_key(&mut self, key: impl Into<Key>, component: impl Component) -> &mut Self {
        let rendering_tree = match component.direct_rendering_tree() {
            Ok(rendering_tree) => rendering_tree,
            Err(component) => self.render(Some(key.into()), component),
        };

        global_state::add_child(rendering_tree);

        self
    }

    pub fn compose(&mut self, compose: impl FnOnce(&mut ComposeCtx)) -> &mut Self {
        {
            let mut child_compose_ctx = ComposeCtx::new(
                self.get_or_create_compose_child(),
                GlobalStatePop::compose(),
            );

            compose(&mut child_compose_ctx);
        }

        self
    }

    // 이거 살려야해. 근데 이거 살릴려면 Key 구조가 애매해져.
    // Instance를 Component용, Compose용으로 쪼개거나
    // 아니면 Key를 예전처럼 가져가거나 해야할 것 같아.
    // Key가 무거운 것이 현재의 문제라서, Instance를 쪼개는게 좋을 것 같아.
    pub fn compose_with_key(
        &mut self,
        key: impl Into<Key>,
        compose: impl FnOnce(&mut ComposeCtx),
    ) -> &mut Self {
        // self.compose_inner(Some(key.into()), compose)
        todo!()
    }

    /// Get RenderingTree but don't add it to the children.
    pub fn ghost_compose<IntoOptionKey: Into<Option<Key>>>(
        &mut self,
        key: IntoOptionKey,
        compose: impl FnOnce(&mut ComposeCtx),
    ) -> RenderingTree {
        // let lazy: LazyShared = Default::default();
        // {
        //     let mut child_compose_ctx =
        //         ComposeCtx::new(key.into(), self.instance_id, GlobalStatePop::compose());

        //     compose(&mut child_compose_ctx);
        // }

        // lazy.get_rendering_tree()
        todo!()
    }

    /// Get RenderingTree but don't add it to the children.
    pub fn ghost_add<IntoOptionKey: Into<Option<Key>>>(
        &mut self,
        key: IntoOptionKey,
        component: impl Component,
        GhostComposeOption {
            enable_event_handling,
        }: GhostComposeOption,
    ) -> RenderingTree {
        let prev_enable_event =
            global_state::tree_ctx().enable_event_handling(enable_event_handling);

        let rendering_tree = self.render(key.into(), component);

        global_state::tree_ctx().enable_event_handling(prev_enable_event);

        rendering_tree
    }

    pub fn add_and_get_bounding_box(&mut self, component: impl Component) -> Option<Rect<Px>> {
        let rendering_tree = self.render(None, component);

        let bounding_box = rendering_tree.bounding_box();
        global_state::add_child(rendering_tree);

        bounding_box
    }

    fn render(&self, key: Option<Key>, component: impl Component) -> RenderingTree {
        // Renderer {
        //     instance: get_instance(self.instance_id),
        // }
        // .render(key, component)
        todo!()
    }

    fn get_or_create_compose_child(&mut self) -> HookNodeWrapper {
        let index = self.get_next_child_index();
        self.hook_node
            .get_or_create_child_node(Key::IncrementalCompose { index }, || {
                hook_tree::HookInstance::new(hook_tree::HookType::Compose)
            })
    }

    fn get_next_child_index(&mut self) -> usize {
        let index = self.next_child_index;
        self.next_child_index += 1;
        index
    }
}
