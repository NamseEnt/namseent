mod special;

use crate::*;
pub use special::*;

#[derive(Debug, PartialEq, Clone, Default, Hash, Eq)]
pub enum RenderingTree {
    #[default]
    Empty,
    Node(DrawCommand),
    Children(Vec<RenderingTree>),
    Special(SpecialRenderingNode),
    Boxed(Box<RenderingTree>),
    BoxedChildren(Vec<Box<RenderingTree>>),
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
            RenderingTree::Boxed(rendering_tree) => {
                return rendering_tree.iter();
            }
            RenderingTree::BoxedChildren(children) => {
                for child in children.iter() {
                    vec.extend(child.iter());
                }
            }
        };

        vec.into_iter()
    }

    pub fn wrap(rendering_trees: impl IntoIterator<Item = RenderingTree>) -> RenderingTree {
        let mut iter = rendering_trees.into_iter();
        let first = 'outer: {
            for x in iter.by_ref() {
                if x != RenderingTree::Empty {
                    break 'outer x;
                }
            }
            return RenderingTree::Empty;
        };
        let second = 'outer: {
            for x in iter.by_ref() {
                if x != RenderingTree::Empty {
                    break 'outer x;
                }
            }
            return first;
        };

        let mut children = vec![first, second];
        children.extend(iter.filter(|x| *x != RenderingTree::Empty));
        RenderingTree::Children(children)
    }
}
