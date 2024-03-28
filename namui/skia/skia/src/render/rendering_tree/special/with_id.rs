use super::*;

#[type_derives(-serde::Deserialize)]
pub struct WithIdNode {
    pub rendering_tree: Box<RenderingTree>,
    pub id: crate::Uuid,
}

impl RenderingTree {
    pub fn with_id(self, id: crate::Uuid) -> RenderingTree {
        RenderingTree::Special(SpecialRenderingNode::WithId(WithIdNode {
            rendering_tree: Box::new(self),
            id,
        }))
    }
}
