use crate::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Default, Clone)]
pub(crate) struct LazyShared {
    inner: Rc<RefCell<Option<LazyRenderingTree>>>,
}

impl LazyShared {
    pub(crate) fn new(lazy: LazyRenderingTree) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Some(lazy))),
        }
    }
    pub(crate) fn get_rendering_tree(&self) -> RenderingTree {
        let Some(inner) = self.inner.borrow_mut().take() else {
            return RenderingTree::Empty;
        };

        inner.into_rendering_tree()
    }

    pub(crate) fn set(&self, lazy: LazyRenderingTree) {
        *self.inner.borrow_mut() = Some(lazy);
    }
}

#[derive(Debug)]
pub(crate) enum LazyRenderingTree {
    Translate {
        xy: Xy<Px>,
        lazy: LazyShared,
    },
    Absolute {
        xy: Xy<Px>,
        lazy: LazyShared,
    },
    Clip {
        path: crate::Path,
        clip_op: crate::ClipOp,
        lazy: LazyShared,
    },
    OnTop {
        lazy: LazyShared,
    },
    Rotate {
        angle: Angle,
        lazy: LazyShared,
    },
    Scale {
        scale_xy: Xy<f32>,
        lazy: LazyShared,
    },
    Children {
        children: Vec<LazyShared>,
    },
    RenderingTree {
        rendering_tree: RenderingTree,
    },
}
impl LazyRenderingTree {
    pub(crate) fn into_rendering_tree(self) -> RenderingTree {
        match self {
            LazyRenderingTree::Translate { xy, lazy } => {
                let rendering_tree = lazy.get_rendering_tree();
                crate::translate(xy.x, xy.y, rendering_tree)
            }
            LazyRenderingTree::Absolute { xy, lazy } => {
                let rendering_tree = lazy.get_rendering_tree();
                crate::absolute(xy.x, xy.y, rendering_tree)
            }
            LazyRenderingTree::Clip {
                path,
                clip_op,
                lazy,
            } => {
                let rendering_tree = lazy.get_rendering_tree();
                RenderingTree::Special(SpecialRenderingNode::Clip(ClipNode {
                    path,
                    clip_op,
                    rendering_tree: rendering_tree.into(),
                }))
            }
            LazyRenderingTree::OnTop { lazy } => {
                let rendering_tree = lazy.get_rendering_tree();
                crate::on_top(rendering_tree)
            }
            LazyRenderingTree::Rotate { angle, lazy } => {
                let rendering_tree = lazy.get_rendering_tree();
                crate::rotate(angle, rendering_tree)
            }
            LazyRenderingTree::Scale { scale_xy, lazy } => {
                let rendering_tree = lazy.get_rendering_tree();
                crate::scale(scale_xy.x, scale_xy.y, rendering_tree)
            }
            LazyRenderingTree::Children { children } => {
                crate::render(children.into_iter().map(|child| child.get_rendering_tree()))
            }
            LazyRenderingTree::RenderingTree { rendering_tree } => rendering_tree,
        }
    }
}
