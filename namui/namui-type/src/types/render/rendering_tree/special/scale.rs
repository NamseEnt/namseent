use super::*;

#[type_derives]
pub struct ScaleNode {
    pub x: f32,
    pub y: f32,
    pub rendering_tree: Box<RenderingTree>,
}

pub fn scale(x: f32, y: f32, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Scale(ScaleNode {
        x,
        y,
        rendering_tree: Box::new(rendering_tree),
    }))
}
impl ScaleNode {
    pub fn get_matrix(&self) -> Matrix3x3 {
        Matrix3x3::from_scale(self.x, self.y)
    }
}
