use super::*;

#[type_derives]
pub struct RotateNode {
    pub angle: Angle,
    pub rendering_tree: Box<RenderingTree>,
}

/// angle is in **cw** direction.
pub fn rotate(angle: Angle, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Rotate(RotateNode {
        angle,
        rendering_tree: Box::new(rendering_tree),
    }))
}

impl RotateNode {
    pub fn get_matrix(&self) -> Matrix3x3 {
        Matrix3x3::from_rotate(self.angle)
    }
    pub fn get_counter_wise_matrix(&self) -> Matrix3x3 {
        Matrix3x3::from_rotate(-self.angle)
    }
}
