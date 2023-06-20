use super::SpecialRenderingNode;
use crate::RenderingTree;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
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
    Custom(RenderingTree),
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct MouseCursorNode {
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
    pub cursor: Box<MouseCursor>,
}

impl RenderingTree {
    pub fn with_mouse_cursor(self, cursor: MouseCursor) -> RenderingTree {
        RenderingTree::Special(SpecialRenderingNode::MouseCursor(MouseCursorNode {
            rendering_tree: std::sync::Arc::new(self),
            cursor: Box::new(cursor),
        }))
    }
}
