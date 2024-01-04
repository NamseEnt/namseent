use crate::*;
use std::{cell::OnceCell, rc::Rc};

#[derive(Debug)]
pub(crate) enum LazyRenderingTree {
    Translate {
        xy: Xy<Px>,
        lazy: Rc<OnceCell<LazyRenderingTree>>,
    },
    Absolute {
        xy: Xy<Px>,
        lazy: Rc<OnceCell<LazyRenderingTree>>,
    },
    Clip {
        path: crate::Path,
        clip_op: crate::ClipOp,
        lazy: Rc<OnceCell<LazyRenderingTree>>,
    },
    OnTop {
        lazy: Rc<OnceCell<LazyRenderingTree>>,
    },
    Rotate {
        angle: Angle,
        lazy: Rc<OnceCell<LazyRenderingTree>>,
    },
    Children {
        children: Vec<Rc<OnceCell<LazyRenderingTree>>>,
    },
    RenderingTree {
        rendering_tree: RenderingTree,
    },
}
impl LazyRenderingTree {
    pub(crate) fn into_rendering_tree(self) -> RenderingTree {
        let lazy_to_rendering_tree = |lazy: Rc<OnceCell<LazyRenderingTree>>| {
            Rc::into_inner(lazy)
                .unwrap()
                .take()
                .unwrap()
                .into_rendering_tree()
        };

        match self {
            LazyRenderingTree::Translate { xy, lazy } => {
                let rendering_tree = lazy_to_rendering_tree(lazy);
                crate::translate(xy.x, xy.y, rendering_tree)
            }
            LazyRenderingTree::Absolute { xy, lazy } => {
                let rendering_tree = lazy_to_rendering_tree(lazy);
                crate::absolute(xy.x, xy.y, rendering_tree)
            }
            LazyRenderingTree::Clip {
                path,
                clip_op,
                lazy,
            } => {
                let rendering_tree = lazy_to_rendering_tree(lazy);
                RenderingTree::Special(SpecialRenderingNode::Clip(ClipNode {
                    path,
                    clip_op,
                    rendering_tree: Box::new(rendering_tree),
                }))
            }
            LazyRenderingTree::OnTop { lazy } => {
                let rendering_tree = lazy_to_rendering_tree(lazy);
                crate::on_top(rendering_tree)
            }
            LazyRenderingTree::Rotate { angle, lazy } => {
                let rendering_tree = lazy_to_rendering_tree(lazy);
                crate::rotate(angle, rendering_tree)
            }
            LazyRenderingTree::Children { children } => {
                crate::render(children.into_iter().map(lazy_to_rendering_tree))
            }
            LazyRenderingTree::RenderingTree { rendering_tree } => rendering_tree,
        }
    }
}
