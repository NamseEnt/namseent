use super::{RenderingTree, SpecialRenderingNode};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum MouseCursor {
    TopBottomResize,
    LeftRightResize,
    LeftTopRightBottomResize,
    RightTopLeftBottomResize,
    Default,
    Text,
    Grab,
    Move,
}

#[derive(Serialize, Clone)]
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
