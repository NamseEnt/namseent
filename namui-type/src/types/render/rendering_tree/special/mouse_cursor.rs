use super::*;

#[type_derives]
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
impl MouseCursor {
    pub fn to_css_cursor_value(&self) -> &str {
        match self {
            Self::Default => "default",
            Self::TopBottomResize => "ns-resize",
            Self::LeftRightResize => "ew-resize",
            Self::LeftTopRightBottomResize => "nwse-resize",
            Self::RightTopLeftBottomResize => "nesw-resize",
            Self::Text => "text",
            Self::Grab => "grab",
            Self::Move => "move",
            Self::Pointer => "pointer",
            MouseCursor::Custom(_) => "none",
        }
    }
}

#[type_derives]
pub struct MouseCursorNode {
    pub rendering_tree: Box<RenderingTree>,
    pub cursor: Box<MouseCursor>,
}

// impl RenderingTree {
//     pub fn with_mouse_cursor(self, cursor: MouseCursor) -> RenderingTree {
//         RenderingTree::Special(SpecialRenderingNode::MouseCursor(MouseCursorNode {
//             rendering_tree: Box::new(self),
//             cursor: Box::new(cursor),
//         }))
//     }
// }
