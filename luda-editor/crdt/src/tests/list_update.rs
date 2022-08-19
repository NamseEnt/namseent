use crate as crdt;
use crdt::{history, History, HistorySystem, List};

#[test]
fn list_update_works() {
    #[history]
    #[derive(Debug, Clone, PartialEq)]
    struct A {
        a: i32,
        b: String,
    }

    #[history(version = 0)]
    #[derive(Debug, Clone, PartialEq)]
    struct B {
        a_list: List<A>,
    }

    let history_system = HistorySystem::new(B {
        a_list: List::new([]),
    });

    assert_eq!(
        B {
            a_list: List::new([]),
        },
        history_system.get_state()
    );

    let encoded = history_system.encode();
    {
        let history_system = HistorySystem::decode(&encoded);
        assert_eq!(
            B {
                a_list: List::new([]),
            },
            history_system.get_state()
        );
    }

    let _encoded_1 = {
        let mut history_system = HistorySystem::decode(&encoded);
        history_system.mutate(|state: &mut B| {
            state.a_list.push(A {
                a: 1,
                b: "a".to_string(),
            });
        });

        history_system.mutate(|state: &mut B| {
            state.a_list.update(0, |a| {
                a.a += 1;
            });
        });

        assert_eq!(
            B {
                a_list: List::new([A {
                    a: 2,
                    b: "a".to_string()
                }]),
            },
            history_system.get_state()
        );
        history_system.encode()
    };
}
