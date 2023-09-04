mod clip_in;
mod lazy_rendering_tree;
mod nesting;

use super::*;
use crate::{
    hooks::key::{Key, KeyVec},
    *,
};
pub(crate) use lazy_rendering_tree::*;
pub use nesting::*;

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
    pub fn ghost_render<IntoKey: Into<Key>>(
        &mut self,
        key: Option<IntoKey>,
        component_type_name: &'static str,
        func: impl FnOnce(&RenderCtx) -> RenderDone,
    ) -> RenderingTree {
        let key_vec = if let Some(key) = key {
            self.pre_key_vec.custom_key(key)
        } else {
            self.next_child_key_vec()
        };
        let ctx = self.renderer.spawn_render_ctx(
            key_vec,
            component_type_name,
            self.matrix,
            self.clippings.clone(),
        );
        ctx.disable_event_handling();
        let done = func(&ctx);
        done.rendering_tree
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
        let key_vec = self.next_child_key_vec();
        self.compose_inner(key_vec, compose)
    }

    pub fn compose_with_key(
        &mut self,
        key: impl Into<Key>,
        compose: impl FnOnce(&mut ComposeCtx),
    ) -> &mut Self {
        let key_vec = self.pre_key_vec.custom_key(key);
        self.compose_inner(key_vec, compose)
    }

    fn compose_inner(
        &mut self,
        key_vec: KeyVec,
        compose: impl FnOnce(&mut ComposeCtx),
    ) -> &mut Self {
        let lazy: Arc<Mutex<Option<LazyRenderingTree>>> = Default::default();
        {
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
        self.lazy_children.push(Arc::new(Mutex::new(Some(
            LazyRenderingTree::RenderingTree { rendering_tree },
        ))));

        self
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
