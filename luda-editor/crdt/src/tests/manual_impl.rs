use crate::*;

#[test]
fn manual_impl_works() {
    #[derive(Debug, Clone, PartialEq)]
    struct A {
        a: i32,
        b: String,
    }
    impl History for A {
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
            self.a.insert_to_map(txn, &mut head, "a");
            self.b.insert_to_map(txn, &mut head, "b");
        }
        fn insert_to_root(self, _txn: &mut yrs::Transaction) {
            unreachable!()
        }
        fn from_map(root: &yrs::Map) -> Self {
            A {
                a: Value::from_yrs_value(root.get("a").unwrap()).into(),
                b: Value::from_yrs_value(root.get("b").unwrap()).into(),
            }
        }
        fn update_to_array(self, _txn: &mut yrs::Transaction, _head: &yrs::Array) {
            unreachable!()
        }
        fn update_to_map(self, _txn: &mut yrs::Transaction, _head: &yrs::Map) {
            unreachable!()
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

    #[derive(Debug, Clone, PartialEq)]
    struct B {
        i32: i32,
        string: String,
        a_list: List<A>,
        a_map: Map<A>,
        i32_list: List<i32>,
        i32_map: Map<i32>,
        string_list: List<String>,
        string_map: Map<String>,
    }

    impl History for B {
        fn insert_to_array(self, txn: &mut yrs::Transaction, array: &yrs::Array, index: u32) {
            array.insert(txn, index, yrs::PrelimMap::<bool>::new());
            let mut head = array.get(index).unwrap().to_ymap().unwrap();
            self.i32.insert_to_map(txn, &mut head, "i32");
            self.string.insert_to_map(txn, &mut head, "string");
            self.a_list.insert_to_map(txn, &mut head, "a_list");
            self.a_map.insert_to_map(txn, &mut head, "a_map");
            self.i32_list.insert_to_map(txn, &mut head, "i32_list");
            self.i32_map.insert_to_map(txn, &mut head, "i32_map");
            self.string_list
                .insert_to_map(txn, &mut head, "string_list");
            self.string_map.insert_to_map(txn, &mut head, "string_map");
        }
        fn insert_to_map(
            self,
            txn: &mut yrs::Transaction,
            map: &yrs::Map,
            key: impl Into<std::rc::Rc<str>>,
        ) {
            let key: std::rc::Rc<str> = key.into();
            map.insert(txn, key.clone(), yrs::PrelimMap::<bool>::new());
            let mut head = map.get(&key).unwrap().to_ymap().unwrap();
            self.i32.insert_to_map(txn, &mut head, "i32");
            self.string.insert_to_map(txn, &mut head, "string");
            self.a_list.insert_to_map(txn, &mut head, "a_list");
            self.a_map.insert_to_map(txn, &mut head, "a_map");
            self.i32_list.insert_to_map(txn, &mut head, "i32_list");
            self.i32_map.insert_to_map(txn, &mut head, "i32_map");
            self.string_list
                .insert_to_map(txn, &mut head, "string_list");
            self.string_map.insert_to_map(txn, &mut head, "string_map");
        }
        fn insert_to_root(self, txn: &mut yrs::Transaction) {
            let mut root = txn.get_map("root");
            self.i32.insert_to_map(txn, &mut root, "i32");
            self.string.insert_to_map(txn, &mut root, "string");
            self.a_list.insert_to_map(txn, &mut root, "a_list");
            self.a_map.insert_to_map(txn, &mut root, "a_map");
            self.i32_list.insert_to_map(txn, &mut root, "i32_list");
            self.i32_map.insert_to_map(txn, &mut root, "i32_map");
            self.string_list
                .insert_to_map(txn, &mut root, "string_list");
            self.string_map.insert_to_map(txn, &mut root, "string_map");
        }
        fn from_map(root: &yrs::Map) -> Self {
            Self {
                i32: Value::from_yrs_value(root.get("i32").unwrap()).into(),
                string: Value::from_yrs_value(root.get("string").unwrap()).into(),
                a_list: Value::from_yrs_value(root.get("a_list").unwrap()).into(),
                a_map: Value::from_yrs_value(root.get("a_map").unwrap()).into(),
                i32_list: Value::from_yrs_value(root.get("i32_list").unwrap()).into(),
                i32_map: Value::from_yrs_value(root.get("i32_map").unwrap()).into(),
                string_list: Value::from_yrs_value(root.get("string_list").unwrap()).into(),
                string_map: Value::from_yrs_value(root.get("string_map").unwrap()).into(),
            }
        }
        fn update_to_array(self, _txn: &mut yrs::Transaction, _head: &yrs::Array) {
            unreachable!()
        }
        fn update_to_map(self, txn: &mut yrs::Transaction, head: &yrs::Map) {
            if Value::from(self.i32) != Value::from_yrs_value(head.get("i32").unwrap()) {
                head.insert(txn, "i32", self.i32);
            }
            if self.string != Value::from_yrs_value(head.get("string").unwrap()).into_string() {
                head.insert(txn, "string", self.string);
            }
            self.a_list
                .update_to_array(txn, &mut head.get("a_list").unwrap().to_yarray().unwrap());
            self.a_map
                .update_to_map(txn, &mut head.get("a_map").unwrap().to_ymap().unwrap());
            self.i32_list
                .update_to_array(txn, &mut head.get("i32_list").unwrap().to_yarray().unwrap());
            self.i32_map
                .update_to_map(txn, &mut head.get("i32_map").unwrap().to_ymap().unwrap());
            self.string_list.update_to_array(
                txn,
                &mut head.get("string_list").unwrap().to_yarray().unwrap(),
            );
            self.string_map
                .update_to_map(txn, &mut head.get("string_map").unwrap().to_ymap().unwrap());
        }
        fn from_value(value: crate::Value) -> Self {
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

    let history_system = HistorySystem::new(B {
        i32: 1,
        string: "hello".to_string(),
        a_list: List::new([]),
        a_map: Map::new([]),
        i32_list: List::new([]),
        i32_map: Map::new([]),
        string_list: List::new([]),
        string_map: Map::new([]),
    });

    assert_eq!(
        B {
            i32: 1,
            string: "hello".to_string(),
            a_list: List::new([]),
            a_map: Map::new([]),
            i32_list: List::new([]),
            i32_map: Map::new([]),
            string_list: List::new([]),
            string_map: Map::new([]),
        },
        history_system.get_state()
    );

    let encoded = history_system.encode();
    println!("encoded: {:?}", encoded);
    {
        let history_system = HistorySystem::decode(&encoded);
        assert_eq!(
            B {
                i32: 1,
                string: "hello".to_string(),
                a_list: List::new([]),
                a_map: Map::new([]),
                i32_list: List::new([]),
                i32_map: Map::new([]),
                string_list: List::new([]),
                string_map: Map::new([]),
            },
            history_system.get_state()
        );
    }

    let encoded_1 = {
        let mut history_system = HistorySystem::decode(&encoded);
        history_system.mutate(|state: &mut B| {
            state.i32 = 2;
            state.string = "world".to_string();
            state.a_list.push(A {
                a: 1,
                b: "a".to_string(),
            });
            state.a_map.set(
                "1",
                A {
                    a: 2,
                    b: "b".to_string(),
                },
            );
            state.i32_list.push(1);
            state.i32_map.set("1", 2);
            state.string_list.push("a".to_string());
            state.string_map.set("1", "b".to_string());
        });
        assert_eq!(
            B {
                i32: 2,
                string: "world".to_string(),
                a_list: List::new([A {
                    a: 1,
                    b: "a".to_string()
                }]),
                a_map: Map::new([(
                    "1".to_string(),
                    A {
                        a: 2,
                        b: "b".to_string()
                    }
                )]),
                i32_list: List::new([1]),
                i32_map: Map::new([("1".to_string(), 2)]),
                string_list: List::new(["a".to_string()]),
                string_map: Map::new([("1".to_string(), "b".to_string())]),
            },
            history_system.get_state()
        );
        history_system.encode()
    };
    println!("encoded_1: {:?}", encoded_1);

    let encoded_2 = {
        let mut history_system = HistorySystem::decode(&encoded);
        history_system.mutate(|state: &mut B| {
            state.i32 = 3;
            state.string = "konomi".to_string();
            state.a_list.push(A {
                a: 2,
                b: "b".to_string(),
            });
            state.a_map.set(
                "2",
                A {
                    a: 3,
                    b: "c".to_string(),
                },
            );
            state.i32_list.push(2);
            state.i32_map.set("2", 3);
            state.string_list.push("b".to_string());
            state.string_map.set("1", "c".to_string()); // NOTE: I seted in "1", not "2".
        });
        assert_eq!(
            B {
                i32: 3,
                string: "konomi".to_string(),
                a_list: List::new([A {
                    a: 2,
                    b: "b".to_string()
                }]),
                a_map: Map::new([(
                    "2".to_string(),
                    A {
                        a: 3,
                        b: "c".to_string()
                    }
                )]),
                i32_list: List::new([2]),
                i32_map: Map::new([("2".to_string(), 3)]),
                string_list: List::new(["b".to_string()]),
                string_map: Map::new([("1".to_string(), "c".to_string())]),
            },
            history_system.get_state()
        );
        history_system.encode()
    };
    println!("encode_2: {:?}", encoded_2);

    let history_system: HistorySystem<B> = HistorySystem::decode(&encoded_1);

    assert_eq!(
        B {
            i32: 2,
            string: "world".to_string(),
            a_list: List::new([A {
                a: 1,
                b: "a".to_string()
            }]),
            a_map: Map::new([(
                "1".to_string(),
                A {
                    a: 2,
                    b: "b".to_string()
                }
            )]),
            i32_list: List::new([1]),
            i32_map: Map::new([("1".to_string(), 2)]),
            string_list: List::new(["a".to_string()]),
            string_map: Map::new([("1".to_string(), "b".to_string())]),
        },
        history_system.get_state()
    );

    let mut history_system = HistorySystem::decode(&encoded_1);
    history_system.merge(&encoded_2);

    let state_if_1_is_earlier = B {
        i32: 3,
        string: "konomi".to_string(),
        a_list: List::new([
            A {
                a: 1,
                b: "a".to_string(),
            },
            A {
                a: 2,
                b: "b".to_string(),
            },
        ]),
        a_map: Map::new([
            (
                "1".to_string(),
                A {
                    a: 2,
                    b: "b".to_string(),
                },
            ),
            (
                "2".to_string(),
                A {
                    a: 3,
                    b: "c".to_string(),
                },
            ),
        ]),
        i32_list: List::new([1, 2]),
        i32_map: Map::new([("1".to_string(), 2), ("2".to_string(), 3)]),
        string_list: List::new(["a".to_string(), "b".to_string()]),
        string_map: Map::new([("1".to_string(), "c".to_string())]),
    };
    let state_if_2_is_earlier = B {
        i32: 2,
        string: "world".to_string(),
        a_list: List::new([
            A {
                a: 2,
                b: "b".to_string(),
            },
            A {
                a: 1,
                b: "a".to_string(),
            },
        ]),
        a_map: Map::new([
            (
                "2".to_string(),
                A {
                    a: 3,
                    b: "c".to_string(),
                },
            ),
            (
                "1".to_string(),
                A {
                    a: 2,
                    b: "b".to_string(),
                },
            ),
        ]),
        i32_list: List::new([2, 1]),
        i32_map: Map::new([("1".to_string(), 2), ("2".to_string(), 3)]),
        string_list: List::new(["b".to_string(), "a".to_string()]),
        string_map: Map::new([("1".to_string(), "b".to_string())]),
    };

    if state_if_1_is_earlier != history_system.get_state() {
        assert_eq!(state_if_2_is_earlier, history_system.get_state());
    }
}
