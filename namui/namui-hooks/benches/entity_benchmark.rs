use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use namui_hooks::*;
use namui_rendering_tree::{Color, Paint, Path, PathDrawCommand};
use namui_type::*;

#[derive(Debug)]
struct Entity {
    index: usize,
}
impl Component for Entity {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(PathDrawCommand {
            path: Path::new().add_rect(Rect::Xywh {
                x: (self.index as f32).px(),
                y: 0.px(),
                width: 10.px(),
                height: 10.px(),
            }),
            paint: Paint::new(Color::from_u8(200, 100, 50, 255)),
        });
    }
}

#[derive(Debug)]
struct StatefulEntity {
    index: usize,
}
impl Component for StatefulEntity {
    fn render(self, ctx: &RenderCtx) {
        let (count, _set_count) = ctx.state(|| 0usize);
        let doubled = ctx.memo(|| *count * 2);
        ctx.add(PathDrawCommand {
            path: Path::new().add_rect(Rect::Xywh {
                x: (self.index as f32).px(),
                y: (*doubled as f32).px(),
                width: 10.px(),
                height: 10.px(),
            }),
            paint: Paint::new(Color::from_u8(200, 100, 50, 255)),
        });
    }
}

#[derive(Debug)]
struct FlatRoot {
    count: usize,
}
impl Component for FlatRoot {
    fn render(self, ctx: &RenderCtx) {
        for i in 0..self.count {
            ctx.add(Entity { index: i });
        }
    }
}

#[derive(Debug)]
struct TranslatedRoot {
    count: usize,
}
impl Component for TranslatedRoot {
    fn render(self, ctx: &RenderCtx) {
        for i in 0..self.count {
            ctx.translate(((i as f32).px(), 0.px()))
                .add(Entity { index: i });
        }
    }
}

#[derive(Debug)]
struct StatefulRoot {
    count: usize,
}
impl Component for StatefulRoot {
    fn render(self, ctx: &RenderCtx) {
        for i in 0..self.count {
            ctx.add(StatefulEntity { index: i });
        }
    }
}

#[derive(Debug, Clone)]
struct NestedNode {
    depth: usize,
    breadth: usize,
}
impl Component for NestedNode {
    fn render(self, ctx: &RenderCtx) {
        if self.depth == 0 {
            ctx.add(Entity { index: self.breadth });
            return;
        }
        for _ in 0..self.breadth {
            ctx.add(NestedNode {
                depth: self.depth - 1,
                breadth: self.breadth,
            });
        }
    }
}

const COUNTS: [usize; 3] = [1_000, 5_000, 10_000];

fn bench_create<C: Component>(
    c: &mut Criterion,
    group_name: &str,
    mut factory: impl FnMut(usize) -> C,
) {
    let mut group = c.benchmark_group(group_name);
    group.sample_size(20);
    for count in COUNTS {
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            b.iter(|| {
                let mut world = World::init(Instant::now);
                std::hint::black_box(World::run(&mut world, factory(count)));
            });
        });
    }
    group.finish();
}

fn bench_rerender<C: Component>(
    c: &mut Criterion,
    group_name: &str,
    mut factory: impl FnMut(usize) -> C,
) {
    let mut group = c.benchmark_group(group_name);
    group.sample_size(20);
    for count in COUNTS {
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let mut world = World::init(Instant::now);
            World::run(&mut world, factory(count));
            b.iter(|| {
                std::hint::black_box(World::run(&mut world, factory(count)));
            });
        });
    }
    group.finish();
}

fn benchmarks(c: &mut Criterion) {
    bench_create(c, "create/flat", |count| FlatRoot { count });
    bench_rerender(c, "rerender/flat", |count| FlatRoot { count });

    bench_create(c, "create/translated", |count| TranslatedRoot { count });
    bench_rerender(c, "rerender/translated", |count| TranslatedRoot { count });

    bench_create(c, "create/stateful", |count| StatefulRoot { count });
    bench_rerender(c, "rerender/stateful", |count| StatefulRoot { count });

    let mut group = c.benchmark_group("nested");
    group.sample_size(20);
    for (depth, breadth, leaves) in [(3, 10, 1_000usize), (4, 10, 10_000usize)] {
        group.throughput(Throughput::Elements(leaves as u64));
        group.bench_with_input(
            BenchmarkId::new("create", leaves),
            &(depth, breadth),
            |b, &(depth, breadth)| {
                b.iter(|| {
                    let mut world = World::init(Instant::now);
                    std::hint::black_box(World::run(&mut world, NestedNode { depth, breadth }));
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("rerender", leaves),
            &(depth, breadth),
            |b, &(depth, breadth)| {
                let mut world = World::init(Instant::now);
                World::run(&mut world, NestedNode { depth, breadth });
                b.iter(|| {
                    std::hint::black_box(World::run(&mut world, NestedNode { depth, breadth }));
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
