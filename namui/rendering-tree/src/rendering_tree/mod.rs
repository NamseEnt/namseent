mod special;

use crate::*;
pub use special::*;

#[derive(Debug, PartialEq, Clone, Copy, Default, Hash, Eq, bincode::Encode)]
pub enum RenderingTree {
    #[default]
    Empty,
    Node(DrawCommand),
    Children(&'static [RenderingTree]),
    Special(SpecialRenderingNode),
}

/// NOTE
/// Order of tree traversal is important.
/// - draw = pre-order dfs (NLR)
/// - events = Reverse post-order (RLN)
///
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

    pub fn wrap(rendering_trees: impl IntoIterator<Item = RenderingTree>) -> RenderingTree {
        let children: Vec<RenderingTree> = rendering_trees
            .into_iter()
            .filter(|x| *x != RenderingTree::Empty)
            .collect();

        match children.len() {
            0 => RenderingTree::Empty,
            1 => children[0],
            _ => RenderingTree::Children(arena_alloc_slice(children)),
        }
    }
}
