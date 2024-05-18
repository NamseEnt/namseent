use super::*;

#[type_derives()]
pub struct ScaleNode {
    pub x: f32,
    pub y: f32,
    pub rendering_tree: Box<RenderingTree>,
}

pub fn scale(x: f32, y: f32, rendering_tree: RenderingTree) -> RenderingTree {
    if rendering_tree == RenderingTree::Empty {
        return RenderingTree::Empty;
    }
    RenderingTree::Special(SpecialRenderingNode::Scale(ScaleNode {
        x,
        y,
        rendering_tree: rendering_tree.into(),
    }))
}
impl ScaleNode {
    pub fn get_matrix(&self) -> TransformMatrix {
        TransformMatrix::from_scale(self.x, self.y)
    }
}
