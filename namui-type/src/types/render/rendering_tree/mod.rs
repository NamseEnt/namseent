mod padding;
mod special;

use crate::*;
pub use special::*;

#[type_derives(Default)]
pub struct DrawCall {
    pub commands: Vec<DrawCommand>,
}

#[type_derives(Default)]
pub struct RenderingData {
    pub draw_calls: Vec<DrawCall>,
}

#[type_derives]
pub enum RenderingTree {
    Node(RenderingData),
    Children(Vec<RenderingTree>),
    Special(SpecialRenderingNode),
    Empty,
}

impl Default for RenderingTree {
    fn default() -> Self {
        RenderingTree::Empty
    }
}

unsafe impl Send for RenderingTree {}
unsafe impl Sync for RenderingTree {}

/// NOTE
/// Order of tree traversal is important.
/// - draw = pre-order dfs (NLR)
/// - events = Reverse post-order (RLN)
/// reference: https://en.wikipedia.org/wiki/Tree_traversal
impl RenderingTree {
    pub fn iter(&self) -> impl Iterator<Item = &RenderingTree> {
        let mut vec = vec![];
        match self {
            RenderingTree::Children(children) => {
                vec.extend(children.iter());
            }
            RenderingTree::Node(_) | RenderingTree::Special(_) => vec.push(self),
            RenderingTree::Empty => {}
        };

        vec.into_iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = RenderingTree> {
        let mut vec = vec![];
        match self {
            RenderingTree::Children(children) => {
                vec.extend(children.into_iter());
            }
            RenderingTree::Node(_) | RenderingTree::Special(_) => vec.push(self),
            RenderingTree::Empty => {}
        };

        vec.into_iter()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        postcard::to_allocvec(self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        postcard::from_bytes(bytes).unwrap()
    }
}
