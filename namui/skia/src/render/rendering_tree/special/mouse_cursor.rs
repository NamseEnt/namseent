use super::*;

#[type_derives(Hash, Eq, -serde::Serialize, -serde::Deserialize)]
pub enum MouseCursor {
    TopBottomResize,
    LeftRightResize,
    LeftTopRightBottomResize,
    RightTopLeftBottomResize,
    Default,
    Text,
    Grab,
    Grabbing,
    Move,
    Pointer,
    Crosshair,
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
            Self::Grabbing => "grabbing",
            Self::Move => "move",
            Self::Pointer => "pointer",
            Self::Crosshair => "crosshair",
            MouseCursor::Custom(_) => "none",
        }
    }
}
