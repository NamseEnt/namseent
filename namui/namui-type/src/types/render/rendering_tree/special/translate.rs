use super::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct TranslateNode {
    pub x: Px,
    pub y: Px,
    pub rendering_tree: Box<RenderingTree>,
}
impl TranslateNode {
    pub fn get_matrix(&self) -> TransformMatrix {
        TransformMatrix::from_translate(self.x.as_f32(), self.y.as_f32())
    }
}

pub fn translate(x: Px, y: Px, rendering_tree: RenderingTree) -> RenderingTree {
    if rendering_tree == RenderingTree::Empty {
        return RenderingTree::Empty;
    }

    RenderingTree::Special(SpecialRenderingNode::Translate(TranslateNode {
        x,
        y,
        rendering_tree: rendering_tree.into(),
    }))
}
