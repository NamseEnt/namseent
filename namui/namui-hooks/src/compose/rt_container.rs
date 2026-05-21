use crate::*;
use std::cell::RefCell;

pub(crate) struct RtContainer<'a> {
    inner: RefCell<Vec<RenderingTree>>,
    world: &'a World,
}

impl<'a> RtContainer<'a> {
    pub(crate) fn new(world: &'a World) -> RtContainer<'a> {
        RtContainer {
            inner: RefCell::new(world.take_rt_vec()),
            world,
        }
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.inner.borrow().is_empty()
    }
    pub(crate) fn push(&self, rendering_tree: RenderingTree) {
        self.inner.borrow_mut().push(rendering_tree);
    }
    pub(crate) fn with<R>(&self, f: impl FnOnce(&[RenderingTree]) -> R) -> R {
        f(&self.inner.borrow())
    }
    pub(crate) fn into_rendering_tree(self) -> RenderingTree {
        let mut vec = self.inner.into_inner();

        match vec.len() {
            0 => {
                self.world.recycle_rt_vec(vec);
                RenderingTree::Empty
            }
            1 => {
                let only = vec.swap_remove(0);
                self.world.recycle_rt_vec(vec);
                only
            }
            _ => {
                let children = arena_alloc_slice(vec.drain(..));
                self.world.recycle_rt_vec(vec);
                RenderingTree::Children(children)
            }
        }
    }
}
