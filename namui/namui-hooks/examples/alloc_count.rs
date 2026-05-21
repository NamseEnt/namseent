use namui_hooks::*;
use namui_rendering_tree::{Color, Paint, Path, PathDrawCommand};
use namui_type::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
static MEASURING: AtomicUsize = AtomicUsize::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if MEASURING.load(Ordering::Relaxed) == 1 {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        }
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static COUNTING: Counting = Counting;

fn measure<R>(f: impl FnOnce() -> R) -> (usize, usize, R) {
    let allocs_before = ALLOCS.load(Ordering::Relaxed);
    let bytes_before = BYTES.load(Ordering::Relaxed);
    MEASURING.store(1, Ordering::Relaxed);
    let result = f();
    MEASURING.store(0, Ordering::Relaxed);
    let allocs = ALLOCS.load(Ordering::Relaxed) - allocs_before;
    let bytes = BYTES.load(Ordering::Relaxed) - bytes_before;
    (allocs, bytes, result)
}

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

fn report(name: &str, entities: usize, factory: impl Fn() -> Box<dyn FnOnce(&mut World)>) {
    let mut world = World::init(Instant::now);

    let run_create = factory();
    let (create_allocs, create_bytes, _) = measure(|| run_create(&mut world));

    for _ in 0..3 {
        factory()(&mut world);
    }

    let run_rerender = factory();
    let (rerender_allocs, rerender_bytes, _) = measure(|| run_rerender(&mut world));

    println!(
        "{name:<22} entities={entities:>6}  \
         create: {create_allocs:>8} allocs / {:>5} KB ({:>5.1}/entity)  \
         rerender: {rerender_allocs:>8} allocs / {:>5} KB ({:>5.1}/entity)",
        create_bytes / 1024,
        create_allocs as f64 / entities as f64,
        rerender_bytes / 1024,
        rerender_allocs as f64 / entities as f64,
    );
}

fn main() {
    println!("allocation count per World::run (counted only inside run)\n");

    for count in [1_000usize, 10_000] {
        report(&format!("flat"), count, || {
            Box::new(move |w: &mut World| {
                std::hint::black_box(World::run(w, FlatRoot { count }));
            })
        });
    }
    for count in [1_000usize, 10_000] {
        report(&format!("translated"), count, || {
            Box::new(move |w: &mut World| {
                std::hint::black_box(World::run(w, TranslatedRoot { count }));
            })
        });
    }
    for count in [1_000usize, 10_000] {
        report(&format!("stateful"), count, || {
            Box::new(move |w: &mut World| {
                std::hint::black_box(World::run(w, StatefulRoot { count }));
            })
        });
    }
    for (depth, breadth, leaves) in [(3usize, 10usize, 1_000usize), (4, 10, 10_000)] {
        report(&format!("nested"), leaves, || {
            Box::new(move |w: &mut World| {
                std::hint::black_box(World::run(w, NestedNode { depth, breadth }));
            })
        });
    }
}
