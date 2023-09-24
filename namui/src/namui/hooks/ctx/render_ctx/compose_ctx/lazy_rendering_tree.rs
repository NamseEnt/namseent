use crate::*;
use std::sync::{Arc, Mutex};

pub(crate) enum LazyRenderingTree {
    Translate {
        xy: Xy<Px>,
        lazy: Arc<Mutex<Option<LazyRenderingTree>>>,
    },
    Absolute {
        xy: Xy<Px>,
        lazy: Arc<Mutex<Option<LazyRenderingTree>>>,
    },
    Clip {
        path: crate::Path,
        clip_op: crate::ClipOp,
        lazy: Arc<Mutex<Option<LazyRenderingTree>>>,
    },
    OnTop {
        lazy: Arc<Mutex<Option<LazyRenderingTree>>>,
    },
    Rotate {
        angle: Angle,
        lazy: Arc<Mutex<Option<LazyRenderingTree>>>,
    },
    Children {
        children: Vec<Arc<Mutex<Option<LazyRenderingTree>>>>,
    },
    RenderingTree {
        rendering_tree: RenderingTree,
    },
}
impl LazyRenderingTree {
    pub(crate) fn into_rendering_tree(self) -> RenderingTree {
        match self {
            LazyRenderingTree::Translate { xy, lazy } => {
                let rendering_tree = lazy.lock().unwrap().take().unwrap().into_rendering_tree();
                crate::translate(xy.x, xy.y, rendering_tree)
            }
            LazyRenderingTree::Absolute { xy, lazy } => {
                let rendering_tree = lazy.lock().unwrap().take().unwrap().into_rendering_tree();
                crate::absolute(xy.x, xy.y, rendering_tree)
            }
            LazyRenderingTree::Clip {
                path,
                clip_op,
                lazy,
            } => {
                let rendering_tree = lazy.lock().unwrap().take().unwrap().into_rendering_tree();
                RenderingTree::Special(SpecialRenderingNode::Clip(ClipNode {
                    path,
                    clip_op,
                    rendering_tree: Box::new(rendering_tree),
                }))
            }
            LazyRenderingTree::OnTop { lazy } => {
                let rendering_tree = lazy.lock().unwrap().take().unwrap().into_rendering_tree();
                crate::on_top(rendering_tree)
            }
            LazyRenderingTree::Rotate { angle, lazy } => {
                let rendering_tree = lazy.lock().unwrap().take().unwrap().into_rendering_tree();
                crate::rotate(angle, rendering_tree)
            }
            LazyRenderingTree::Children { children } => crate::render(
                children
                    .into_iter()
                    .map(|child| child.lock().unwrap().take().unwrap().into_rendering_tree()),
            ),
            LazyRenderingTree::RenderingTree { rendering_tree } => rendering_tree,
        }
    }
}
