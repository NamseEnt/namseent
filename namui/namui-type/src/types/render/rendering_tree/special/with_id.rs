use super::*;

#[derive(Debug, bincode::Decode, bincode::Encode, PartialEq, Clone, Hash, Eq)]
pub struct WithIdNode {
    pub rendering_tree: Box<RenderingTree>,
    pub id: u128,
}

impl RenderingTree {
    pub fn with_id(self, id: u128) -> RenderingTree {
        RenderingTree::Special(SpecialRenderingNode::WithId(WithIdNode {
            rendering_tree: Box::new(self),
            id,
        }))
    }
}
