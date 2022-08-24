use crate::*;

pub mod system_tree_0 {
    use crate as crdt;
    use crate::{History, Value};
    use serde::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct SystemTree {
        pub a: NotHistory,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct NotHistory {
        pub b: i32,
    }

    impl History for SystemTree {
        fn insert_to_array(self, txn: &mut yrs::Transaction, array: &yrs::Array, index: u32) {
            array.insert(txn, index, yrs::PrelimMap::<bool>::new());
            let head = array.get(index).unwrap().to_ymap().unwrap();
            head.insert(txn, "a", crdt::Value::from(self.a).into_any());
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
            head.insert(txn, "a", crdt::Value::from(self.a).into_any());
        }
        fn insert_to_root(self, txn: &mut yrs::Transaction) {
            let root = txn.get_map("root");
            root.insert(txn, "__version__", 0);
            root.insert(txn, "a", crdt::Value::from(self.a).into_any());
        }
        fn from_map(root: &yrs::Map) -> Self {
            Self {
                a: Value::from_yrs_value(root.get("a").unwrap()).deserialize(),
            }
        }
        fn update_to_array(self, _txn: &mut yrs::Transaction, _head: &yrs::Array) {
            todo!()
        }
        fn update_to_map(self, txn: &mut yrs::Transaction, head: &yrs::Map) {
            let value_a = crdt::Value::from(self.a);
            if value_a != Value::from_yrs_value(head.get("a").unwrap()) {
                head.insert(txn, "a", value_a.into_any());
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
            Some(0)
        }
        fn migrate(_version_of_doc: u32, doc: yrs::Doc) -> Self {
            let mut txn = doc.transact();
            let root = txn.get_map("root");
            println!("{:?}", root);
            Self::from_map(&root)
        }
    }
}

#[test]
fn serde_manual_impl_works() {
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
