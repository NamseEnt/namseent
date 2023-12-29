mod clip_in;
mod lazy_rendering_tree;
mod nesting;

use super::*;
use crate::{
    hooks::key::{Key, KeyVec},
    *,
};
pub(crate) use lazy_rendering_tree::*;

pub struct ComposeCtx {
    tree_ctx: TreeContext,
    matrix: Matrix3x3,
    children_index: usize,
    pre_key_vec: KeyVec,
    renderer: Renderer,
    unlazy_children: Vec<RenderingTree>,
    lazy_children: Vec<Arc<Mutex<Option<LazyRenderingTree>>>>,
    lazy: Arc<Mutex<Option<LazyRenderingTree>>>,
    raw_event: RawEventContainer,
    clippings: Vec<Clipping>,
}
impl Drop for ComposeCtx {
    fn drop(&mut self) {
        let unlazy_children = std::mem::take(&mut self.unlazy_children);
        let lazy_children = std::mem::take(&mut self.lazy_children);

        let children = unlazy_children
            .into_iter()
            .map(|x| {
                Arc::new(Mutex::new(Some(LazyRenderingTree::RenderingTree {
                    rendering_tree: x,
                })))
            })
            .chain(lazy_children);

        self.lazy
            .lock()
            .unwrap()
            .replace(LazyRenderingTree::Children {
                children: children.collect(),
            });
    }
}
impl ComposeCtx {
    pub(super) fn new(
        tree_ctx: TreeContext,
        pre_key_vec: KeyVec,
        matrix: Matrix3x3,
        renderer: Renderer,
        lazy: Arc<Mutex<Option<LazyRenderingTree>>>,
        raw_event: RawEventContainer,
        clippings: Vec<Clipping>,
    ) -> Self {
        ComposeCtx {
            tree_ctx,
            matrix,
            children_index: Default::default(),
            pre_key_vec,
            renderer,
            unlazy_children: Default::default(),
            lazy_children: Default::default(),
            lazy,
            raw_event,
            clippings,
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

    fn add_lazy(&mut self, lazy: LazyRenderingTree) {
        self.lazy_children.push(Arc::new(Mutex::new(Some(lazy))));
    }

    pub fn add(&mut self, component: impl Component) -> &mut Self {
        let key_vec = self.next_child_key_vec();
        self.add_inner(key_vec, component);
        self
    }

    pub fn add_with_key(&mut self, key: impl Into<Key>, component: impl Component) -> &mut Self {
        let key_vec = self.pre_key_vec.custom_key(key);
        self.add_inner(key_vec, component);
        self
    }

    fn add_inner(&mut self, key_vec: KeyVec, component: impl Component) {
        let rendering_tree =
            self.renderer
                .render(key_vec, component, self.matrix, self.clippings.clone());
        self.lazy_children.push(Arc::new(Mutex::new(Some(
            LazyRenderingTree::RenderingTree { rendering_tree },
        ))));
    }

    pub fn compose(&mut self, compose: impl FnOnce(&mut ComposeCtx)) -> &mut Self {
        self.compose_inner(None, compose)
    }

    pub fn compose_with_key(
        &mut self,
        key: impl Into<Key>,
        compose: impl FnOnce(&mut ComposeCtx),
    ) -> &mut Self {
        self.compose_inner(Some(key.into()), compose)
    }

    fn compose_inner<IntoOptionKey: Into<Option<Key>>>(
        &mut self,
        key: IntoOptionKey,
        compose: impl FnOnce(&mut ComposeCtx),
    ) -> &mut Self {
        let rendering_tree = self.ghost_compose(key, compose);
        self.lazy_children.push(Arc::new(Mutex::new(Some(
            LazyRenderingTree::RenderingTree { rendering_tree },
        ))));

        self
    }

    /// Get RenderingTree but don't add it to the children.
    pub fn ghost_compose<IntoOptionKey: Into<Option<Key>>>(
        &mut self,
        key: IntoOptionKey,
        compose: impl FnOnce(&mut ComposeCtx),
    ) -> RenderingTree {
        let lazy: Arc<Mutex<Option<LazyRenderingTree>>> = Default::default();
        {
            let key_vec = if let Some(key) = key.into() {
                self.pre_key_vec.custom_key(key)
            } else {
                self.next_child_key_vec()
            };

            let mut child_compose_ctx = ComposeCtx::new(
                self.tree_ctx.clone(),
                key_vec,
                self.matrix,
                self.renderer.clone(),
                lazy.clone(),
                self.raw_event.clone(),
                self.clippings.clone(),
            );

            compose(&mut child_compose_ctx);
        }
        let rendering_tree = lazy.lock().unwrap().take().unwrap().into_rendering_tree();
        rendering_tree
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
        let key_vec = if let Some(key) = key.into() {
            self.pre_key_vec.custom_key(key)
        } else {
            self.next_child_key_vec()
        };

        let prev_enable_event = self.tree_ctx.enable_event_handling(enable_event_handling);

        let rendering_tree =
            self.renderer
                .render(key_vec, component, self.matrix, self.clippings.clone());

        self.tree_ctx.enable_event_handling(prev_enable_event);

        rendering_tree
    }

    pub fn add_and_get_bounding_box(&mut self, component: impl Component) -> Option<Rect<Px>> {
        let key_vec = self.next_child_key_vec();
        let rendering_tree =
            self.renderer
                .render(key_vec, component, self.matrix, self.clippings.clone());

        let bounding_box = rendering_tree.bounding_box();

        self.lazy_children.push(Arc::new(Mutex::new(Some(
            LazyRenderingTree::RenderingTree {
                rendering_tree: rendering_tree.clone(),
            },
        ))));

        bounding_box
    }
}
