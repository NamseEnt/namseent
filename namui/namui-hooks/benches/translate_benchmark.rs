use criterion::{Criterion, criterion_group, criterion_main};
use namui_hooks::*;
use namui_type::*;

fn my_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("translate_benchmark");
    group.sample_size(10);
    group.bench_function("bench_translate", |b| b.iter(bench_translate));
    group.bench_function("bench_translate_run_twice", |b| {
        b.iter(bench_translate_run_twice)
    });
    group.finish();
}

fn bench_translate() {
    let mut world = World::init(Instant::now);

    #[derive(Debug)]
    struct A {}

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            for _ in 0..2000 {
                ctx.translate((10.px(), 10.px())).add(B {});
            }
        }
    }

    #[derive(Debug)]
    struct B {}

    impl Component for B {
        fn render(self, _ctx: &RenderCtx) {}
    }

    World::run(&mut world, A {});
}
fn bench_translate_run_twice() {
    let mut world = World::init(Instant::now);

    #[derive(Debug)]
    struct A {}

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            for _ in 0..2000 {
                ctx.translate((10.px(), 10.px())).add(B {});
            }
        }
    }

    #[derive(Debug)]
    struct B {}

    impl Component for B {
        fn render(self, _ctx: &RenderCtx) {}
    }

    World::run(&mut world, A {});
    World::run(&mut world, A {});
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);
