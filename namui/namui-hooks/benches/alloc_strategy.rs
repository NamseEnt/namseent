use bumpalo::Bump;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

enum StdTree {
    Node { data: Vec<u32>, x: f32, y: f32 },
    Translate(Box<StdTree>),
    Children(Vec<StdTree>),
}

fn build_std(n: usize) -> StdTree {
    let mut children = Vec::with_capacity(n);
    for i in 0..n {
        let leaf = StdTree::Node {
            data: vec![i as u32, 0],
            x: i as f32,
            y: 0.0,
        };
        children.push(StdTree::Translate(Box::new(leaf)));
    }
    StdTree::Children(children)
}

fn sum_std(tree: &StdTree) -> f32 {
    match tree {
        StdTree::Node { x, y, data } => x + y + data.len() as f32,
        StdTree::Translate(inner) => sum_std(inner),
        StdTree::Children(children) => children.iter().map(sum_std).sum(),
    }
}

enum BumpTree<'b> {
    Node { data: &'b [u32], x: f32, y: f32 },
    Translate(&'b BumpTree<'b>),
    Children(&'b [BumpTree<'b>]),
}

fn build_bump<'b>(bump: &'b Bump, n: usize) -> BumpTree<'b> {
    let children = bump.alloc_slice_fill_iter((0..n).map(|i| {
        let data: &[u32] = bump.alloc_slice_copy(&[i as u32, 0]);
        let leaf = bump.alloc(BumpTree::Node {
            data,
            x: i as f32,
            y: 0.0,
        });
        BumpTree::Translate(leaf)
    }));
    BumpTree::Children(children)
}

fn sum_bump(tree: &BumpTree) -> f32 {
    match tree {
        BumpTree::Node { x, y, data } => x + y + data.len() as f32,
        BumpTree::Translate(inner) => sum_bump(inner),
        BumpTree::Children(children) => children.iter().map(sum_bump).sum(),
    }
}

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("alloc_strategy");
    group.sample_size(50);

    for n in [1_000usize, 10_000] {
        group.throughput(Throughput::Elements(n as u64));

        group.bench_with_input(BenchmarkId::new("std_box_vec", n), &n, |b, &n| {
            b.iter(|| {
                let tree = build_std(n);
                std::hint::black_box(sum_std(&tree));
            });
        });

        let mut bump = Bump::new();
        build_bump(&bump, n);
        group.bench_with_input(BenchmarkId::new("bump_arena", n), &n, |b, &n| {
            b.iter(|| {
                bump.reset();
                let tree = build_bump(&bump, n);
                std::hint::black_box(sum_bump(&tree));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
