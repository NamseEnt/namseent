use crate as crdt;
use crdt::HistorySystem;

pub mod system_tree_0 {
    use crate as crdt;
    use crdt::{history, History};
    use serde::*;

    #[history]
    #[derive(Debug, Clone, PartialEq)]
    pub struct SystemTree {
        pub a: NotHistory,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct NotHistory {
        pub b: i32,
    }
}
#[test]
fn serde_using_macro_works() {
    let mut history_system: HistorySystem<system_tree_0::SystemTree> =
        HistorySystem::new(system_tree_0::SystemTree {
            a: system_tree_0::NotHistory { b: 1 },
        });

    history_system.mutate(|state| {
        state.a.b = 2;
    });
    let version_0_encode = history_system.encode();

    let history_system = HistorySystem::decode(&version_0_encode);

    assert_eq!(
        system_tree_0::SystemTree {
            a: system_tree_0::NotHistory { b: 2 }
        },
        history_system.get_state()
    );
}
