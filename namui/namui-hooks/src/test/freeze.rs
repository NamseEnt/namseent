use crate::*;
use std::collections::HashMap;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn freeze_and_restore_various_state_types() {
    // 1. Create first World and run component
    let mut world1 = World::init(Instant::now);

    let record = Arc::new(AtomicUsize::new(0));

    // Define various state types using struct and enum
    #[derive(Debug, Clone, PartialEq, namui_type::State)]
    struct Person {
        name: String,
        age: u32,
    }

    #[derive(Debug, Clone, PartialEq, namui_type::State)]
    enum Status {
        Active,
        Inactive,
        Pending(String),
    }

    #[derive(Debug)]
    struct TestComponent {
        record: Arc<AtomicUsize>,
    }

    impl Component for TestComponent {
        fn render(self, ctx: &RenderCtx) {
            // Vec state
            let (vec_state, _) = ctx.state(|| vec![1, 2, 3, 4, 5]);

            // HashMap state
            let (map_state, _) = ctx.state(|| {
                let mut map = HashMap::new();
                map.insert("apple".to_string(), 100);
                map.insert("banana".to_string(), 200);
                map.insert("cherry".to_string(), 300);
                map
            });

            // Struct state
            let (person_state, _) = ctx.state(|| Person {
                name: "Bob".to_string(),
                age: 30,
            });

            // Enum state
            let (status_state, _) = ctx.state(|| Status::Pending("processing".to_string()));

            // String state
            let (string_state, _) = ctx.state(|| "frozen_value".to_string());

            // Tuple state
            let (tuple_state, _) = ctx.state(|| (100, "data".to_string(), 2.71));

            // Option state
            let (option_state, _) = ctx.state(|| Some(vec![10, 20, 30, 40]));

            // Sum all state values and store in record for verification
            let option_len = match option_state.as_ref() {
                Some(v) => v.len(),
                None => 0,
            };

            let total = vec_state.iter().sum::<i32>() as usize
                + map_state.values().sum::<i32>() as usize
                + person_state.age as usize
                + (match &*status_state {
                    Status::Active => 1,
                    Status::Inactive => 0,
                    Status::Pending(_) => 2,
                })
                + string_state.len()
                + tuple_state.0 as usize
                + option_len;

            self.record.store(total, Ordering::Relaxed);
        }
    }

    // Run in first World
    World::run(
        &mut world1,
        TestComponent {
            record: record.clone(),
        },
    );

    let value_in_world1 = record.load(Ordering::Relaxed);

    // 2. Freeze the World
    let frozen_bytes = world1.freeze_states();

    // 3. Create new World and restore frozen state
    let mut world2 = World::init(Instant::now);
    world2.set_frozen_states(&frozen_bytes);

    // 4. Run same component in new World
    let record2 = Arc::new(AtomicUsize::new(0));
    World::run(
        &mut world2,
        TestComponent {
            record: record2.clone(),
        },
    );

    let value_in_world2 = record2.load(Ordering::Relaxed);

    // 5. Verify restored value matches original value
    assert_eq!(
        value_in_world1, value_in_world2,
        "Restored state should match frozen state"
    );

    // 6. Run once more in restored World to verify state persistence
    World::run(
        &mut world2,
        TestComponent {
            record: record2.clone(),
        },
    );

    let value_after_second_run = record2.load(Ordering::Relaxed);
    assert_eq!(
        value_in_world2, value_after_second_run,
        "State should persist across multiple runs"
    );
}

#[test]
fn freeze_with_nested_components() {
    let mut world1 = World::init(Instant::now);

    let parent_record = Arc::new(AtomicUsize::new(0));
    let child_record = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct Parent {
        parent_record: Arc<AtomicUsize>,
        child_record: Arc<AtomicUsize>,
    }

    #[derive(Debug)]
    struct Child {
        child_record: Arc<AtomicUsize>,
        parent_value: usize,
    }

    impl Component for Parent {
        fn render(self, ctx: &RenderCtx) {
            let (counter, _) = ctx.state(|| 20usize);
            let (list, _) = ctx.state(|| {
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                ]
            });

            self.parent_record
                .store(*counter + list.len(), Ordering::Relaxed);

            ctx.add(Child {
                child_record: self.child_record.clone(),
                parent_value: *counter,
            });
        }
    }

    impl Component for Child {
        fn render(self, ctx: &RenderCtx) {
            let (child_counter, _) = ctx.state(|| 15usize);

            self.child_record
                .store(*child_counter + self.parent_value, Ordering::Relaxed);
        }
    }

    // Run in first World
    World::run(
        &mut world1,
        Parent {
            parent_record: parent_record.clone(),
            child_record: child_record.clone(),
        },
    );

    let parent_value_in_world1 = parent_record.load(Ordering::Relaxed);
    let child_value_in_world1 = child_record.load(Ordering::Relaxed);

    // Freeze
    let frozen_bytes = world1.freeze_states();

    // Restore in new World
    let mut world2 = World::init(Instant::now);
    world2.set_frozen_states(&frozen_bytes);

    let parent_record2 = Arc::new(AtomicUsize::new(0));
    let child_record2 = Arc::new(AtomicUsize::new(0));

    World::run(
        &mut world2,
        Parent {
            parent_record: parent_record2.clone(),
            child_record: child_record2.clone(),
        },
    );

    let parent_value_in_world2 = parent_record2.load(Ordering::Relaxed);
    let child_value_in_world2 = child_record2.load(Ordering::Relaxed);

    assert_eq!(
        parent_value_in_world1, parent_value_in_world2,
        "Parent state should be restored"
    );
    assert_eq!(
        child_value_in_world1, child_value_in_world2,
        "Child state should be restored"
    );
}

