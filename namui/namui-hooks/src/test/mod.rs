mod mouse_event;
mod pass_sig;

use crate::*;
use std::sync::Mutex;

#[test]
fn memo_should_work() {
    use std::sync::{Arc, atomic::AtomicUsize};

    let mut world = World::init(Instant::now);

    let record = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct A {
        record: Arc<AtomicUsize>,
        update_state: bool,
        update_state_1: bool,
    }

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let (state, set_state) = ctx.state(|| 1);
            let (state1, set_state1) = ctx.state(|| 2);

            let memo = ctx.memo(|| *state + *state1);

            self.record
                .store(*memo, std::sync::atomic::Ordering::Relaxed);

            if self.update_state {
                set_state.set(2);
            }

            if self.update_state_1 {
                set_state1.set(3);
            }
        }
    }

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: true,
            update_state_1: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 3);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
            update_state_1: true,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 4);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
            update_state_1: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 5);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
            update_state_1: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 5);
}

#[test]
fn effect_by_set_state_should_work() {
    use std::sync::{Arc, atomic::AtomicUsize};

    let mut world = World::init(Instant::now);

    let record = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct A {
        record: Arc<AtomicUsize>,
        update_state: bool,
        update_state_1: bool,
    }

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let (state, set_state) = ctx.state(|| 1);
            let (state1, set_state1) = ctx.state(|| 2);

            ctx.effect("", || {
                self.record
                    .store(*state + *state1, std::sync::atomic::Ordering::Relaxed);
            });

            if self.update_state {
                set_state.set(2);
            }

            if self.update_state_1 {
                set_state1.set(3);
            }
        }
    }

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: true,
            update_state_1: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 3);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
            update_state_1: true,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 4);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
            update_state_1: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 5);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
            update_state_1: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 5);
}

#[test]
fn effect_by_memo_should_work() {
    use std::sync::{Arc, atomic::AtomicUsize};

    let mut world = World::init(Instant::now);

    let call_count = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct A {
        call_count: Arc<AtomicUsize>,
        update_state: bool,
    }

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let (state, set_state) = ctx.state(|| 1);

            let memo = ctx.memo(|| *state * 2);

            ctx.effect("run it", || {
                memo.record_as_used();
                self.call_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            });

            if self.update_state {
                set_state.mutate(|state| *state = 2);
            }
        }
    }

    World::run(
        &mut world,
        A {
            call_count: call_count.clone(),
            update_state: false,
        },
    );

    assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 1);

    World::run(
        &mut world,
        A {
            call_count: call_count.clone(),
            update_state: false,
        },
    );

    assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 1);

    World::run(
        &mut world,
        A {
            call_count: call_count.clone(),
            update_state: true,
        },
    );

    assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 1);

    World::run(
        &mut world,
        A {
            call_count: call_count.clone(),
            update_state: false,
        },
    );

    assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 2);

    World::run(
        &mut world,
        A {
            call_count: call_count.clone(),
            update_state: false,
        },
    );
}

#[test]
fn effect_clean_up_should_work() {
    use std::sync::{Arc, atomic::AtomicUsize};

    let mut world = World::init(Instant::now);

    let record = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct A {
        record: Arc<AtomicUsize>,
        use_b: bool,
    }

    #[derive(Debug)]
    struct B {
        record: Arc<AtomicUsize>,
    }

    #[derive(Debug)]
    struct C {
        record: Arc<AtomicUsize>,
    }

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let Self { record, use_b } = self;

            if use_b {
                ctx.add(B { record })
            } else {
                ctx.add(C { record })
            };
        }
    }

    impl Component for B {
        fn render(self, ctx: &RenderCtx) {
            let Self { record } = self;

            ctx.effect("B", || {
                move || {
                    record.fetch_add(5, std::sync::atomic::Ordering::Relaxed);
                }
            })
        }
    }

    impl Component for C {
        fn render(self, ctx: &RenderCtx) {
            let Self { record } = self;

            ctx.effect("C", || {
                move || {
                    record.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            })
        }
    }

    World::run(
        &mut world,
        A {
            record: record.clone(),
            use_b: true,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 0);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            use_b: true,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 0);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            use_b: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 5);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            use_b: true,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 6);
}

