pub mod closure;
mod codes;
mod event;
mod file;
mod open_external;
pub(crate) mod url;

use crate::*;
pub use closure::*;
pub use codes::*;
pub use event::*;
pub use file::*;
pub use open_external::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{atomic::AtomicBool, Arc},
};

pub fn render(rendering_trees: impl IntoIterator<Item = RenderingTree>) -> RenderingTree {
    let vec: Vec<_> = rendering_trees.into_iter().collect();

    if vec.is_empty() {
        RenderingTree::Empty
    } else if vec.len() == 1 {
        vec.into_iter().next().unwrap()
    } else {
        RenderingTree::Children(vec)
    }
}

pub fn try_render(func: impl FnOnce() -> Option<RenderingTree>) -> RenderingTree {
    func().unwrap_or(RenderingTree::Empty)
}

pub type Rendering = RenderingTree;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Language {
    Ko,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub trait AsXyPx {
    fn as_xy_px(self) -> Xy<Px>;
}
impl AsXyPx for (Px, Px) {
    fn as_xy_px(self) -> Xy<Px> {
        Xy::new(self.0, self.1)
    }
}
impl AsXyPx for Xy<Px> {
    fn as_xy_px(self) -> Xy<Px> {
        self.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionDirection {
    Forward,
    Backward,
    None,
}
