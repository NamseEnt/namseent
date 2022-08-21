use crdt::List;
use editor_core::*;

#[history(version = 3)]
#[derive(Debug, Clone)]
pub struct SystemTree {
    pub sequence_list: List<Sequence>,
}

#[history]
#[derive(Debug, Clone)]
pub struct Sequence {
    pub id: String,
    pub name: String,
    pub cuts: List<Cut>,
}

#[history]
#[derive(Debug, Clone)]
pub struct Cut {
    pub id: String,
}

pub fn migrate(prev: super::system_tree_2::SystemTree) -> SystemTree {
    SystemTree {
        sequence_list: prev
            .sequence_list
            .iter()
            .map(|sequence| Sequence {
                id: sequence.id.clone(),
                name: sequence.name.clone(),
                cuts: List::new([]),
            })
            .collect(),
    }
}
