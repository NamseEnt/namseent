use super::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq, bincode::Encode, bincode::Decode)]
pub struct RotateNode {
    pub angle: Angle,
    pub rendering_tree: Box<RenderingTree>,
}

/// angle is in **cw** direction.
pub fn rotate(angle: Angle, rendering_tree: RenderingTree) -> RenderingTree {
    if rendering_tree == RenderingTree::Empty {
        return RenderingTree::Empty;
    }
    RenderingTree::Special(SpecialRenderingNode::Rotate(RotateNode {
        angle,
        rendering_tree: rendering_tree.into(),
    }))
}

impl RotateNode {
    pub fn get_matrix(&self) -> TransformMatrix {
        TransformMatrix::from_rotate(self.angle)
    }
    pub fn get_counter_wise_matrix(&self) -> TransformMatrix {
        TransformMatrix::from_rotate(-self.angle)
    }
}
