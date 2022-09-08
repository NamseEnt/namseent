use crate::*;

pub mod system_tree_0 {
    use crate::{History, Value};

    #[derive(Debug, Clone, PartialEq)]
    pub struct SystemTree {
        pub a: i32,
    }

    impl History for SystemTree {
        fn insert_to_array(self, txn: &mut yrs::Transaction, array: &yrs::Array, index: u32) {
            array.insert(txn, index, yrs::PrelimMap::<bool>::new());
            let mut head = array.get(index).unwrap().to_ymap().unwrap();
            self.a.insert_to_map(txn, &mut head, "a");
        }
        fn insert_to_map(
            self,
            txn: &mut yrs::Transaction,
            map: &yrs::Map,
            key: impl Into<std::rc::Rc<str>>,
        ) {
            let key: std::rc::Rc<str> = key.into();
            map.insert(txn, key.clone(), yrs::PrelimMap::<bool>::new());
            let mut head = map.get(key.as_ref()).unwrap().to_ymap().unwrap();
            self.a.insert_to_map(txn, &mut head, "a");
        }
        fn insert_to_root(self, txn: &mut yrs::Transaction) {
            let mut root = txn.get_map("root");
            root.insert(txn, "__version__", 0);
            self.a.insert_to_map(txn, &mut root, "a");
        }
        fn from_map(root: &yrs::Map) -> Self {
            Self {
                a: Value::from_yrs_value(root.get("a").unwrap()).into(),
            }
        }
        fn update_to_array(self, _txn: &mut yrs::Transaction, _head: &yrs::Array) {
            todo!()
        }
        fn update_to_map(self, _txn: &mut yrs::Transaction, _head: &yrs::Map) {
            todo!()
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
pub mod system_tree_1 {
    use crate::{History, Value};

    #[derive(Debug, Clone, PartialEq)]
    pub struct SystemTree {
        pub a: i32,
        pub b: i32,
    }

    impl History for SystemTree {
        fn insert_to_array(self, txn: &mut yrs::Transaction, array: &yrs::Array, index: u32) {
            array.insert(txn, index, yrs::PrelimMap::<bool>::new());
            let mut head = array.get(index).unwrap().to_ymap().unwrap();
            self.a.insert_to_map(txn, &mut head, "a");
            self.b.insert_to_map(txn, &mut head, "b");
        }
        fn insert_to_map(
            self,
            txn: &mut yrs::Transaction,
            map: &yrs::Map,
            key: impl Into<std::rc::Rc<str>>,
        ) {
            let key: std::rc::Rc<str> = key.into();
            map.insert(txn, key.clone(), yrs::PrelimMap::<bool>::new());
            let mut head = map.get(key.as_ref()).unwrap().to_ymap().unwrap();
            head.insert(txn, "__version__", 1);
            self.a.insert_to_map(txn, &mut head, "a");
            self.b.insert_to_map(txn, &mut head, "b");
        }
        fn insert_to_root(self, txn: &mut yrs::Transaction) {
            let mut root = txn.get_map("root");
            root.insert(txn, "__version__", 1);
            self.a.insert_to_map(txn, &mut root, "a");
            self.b.insert_to_map(txn, &mut root, "b");
        }
        fn from_map(root: &yrs::Map) -> Self {
            Self {
                a: Value::from_yrs_value(root.get("a").unwrap()).into(),
                b: Value::from_yrs_value(root.get("b").unwrap()).into(),
            }
        }
        fn update_to_array(self, _txn: &mut yrs::Transaction, _head: &yrs::Array) {
            todo!()
        }
        fn update_to_map(self, _txn: &mut yrs::Transaction, _head: &yrs::Map) {
            todo!()
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
            Some(1)
        }
        fn migrate(version_of_doc: u32, doc: yrs::Doc) -> Self {
            if version_of_doc == 1 {
                let mut txn = doc.transact();
                let root = txn.get_map("root");
                Self::from_map(&root)
            } else {
                let prev = super::system_tree_0::SystemTree::migrate(version_of_doc, doc);
                super::system_tree_1::migrate(prev)
            }
        }
    }

    pub fn migrate(prev: super::system_tree_0::SystemTree) -> SystemTree {
        SystemTree {
            a: prev.a * 2,
            b: 1313,
        }
    }
}

#[test]
fn versioning_manual_impl_works() {
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
