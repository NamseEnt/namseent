use namui_type::*;

#[derive(Default)]
pub(crate) struct RtContainer {
    inner: elsa::FrozenVec<Box<RenderingTree>>,
}

impl RtContainer {
    pub(crate) fn new() -> RtContainer {
        RtContainer::default()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub(crate) fn push(&self, rendering_tree: Box<RenderingTree>) {
        self.inner.push(rendering_tree);
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = &RenderingTree> {
        self.inner.iter()
    }
}

impl From<RtContainer> for RenderingTree {
    fn from(rt_container: RtContainer) -> Self {
        let mut vec = rt_container.inner.into_vec();

        if vec.is_empty() {
            return RenderingTree::Empty;
        }

        if vec.len() == 1 {
            return RenderingTree::Boxed(vec.swap_remove(0));
        }

        RenderingTree::BoxedChildren(vec)
    }
}
