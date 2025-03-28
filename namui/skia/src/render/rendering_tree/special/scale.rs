use super::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct ScaleNode {
    pub x: OrderedFloat<f32>,
    pub y: OrderedFloat<f32>,
    pub rendering_tree: Box<RenderingTree>,
}

pub fn scale(x: f32, y: f32, rendering_tree: RenderingTree) -> RenderingTree {
    if rendering_tree == RenderingTree::Empty {
        return RenderingTree::Empty;
    }
    RenderingTree::Special(SpecialRenderingNode::Scale(ScaleNode {
        x: x.into(),
        y: y.into(),
        rendering_tree: rendering_tree.into(),
    }))
}
impl ScaleNode {
    pub fn get_matrix(&self) -> TransformMatrix {
        TransformMatrix::from_scale(self.x.into(), self.y.into())
    }
}
