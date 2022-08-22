use crdt::List;
use editor_core::*;

#[history(version = 1)]
#[derive(Debug, Clone)]
pub struct SystemTree {
    pub sequence_list: List<Sequence>,
}

#[history]
#[derive(Debug, Clone)]
pub struct Sequence {
    pub name: String,
}

pub fn migrate(_prev: super::system_tree_0::SystemTree) -> SystemTree {
    SystemTree {
        sequence_list: List::new([]),
    }
}