#[test]
fn interval_should_work() {
    use std::sync::{Arc, atomic::AtomicUsize};

    let now_container = Arc::new(Mutex::new(Instant::new(Duration::from_secs(0))));

    let change_time = |duration| {
        *now_container.lock().unwrap() = Instant::new(duration);
    };

    let mut world = World::init({
        let now_container = now_container.clone();
        move || {
            let now = now_container.lock().unwrap();
            *now
        }
    });

    let call_count = Arc::new(AtomicUsize::new(0));
    let dt_record = Arc::new(Mutex::new(Duration::from_secs(0)));

    #[derive(Debug)]
    struct A {
        call_count: Arc<AtomicUsize>,
        dt_record: Arc<Mutex<Duration>>,
    }

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            ctx.interval("every 30 ms", Duration::from_millis(30), |dt| {
                self.call_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                self.dt_record.lock().unwrap().clone_from(&dt);
            });
        }
    }

    World::run(
        &mut world,
        A {
            call_count: call_count.clone(),
            dt_record: dt_record.clone(),
        },
    );

    assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(*dt_record.lock().unwrap(), Duration::from_millis(0));

    World::run(
        &mut world,
        A {
            call_count: call_count.clone(),
            dt_record: dt_record.clone(),
        },
    );

    assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(*dt_record.lock().unwrap(), Duration::from_millis(0));

    change_time(Duration::from_millis(25));

    World::run(
        &mut world,
        A {
            call_count: call_count.clone(),
            dt_record: dt_record.clone(),
        },
    );

    assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(*dt_record.lock().unwrap(), Duration::from_millis(0)); // because not called interval

    change_time(Duration::from_millis(35));

    World::run(
        &mut world,
        A {
            call_count: call_count.clone(),
            dt_record: dt_record.clone(),
        },
    );

    assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 2);
    assert_eq!(*dt_record.lock().unwrap(), Duration::from_millis(35));

    World::run(
        &mut world,
        A {
            call_count: call_count.clone(),
            dt_record: dt_record.clone(),
        },
    );

    assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 2);
    assert_eq!(*dt_record.lock().unwrap(), Duration::from_millis(35));

    change_time(Duration::from_millis(65));

    World::run(
        &mut world,
        A {
            call_count: call_count.clone(),
            dt_record: dt_record.clone(),
        },
    );

    assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 3);
    assert_eq!(*dt_record.lock().unwrap(), Duration::from_millis(30));
}

#[test]
fn controlled_memo_should_work() {
    use std::sync::{Arc, atomic::AtomicUsize};

    let mut world = World::init(Instant::now);

    let record = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct A {
        record: Arc<AtomicUsize>,
        update_state: bool,
        update_state_1: bool,
    }

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let (state, set_state) = ctx.state(|| 1);
            let (state1, set_state1) = ctx.state(|| 2);

            let memo = ctx.controlled_memo::<Vec<usize>>(|prev| {
                let value = *state + *state1;
                match prev {
                    Some(mut prev) => {
                        if prev.last() == Some(&value) {
                            return ControlledMemo::Unchanged(prev);
                        }

                        prev.push(value);
                        ControlledMemo::Changed(prev)
                    }
                    None => ControlledMemo::Changed(vec![value]),
                }
            });

            self.record.store(
                memo.last().cloned().unwrap_or_default(),
                std::sync::atomic::Ordering::Relaxed,
            );

            if self.update_state {
                set_state.set(2);
            }

            if self.update_state_1 {
                set_state1.set(3);
            }
        }
    }

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: true,
            update_state_1: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 3);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
            update_state_1: true,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 4);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
            update_state_1: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 5);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
            update_state_1: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 5);
}

