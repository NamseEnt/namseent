pub mod absolute;
pub mod clip;
pub mod mouse_cursor;
pub mod on_top;
pub mod rotate;
pub mod scale;
pub mod transform;
pub mod translate;

use crate::*;
pub use absolute::*;
pub use clip::*;
pub use mouse_cursor::*;
pub use on_top::*;
pub use rotate::*;
pub use scale::*;
pub use transform::*;
pub use translate::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq, State)]
pub enum SpecialRenderingNode {
    Translate(TranslateNode),
    Clip(ClipNode),
    Absolute(AbsoluteNode),
    Rotate(RotateNode),
    Scale(ScaleNode),
    Transform(TransformNode),
    OnTop(OnTopNode),
    MouseCursor(MouseCursorNode),
}

impl SpecialRenderingNode {
    pub fn inner_rendering_tree_ref(&self) -> &RenderingTree {
        match self {
            SpecialRenderingNode::Translate(node) => node.rendering_tree.as_ref(),
            SpecialRenderingNode::Clip(node) => node.rendering_tree.as_ref(),
            SpecialRenderingNode::Absolute(node) => node.rendering_tree.as_ref(),
            SpecialRenderingNode::Rotate(node) => node.rendering_tree.as_ref(),
            SpecialRenderingNode::Scale(node) => node.rendering_tree.as_ref(),
            SpecialRenderingNode::Transform(node) => node.rendering_tree.as_ref(),
            SpecialRenderingNode::OnTop(node) => node.rendering_tree.as_ref(),
            SpecialRenderingNode::MouseCursor(node) => node.rendering_tree.as_ref(),
        }
    }
    pub fn inner_rendering_tree(self) -> RenderingTree {
        match self {
            SpecialRenderingNode::Translate(node) => *node.rendering_tree,
            SpecialRenderingNode::Clip(node) => *node.rendering_tree,
            SpecialRenderingNode::Absolute(node) => *node.rendering_tree,
            SpecialRenderingNode::Rotate(node) => *node.rendering_tree,
            SpecialRenderingNode::Scale(node) => *node.rendering_tree,
            SpecialRenderingNode::Transform(node) => *node.rendering_tree,
            SpecialRenderingNode::OnTop(node) => *node.rendering_tree,
            SpecialRenderingNode::MouseCursor(node) => *node.rendering_tree,
        }
    }
}
