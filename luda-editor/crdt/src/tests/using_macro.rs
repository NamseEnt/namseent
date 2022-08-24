use crate as crdt;
use crdt::{history, History, HistorySystem, List, Map};

#[test]
fn derive_macro_works() {
    #[history]
    #[derive(Debug, Clone, PartialEq)]
    struct A {
        a: i32,
        b: String,
    }

    #[history(version = 0)]
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
