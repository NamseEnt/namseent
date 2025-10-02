use super::*;

#[test]
fn pass_sig_to_child_should_work() {
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
            let (state, set_state) = ctx.state(|| 1);

            if self.update_state {
                set_state.mutate(|s| *s += 1);
            }

            ctx.add(B {
                record: self.record,
                passed: state,
            });
        }
    }

    struct B<'a> {
        record: Arc<AtomicUsize>,
        passed: Sig<'a, usize>,
    }

    impl Component for B<'_> {
        fn render(self, ctx: &RenderCtx) {
            let (state, set_state) = ctx.state(|| 1);

            ctx.effect("init", || {
                set_state.set(*self.passed);
            });

            self.record
                .store(*state, std::sync::atomic::Ordering::Relaxed);
        }
    }

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: true,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 1);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 1);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: true,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 2);

    World::run(
        &mut world,
        A {
            record: record.clone(),
            update_state: false,
        },
    );

    assert_eq!(record.load(std::sync::atomic::Ordering::Relaxed), 2);
}