#[test]
fn freeze_and_restore_atom_state() {
    use std::sync::atomic::Ordering;

    static COUNTER_ATOM: Atom<usize> = Atom::uninitialized();
    static MESSAGE_ATOM: Atom<String> = Atom::uninitialized();

    // 1. Create first World and run component
    let mut world1 = World::init(Instant::now);

    let record = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct TestComponent {
        record: Arc<AtomicUsize>,
    }

    impl Component for TestComponent {
        fn render(self, ctx: &RenderCtx) {
            let (counter, _set_counter) = ctx.init_atom(&COUNTER_ATOM, || 42);
            let (message, _set_message) = ctx.init_atom(&MESSAGE_ATOM, || "Hello Atom".to_string());

            self.record
                .store(*counter + message.len(), Ordering::Relaxed);
        }
    }

    // Run in first World
    World::run(
        &mut world1,
        TestComponent {
            record: record.clone(),
        },
    );

    let value_in_world1 = record.load(Ordering::Relaxed);

    // 2. Freeze the World
    let frozen_bytes = world1.freeze_states();

    // 3. Create new World and restore frozen state
    let mut world2 = World::init(Instant::now);
    world2.set_frozen_states(&frozen_bytes);

    // 4. Run same component in new World
    let record2 = Arc::new(AtomicUsize::new(0));
    World::run(
        &mut world2,
        TestComponent {
            record: record2.clone(),
        },
    );

    let value_in_world2 = record2.load(Ordering::Relaxed);

    // 5. Verify restored value matches original value
    assert_eq!(
        value_in_world1, value_in_world2,
        "Restored atom state should match frozen state"
    );

    // 6. Run once more in restored World to verify state persistence
    World::run(
        &mut world2,
        TestComponent {
            record: record2.clone(),
        },
    );

    let value_after_second_run = record2.load(Ordering::Relaxed);
    assert_eq!(
        value_in_world2, value_after_second_run,
        "Atom state should persist across multiple runs"
    );
}

#[test]
fn freeze_and_restore_mixed_state_and_atom() {
    use std::sync::atomic::Ordering;

    static GLOBAL_COUNTER: Atom<usize> = Atom::uninitialized();

    // 1. Create first World
    let mut world1 = World::init(Instant::now);

    let record = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct TestComponent {
        record: Arc<AtomicUsize>,
    }

    impl Component for TestComponent {
        fn render(self, ctx: &RenderCtx) {
            // Local instance state
            let (local_state, _) = ctx.state(|| 100usize);

            // Global atom state
            let (global_counter, _) = ctx.init_atom(&GLOBAL_COUNTER, || 200);

            self.record
                .store(*local_state + *global_counter, Ordering::Relaxed);
        }
    }

    // Run in first World
    World::run(
        &mut world1,
        TestComponent {
            record: record.clone(),
        },
    );

    let value_in_world1 = record.load(Ordering::Relaxed);
    assert_eq!(
        value_in_world1, 300,
        "Initial state should be 100 + 200 = 300"
    );

    // 2. Freeze the World
    let frozen_bytes = world1.freeze_states();

    // 3. Create new World and restore frozen state
    let mut world2 = World::init(Instant::now);
    world2.set_frozen_states(&frozen_bytes);

    // 4. Run same component in new World
    let record2 = Arc::new(AtomicUsize::new(0));
    World::run(
        &mut world2,
        TestComponent {
            record: record2.clone(),
        },
    );

    let value_in_world2 = record2.load(Ordering::Relaxed);

    // 5. Verify both instance state and atom state are restored correctly
    assert_eq!(
        value_in_world1, value_in_world2,
        "Both instance state and atom state should be restored"
    );
    assert_eq!(
        value_in_world2, 300,
        "Restored state should still be 100 + 200 = 300"
    );
}
