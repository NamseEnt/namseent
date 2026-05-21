use super::*;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, bincode::Encode)]
pub struct ScaleNode {
    pub x: OrderedFloat,
    pub y: OrderedFloat,
    pub rendering_tree: &'static RenderingTree,
}

pub fn scale(x: f32, y: f32, rendering_tree: RenderingTree) -> RenderingTree {
    if rendering_tree == RenderingTree::Empty {
        return RenderingTree::Empty;
    }
    RenderingTree::Special(SpecialRenderingNode::Scale(ScaleNode {
        x: x.into(),
        y: y.into(),
        rendering_tree: arena_alloc(rendering_tree),
    }))
}
impl ScaleNode {
    pub fn get_matrix(&self) -> TransformMatrix {
        TransformMatrix::from_scale(*self.x, *self.y)
    }
}
