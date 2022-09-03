use crate as crdt;
use crdt::*;

pub mod system_tree_0 {
    use crate as crdt;
    use crate::*;
    use serde::*;

    #[derive(Debug, Clone, PartialEq)]
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

    #[derive(Debug, Clone, PartialEq)]
    pub struct HistoryItem {
        pub c: i32,
    }

    impl History for SystemTree {
        fn insert_to_array(self, txn: &mut yrs::Transaction, array: &yrs::Array, index: u32) {
            array.insert(txn, index, yrs::PrelimMap::<bool>::new());
            let head = array.get(index).unwrap().to_ymap().unwrap();
            head.insert(
                txn,
                "non_history_item",
                crdt::Value::from(self.non_history_item).into_any(),
            );
            self.history_item.insert_to_map(txn, &head, "history_item");
            self.list_history_item
                .insert_to_map(txn, &head, "list_history_item");
            self.map_history_item
                .insert_to_map(txn, &head, "map_history_item");
        }
        fn insert_to_map(
            self,
            txn: &mut yrs::Transaction,
            map: &yrs::Map,
            key: impl Into<std::rc::Rc<str>>,
        ) {
            let key: std::rc::Rc<str> = key.into();
            map.insert(txn, key.clone(), yrs::PrelimMap::<bool>::new());
            let head = map.get(key.as_ref()).unwrap().to_ymap().unwrap();
            head.insert(
                txn,
                "non_history_item",
                crdt::Value::from(self.non_history_item).into_any(),
            );
            self.history_item.insert_to_map(txn, &head, "history_item");
            self.list_history_item
                .insert_to_map(txn, &head, "list_history_item");
            self.map_history_item
                .insert_to_map(txn, &head, "map_history_item");
        }
        fn insert_to_root(self, txn: &mut yrs::Transaction) {
            let root = txn.get_map("root");
            root.insert(txn, "__version__", 0);
            root.insert(
                txn,
                "non_history_item",
                crdt::Value::from(self.non_history_item).into_any(),
            );
            self.history_item.insert_to_map(txn, &root, "history_item");
            self.list_history_item
                .insert_to_map(txn, &root, "list_history_item");
            self.map_history_item
                .insert_to_map(txn, &root, "map_history_item");
        }
        fn from_map(root: &yrs::Map) -> Self {
            Self {
                non_history_item: Value::from_yrs_value(root.get("non_history_item").unwrap())
                    .deserialize(),
                history_item: Value::from_yrs_value(root.get("history_item").unwrap()).into(),
                list_history_item: Value::from_yrs_value(root.get("list_history_item").unwrap())
                    .into(),
                map_history_item: Value::from_yrs_value(root.get("map_history_item").unwrap())
                    .into(),
            }
        }
        fn update_to_array(self, _txn: &mut yrs::Transaction, _head: &yrs::Array) {
            todo!()
        }
        fn update_to_map(self, txn: &mut yrs::Transaction, head: &yrs::Map) {
            let value_non_history_item = crdt::Value::from(self.non_history_item);
            if value_non_history_item
                != Value::from_yrs_value(head.get("non_history_item").unwrap())
            {
                head.insert(txn, "non_history_item", value_non_history_item.into_any());
            }
            self.history_item.update_to_map(
                txn,
                &mut head.get("history_item").unwrap().to_ymap().unwrap(),
            );
            self.list_history_item.update_to_array(
                txn,
                &mut head.get("list_history_item").unwrap().to_yarray().unwrap(),
            );
            self.map_history_item.update_to_map(
                txn,
                &mut head.get("map_history_item").unwrap().to_ymap().unwrap(),
            );
        }
        fn from_value(value: Value) -> Self {
            if let yrs::types::Value::YMap(map) = value.yvalue {
                Self::from_map(&map)
            } else {
                unreachable!("value is not a map, got {:?}", value);
            }
        }
        fn as_value(&self) -> crate::Value {
            unreachable!()
        }
        fn get_version() -> Option<u32> {
            Some(0)
        }
        fn migrate(_version_of_doc: u32, doc: yrs::Doc) -> Self {
            let mut txn = doc.transact();
            let root = txn.get_map("root");
            println!("{:?}", root);
            Self::from_map(&root)
        }
    }

    impl History for HistoryItem {
        fn insert_to_array(self, txn: &mut yrs::Transaction, array: &yrs::Array, index: u32) {
            array.insert(txn, index, yrs::PrelimMap::<bool>::new());
            let head = array.get(index).unwrap().to_ymap().unwrap();
            head.insert(txn, "c", crdt::Value::from(self.c).into_any());
        }
        fn insert_to_map(
            self,
            txn: &mut yrs::Transaction,
            map: &yrs::Map,
            key: impl Into<std::rc::Rc<str>>,
        ) {
            let key: std::rc::Rc<str> = key.into();
            map.insert(txn, key.clone(), yrs::PrelimMap::<bool>::new());
            let head = map.get(key.as_ref()).unwrap().to_ymap().unwrap();
            head.insert(txn, "c", crdt::Value::from(self.c).into_any());
        }
        fn insert_to_root(self, txn: &mut yrs::Transaction) {
            let root = txn.get_map("root");
            root.insert(txn, "__version__", 0);
            root.insert(txn, "c", crdt::Value::from(self.c).into_any());
        }
        fn from_map(root: &yrs::Map) -> Self {
            Self {
                c: Value::from_yrs_value(root.get("c").unwrap()).deserialize(),
            }
        }
        fn update_to_array(self, _txn: &mut yrs::Transaction, _head: &yrs::Array) {
            todo!()
        }
        fn update_to_map(self, txn: &mut yrs::Transaction, head: &yrs::Map) {
            let value_c = crdt::Value::from(self.c);
            if value_c != Value::from_yrs_value(head.get("c").unwrap()) {
                head.insert(txn, "c", value_c.into_any());
            }
        }
        fn from_value(value: Value) -> Self {
            if let yrs::types::Value::YMap(map) = value.yvalue {
                Self::from_map(&map)
            } else {
                unreachable!("value is not a map, got {:?}", value);
            }
        }
        fn as_value(&self) -> crate::Value {
            unreachable!()
        }
        fn get_version() -> Option<u32> {
            None
        }
        fn migrate(_version_of_doc: u32, _doc: yrs::Doc) -> Self {
            unreachable!()
        }
    }
}

#[test]
fn serde_manual_impl_works() {
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
