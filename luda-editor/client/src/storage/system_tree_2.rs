use crdt::List;
use editor_core::*;

#[history(version = 2)]
#[derive(Debug, Clone)]
pub struct SystemTree {
    pub sequence_list: List<Sequence>,
}

#[history]
#[derive(Debug, Clone)]
pub struct Sequence {
    pub id: String,
    pub name: String,
}

pub fn migrate(prev: super::system_tree_1::SystemTree) -> SystemTree {
    SystemTree {
        sequence_list: prev
            .sequence_list
            .iter()
            .map(|sequence| Sequence {
                id: namui::nanoid(),
                name: sequence.name.clone(),
            })
            .collect(),
    }
}
