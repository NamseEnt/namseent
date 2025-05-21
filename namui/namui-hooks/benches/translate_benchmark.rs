use criterion::{Criterion, criterion_group, criterion_main};
use namui_hooks::*;
use namui_skia::*;
use namui_type::*;
use std::sync::Arc;

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
    let mut world = World::init(Instant::now, Arc::new(MockSkCalculate));

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
    let mut world = World::init(Instant::now, Arc::new(MockSkCalculate));

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

struct MockSkCalculate;
impl SkCalculate for MockSkCalculate {
    fn group_glyph(&self, _font: &Font, _paint: &Paint) -> Arc<dyn GroupGlyph> {
        unimplemented!()
    }

    fn font_metrics(&self, _font: &Font) -> Option<FontMetrics> {
        unimplemented!()
    }

    fn load_typeface(&self, _typeface_name: String, _bytes: Vec<u8>) -> JoinHandle<Result<()>> {
        unimplemented!()
    }

    fn path_contains_xy(&self, _path: &Path, _paint: Option<&Paint>, _xy: Xy<Px>) -> bool {
        unimplemented!()
    }

    fn path_bounding_box(&self, _path: &Path, _paint: Option<&Paint>) -> Option<Rect<Px>> {
        unimplemented!()
    }

    fn load_image_from_raw(&self, _image_info: ImageInfo, _bitmap: &[u8]) -> JoinHandle<Image> {
        unimplemented!()
    }

    fn load_image_from_encoded(&self, _bytes: &[u8]) -> JoinHandle<Image> {
        todo!()
    }
}
