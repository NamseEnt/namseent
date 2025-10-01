use crate::*;

#[derive(Default)]
pub(crate) struct RtContainer {
    inner: boxcar::Vec<RenderingTree>,
}

impl RtContainer {
    pub(crate) fn new() -> RtContainer {
        Default::default()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub(crate) fn push(&self, rendering_tree: RenderingTree) {
        self.inner.push(rendering_tree);
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = &RenderingTree> {
        self.inner.iter().map(|(_, x)| x)
    }
}

impl From<RtContainer> for RenderingTree {
    fn from(rt_container: RtContainer) -> Self {
        let mut vec = rt_container.inner.into_iter().collect::<Vec<_>>();

        if vec.is_empty() {
            return RenderingTree::Empty;
        }

        if vec.len() == 1 {
            return vec.swap_remove(0);
        }

        RenderingTree::Children(vec)
    }
}
