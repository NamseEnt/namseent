mod special;

use crate::*;
pub use special::*;

#[type_derives(Default, -serde::Deserialize)]
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

    // pub fn to_bytes(&self) -> Vec<u8> {
    //     postcard::to_allocvec(self).unwrap()
    // }

    // pub fn from_bytes(bytes: &[u8]) -> Self {
    //     postcard::from_bytes(bytes).unwrap()
    // }
}

// impl std::iter::IntoIterator for RenderingTree {
//     type Item = RenderingTree;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         match self {
//             RenderingTree::Children(children) => children.into_iter(),
//             RenderingTree::Node(_) | RenderingTree::Special(_) => vec![self].into_iter(),
//             RenderingTree::Empty => vec![].into_iter(),
//             RenderingTree::Static(rendering_tree) => rendering_tree.into_iter(),
//         }
//     }
// }
