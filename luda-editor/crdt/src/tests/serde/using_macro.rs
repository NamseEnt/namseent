use crate as crdt;
use crdt::*;

pub mod system_tree_0 {
    use crate as crdt;
    use crdt::*;
    use serde::*;

    #[history]
    #[derive(PartialEq)]
    pub struct SystemTree {
        pub non_history_item: NonHistoryItem,
        pub history_item: Single<HistoryItem>,
        pub list_history_item: List<HistoryItem>,
        pub map_history_item: Map<HistoryItem>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct NonHistoryItem {
        pub b: i32,
    }

    #[history]
    #[derive(PartialEq)]
    pub struct HistoryItem {
        pub c: i32,
    }
}
#[test]
fn serde_using_macro_works() {
    let mut history_system: HistorySystem<system_tree_0::SystemTree> =
        HistorySystem::new(system_tree_0::SystemTree {
            non_history_item: system_tree_0::NonHistoryItem { b: 1 },
            history_item: Single::new(system_tree_0::HistoryItem { c: 2 }),
            list_history_item: List::new([system_tree_0::HistoryItem { c: 3 }]),
            map_history_item: Map::new([("0".to_string(), system_tree_0::HistoryItem { c: 4 })]),
        });

    history_system.mutate(|state| {
        state.non_history_item.b = 2;
        state.history_item.c = 1;
        state.list_history_item.update(0, |item| item.c = 6);
        state
            .list_history_item
            .push(system_tree_0::HistoryItem { c: 7 });
        state
            .map_history_item
            .set("0", system_tree_0::HistoryItem { c: 8 });
        state
            .map_history_item
            .set("1", system_tree_0::HistoryItem { c: 9 });
    });
    let version_0_encode = history_system.encode();

    let history_system = HistorySystem::decode(&version_0_encode);

    assert_eq!(
        system_tree_0::SystemTree {
            non_history_item: system_tree_0::NonHistoryItem { b: 2 },
            history_item: Single::new(system_tree_0::HistoryItem { c: 1 }),
            list_history_item: List::new([
                system_tree_0::HistoryItem { c: 6 },
                system_tree_0::HistoryItem { c: 7 }
            ]),
            map_history_item: Map::new([
                ("0".to_string(), system_tree_0::HistoryItem { c: 8 }),
                ("1".to_string(), system_tree_0::HistoryItem { c: 9 })
            ]),
        },
        history_system.get_state()
    );
}
