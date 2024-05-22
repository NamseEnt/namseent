pub(crate) mod change_path_to_platform;
#[cfg(target_family = "wasm")]
mod file;
mod open_external;

use crate::*;
#[cfg(target_family = "wasm")]
pub use file::*;
pub use open_external::*;
use serde::{Deserialize, Serialize};

pub fn render(rendering_trees: impl IntoIterator<Item = RenderingTree>) -> RenderingTree {
    let mut iter = rendering_trees.into_iter();
    let first = 'outer: {
        for x in iter.by_ref() {
            if x != RenderingTree::Empty {
                break 'outer x;
            }
        }
        return RenderingTree::Empty;
    };
    let second = 'outer: {
        for x in iter.by_ref() {
            if x != RenderingTree::Empty {
                break 'outer x;
            }
        }
        return first;
    };

    let mut children = vec![first, second];
    children.extend(iter.filter(|x| *x != RenderingTree::Empty));
    RenderingTree::Children(children)
}

pub fn try_render(func: impl FnOnce() -> Option<RenderingTree>) -> RenderingTree {
    func().unwrap_or(RenderingTree::Empty)
}

pub type Rendering = RenderingTree;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Language {
    Ko,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionDirection {
    Forward,
    Backward,
    None,
}
