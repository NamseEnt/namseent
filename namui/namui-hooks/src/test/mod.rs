use crate::*;
use std::sync::{Arc, Mutex};

struct MockSkCalculate;
impl SkCalculate for MockSkCalculate {
    fn group_glyph(&self, _font: &Font, _paint: &Paint) -> Arc<dyn GroupGlyph> {
        unimplemented!()
    }

    fn font_metrics(&self, _font: &Font) -> Option<FontMetrics> {
        unimplemented!()
    }

    fn load_typeface(&self, _typeface_name: &str, _bytes: &[u8]) {
        unimplemented!()
    }

    fn path_contains_xy(&self, _path: &Path, _paint: Option<&Paint>, _xy: Xy<Px>) -> bool {
        unimplemented!()
    }

    fn path_bounding_box(&self, _path: &Path, _paint: Option<&Paint>) -> Option<Rect<Px>> {
        unimplemented!()
    }

    fn image(&self, _image_source: &ImageSource) -> Option<Image> {
        unimplemented!()
    }

    fn load_image(&self, _image_source: &ImageSource, _encoded_image: &[u8]) -> ImageInfo {
        unimplemented!()
    }

    fn load_image_from_raw(&self, _image_info: ImageInfo, _bitmap: &[u8]) -> ImageHandle {
        unimplemented!()
    }
}

#[test]
fn memo_should_work() {
    use std::sync::{atomic::AtomicUsize, Arc};

    let mut world = World::init(
        || Instant::from_std(std::time::Instant::now()),
        &MockSkCalculate,
    );

    let record = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct A {
        record: Arc<AtomicUsize>,
        update_state: bool,
        update_state_1: bool,
    }

    impl StaticType for A {}
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
    use std::sync::{atomic::AtomicUsize, Arc};

    let mut world = World::init(
        || Instant::from_std(std::time::Instant::now()),
        &MockSkCalculate,
    );

    let record = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct A {
        record: Arc<AtomicUsize>,
        update_state: bool,
        update_state_1: bool,
    }

    impl StaticType for A {}
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
fn effect_by_memo_should_work() {
    use std::sync::{atomic::AtomicUsize, Arc};

    let mut world = World::init(
        || Instant::from_std(std::time::Instant::now()),
        &MockSkCalculate,
    );

    let call_count = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct A {
        call_count: Arc<AtomicUsize>,
        update_state: bool,
    }

    impl StaticType for A {}
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
fn interval_should_work() {
    use std::sync::{atomic::AtomicUsize, Arc};

    let now_container = Arc::new(Mutex::new(Instant::new(Duration::from_secs(0))));

    let change_time = |duration| {
        *now_container.lock().unwrap() = Instant::new(duration);
    };

    let mut world = World::init(
        {
            let now_container = now_container.clone();
            move || {
                let now = now_container.lock().unwrap();
                *now
            }
        },
        &MockSkCalculate,
    );

    let call_count = Arc::new(AtomicUsize::new(0));
    let dt_record = Arc::new(Mutex::new(Duration::from_secs(0)));

    #[derive(Debug)]
    struct A {
        call_count: Arc<AtomicUsize>,
        dt_record: Arc<Mutex<Duration>>,
    }

    impl StaticType for A {}
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
    use std::sync::{atomic::AtomicUsize, Arc};

    let mut world = World::init(
        || Instant::from_std(std::time::Instant::now()),
        &MockSkCalculate,
    );

    let record = Arc::new(AtomicUsize::new(0));

    #[derive(Debug)]
    struct A {
        record: Arc<AtomicUsize>,
        update_state: bool,
        update_state_1: bool,
    }

    impl StaticType for A {}
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
    use std::sync::{atomic::AtomicUsize, Arc};

    let mut world = World::init(
        || Instant::from_std(std::time::Instant::now()),
        &MockSkCalculate,
    );

    let record = Arc::new(AtomicUsize::new(0));

    static MY_ATOM: Atom<usize> = Atom::uninitialized();

    #[derive(Debug)]
    struct A {
        update_state: bool,
        value_in_b: Arc<AtomicUsize>,
    }

    impl StaticType for A {}
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

    impl StaticType for B {}
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
fn cloned_set_state_should_work() {
    let mut world = World::init(
        || Instant::from_std(std::time::Instant::now()),
        &MockSkCalculate,
    );

    #[derive(Debug)]
    struct A {}

    fn spawn<F>(_future: F)
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
    }

    static MY_ATOM: Atom<usize> = Atom::uninitialized();

    impl StaticType for A {}
    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let (_state, set_state) = ctx.state(|| 5);
            let (_my_atom, set_my_atom) = ctx.init_atom(&MY_ATOM, || 5);

            let set_state = set_state.cloned();
            let set_my_atom = set_my_atom.cloned();
            spawn(async move {
                set_state.set(6);
                set_my_atom.set(6);
            });
        }
    }

    World::run(&mut world, A {});
}