#[test]
fn atom_should_work() {
    use std::sync::{Arc, atomic::AtomicUsize};

    let mut world = World::init(Instant::now);

    let record = Arc::new(AtomicUsize::new(0));

    static MY_ATOM: Atom<usize> = Atom::uninitialized();

    #[derive(Debug)]
    struct A {
        update_state: bool,
        value_in_b: Arc<AtomicUsize>,
    }

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let (_my_atom, set_my_atom) = ctx.init_atom(&MY_ATOM, || 5);

            if self.update_state {
                set_my_atom.set(6);
            }

            ctx.add(B {
                value_in_b: self.value_in_b.clone(),
            });
        }
    }

    #[derive(Debug)]
    struct B {
        value_in_b: Arc<AtomicUsize>,
    }

    impl Component for B {
        fn render(self, ctx: &RenderCtx) {
            let (my_atom, _set_my_atom) = ctx.atom(&MY_ATOM);

            ctx.effect("record value_in_b", || {
                self.value_in_b
                    .store(*my_atom, std::sync::atomic::Ordering::Relaxed);
            });
        }
    }

    World::run(
        &mut world,
        A {
            update_state: false,
            value_in_b: record.clone(),
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 5);

    World::run(
        &mut world,
        A {
            update_state: true,
            value_in_b: record.clone(),
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 5);

    World::run(
        &mut world,
        A {
            update_state: false,
            value_in_b: record.clone(),
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 6);

    World::run(
        &mut world,
        A {
            update_state: false,
            value_in_b: record.clone(),
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 6);
}

#[test]
fn set_state_should_be_copied_into_async_move() {
    let mut world = World::init(Instant::now);

    #[derive(Debug)]
    struct A {}

    fn spawn<F>(_future: F)
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
    }

    static MY_ATOM: Atom<usize> = Atom::uninitialized();

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let (_state, set_state) = ctx.state(|| 5);
            let (_my_atom, set_my_atom) = ctx.init_atom(&MY_ATOM, || 5);

            spawn(async move {
                set_state.set(6);
                set_my_atom.set(6);
            });
        }
    }

    World::run(&mut world, A {});
}

#[test]
fn set_state_should_be_copied_into_async_effect() {
    let mut world = World::init(Instant::now);

    #[derive(Debug)]
    struct A {}

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let (state0, set_state0) = ctx.state(|| 5);
            let (state1, set_state1) = ctx.state(|| 5);

            ctx.async_effect("single sig deps test", state0, move |state| async move {
                set_state0.set(state + 5);
            });

            ctx.async_effect(
                "sig tuple deps test",
                (state0, state1),
                move |(state, state1)| async move {
                    let value = state + state1;
                    set_state1.set(value);
                },
            );

            ctx.async_effect(
                "unit type deps test",
                (),
                move |_unit_type: ()| async move {},
            );

            // TODO: Need to test when deps changed by props

            ctx.async_effect("single ref deps test", &5, move |five| async move {
                let value = five;
                set_state1.set(value);
            });

            ctx.async_effect(
                "ref tuple deps test",
                (&5, &7),
                move |(five, seven): (i32, i32)| async move {
                    let value = five + seven;
                    set_state1.set(value);
                },
            );

            ctx.async_effect(
                "sig ref tuple deps test",
                (state0, &7),
                move |(state0, seven)| async move {
                    let value = state0 + seven;
                    set_state1.set(value);
                },
            );
        }
    }

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        World::run(&mut world, A {});
    });
}

#[test]
fn tuple_set_state_should_work() {
    use std::sync::{Arc, atomic::AtomicUsize};

    let mut world = World::init(Instant::now);

    let record = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct A {
        record: Arc<AtomicUsize>,
        update_state: bool,
    }

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let (state, set_state) = ctx.state(|| 1.0);
            let (state1, set_state1) = ctx.state(|| 2);
            let (state2, set_state2) = ctx.state(|| 2);

            self.record.store(
                *state as usize + *state1 + *state2 as usize,
                std::sync::atomic::Ordering::Relaxed,
            );

            if self.update_state {
                // NOTE: I shuffled set_state order on test purpose
                (set_state1, set_state, set_state2).mutate(|(state1, state, state2)| {
                    *state = 2.0;
                    *state1 = 3;
                    *state2 = 4_i32;
                });
            }
        }
    }

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: true,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 5);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 9);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 9);
}
