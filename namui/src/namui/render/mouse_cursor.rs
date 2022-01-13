use super::{RenderingTree, SpecialRenderingNode};
use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
pub enum MouseCursor {
    TopBottomResize,
    LeftRightResize,
    LeftTopRightBottomResize,
    RightTopLeftBottomResize,
    Default,
    Text,
    Grab,
    Move,
    Pointer,
}

#[derive(Serialize, Clone, Debug)]
pub struct MouseCursorNode {
    pub(crate) rendering_tree: Vec<RenderingTree>,
    pub cursor: MouseCursor,
}

impl RenderingTree {
    pub fn with_mouse_cursor(self, cursor: MouseCursor) -> RenderingTree {
        RenderingTree::Special(SpecialRenderingNode::MouseCursor(MouseCursorNode {
            rendering_tree: vec![self],
            cursor,
        }))
    }
}
