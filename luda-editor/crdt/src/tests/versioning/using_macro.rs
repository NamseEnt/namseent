use crate as crdt;
use crdt::HistorySystem;

pub mod system_tree_0 {
    use crate as crdt;
    use crdt::{history, History};

    #[history(version = 0)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct SystemTree {
        pub a: i32,
    }
}
pub mod system_tree_1 {
    use crate as crdt;
    use crdt::{history, History};

    #[history(version = 1)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct SystemTree {
        pub a: i32,
        pub b: i32,
    }

    pub fn migrate(prev: super::system_tree_0::SystemTree) -> SystemTree {
        SystemTree {
            a: prev.a * 2,
            b: 1313,
        }
    }
}

#[test]
fn versioning_using_macro_works() {
    let history_system: HistorySystem<system_tree_0::SystemTree> =
        HistorySystem::new(system_tree_0::SystemTree { a: 1 });
    let version_0_encode = history_system.encode();

    let history_system: HistorySystem<system_tree_1::SystemTree> =
        HistorySystem::decode(&version_0_encode);

    assert_eq!(
        system_tree_1::SystemTree { a: 2, b: 1313 },
        history_system.get_state()
    );
}
