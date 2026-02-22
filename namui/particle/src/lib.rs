use arc_swap::ArcSwap;
use arrayvec::ArrayVec;
use crossbeam_queue::SegQueue;
use namui_hooks::*;
use namui_rendering_tree::*;
use namui_type::*;
use std::sync::{Arc, OnceLock};
use std::thread::Thread;

pub type ParticleSprites = ArrayVec<ImageSprite, 8>;

pub trait Particle: Send + Sync + 'static + Sized {
    fn tick(&mut self, now: Instant, dt: Duration);
    fn render(&self) -> ParticleSprites;
    fn is_done(&self, now: Instant) -> bool;
}

enum EmitterMsg<P> {
    Spawn(P),
    Tick { now: Instant, dt: Duration },
}

pub struct Emitter<P: Particle> {
    inner: OnceLock<EmitterInner<P>>,
}

struct EmitterInner<P: Particle> {
    queue: Arc<SegQueue<EmitterMsg<P>>>,
    worker_thread: Thread,
    rendered_sprites: Arc<ArcSwap<Vec<ImageSprite>>>,
}

impl<P: Particle> Emitter<P> {
    pub const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }

    pub fn spawn(&self, particle: P) {
        self.init();
        let inner = self.inner.get().unwrap();
        inner.queue.push(EmitterMsg::Spawn(particle));
        inner.worker_thread.unpark();
    }

    pub fn tick(&self, now: Instant, dt: Duration) {
        self.init();
        let inner = self.inner.get().unwrap();
        inner.queue.push(EmitterMsg::Tick { now, dt });
        inner.worker_thread.unpark();
    }

    fn init(&self) {
        self.inner.get_or_init(|| {
            let queue = Arc::new(SegQueue::new());
            let rendered_sprites = Arc::new(ArcSwap::from_pointee(Vec::new()));

            let queue_clone = queue.clone();
            let rendered_sprites_clone = rendered_sprites.clone();

            let handle = std::thread::Builder::new()
                .stack_size(64 * 1024)
                .spawn(move || {
                    tick_thread_main(queue_clone, rendered_sprites_clone);
                })
                .expect("failed to spawn emitter thread");

            EmitterInner {
                queue,
                worker_thread: handle.thread().clone(),
                rendered_sprites,
            }
        });
    }

    pub fn latest_sprites(&self) -> Arc<Vec<ImageSprite>> {
        self.inner
            .get()
            .map(|inner| inner.rendered_sprites.load_full())
            .unwrap_or_else(|| Arc::new(Vec::new()))
    }
}

unsafe impl<P: Particle> Sync for Emitter<P> {}

pub struct RenderEmitter<'a, P: Particle> {
    pub emitter: &'a Emitter<P>,
    pub image: Image,
    pub sprite_colors_blend_mode: BlendMode,
    pub paint: Option<Paint>,
}

impl<P: Particle> Component for RenderEmitter<'_, P> {
    fn render(self, ctx: &RenderCtx) {
        self.emitter.init();
        let sprites = self.emitter.latest_sprites();
        if sprites.is_empty() {
            return;
        }
        let cloned = Arc::unwrap_or_clone(sprites);
        ctx.add(RenderingTree::Node(DrawCommand::Image {
            command: Box::new(ImageDrawCommand {
                image: self.image,
                sprites: cloned,
                paint: self.paint,
                sprite_colors_blend_mode: self.sprite_colors_blend_mode,
            }),
        }));
    }
}

fn tick_thread_main<P: Particle>(
    queue: Arc<SegQueue<EmitterMsg<P>>>,
    rendered_sprites: Arc<ArcSwap<Vec<ImageSprite>>>,
) {
    let _ = rayon::ThreadPoolBuilder::new()
        .spawn_handler(|thread| {
            let mut builder = std::thread::Builder::new();
            if let Some(name) = thread.name() {
                builder = builder.name(namui_hooks::Dependencies::to_owned(&name).to_string());
            }
            if let Some(stack_size) = thread.stack_size() {
                builder = builder.stack_size(stack_size);
            }
            builder.spawn(move || {
                thread.run();
            })?;
            Ok(())
        })
        .build_global();

    let mut particles: Vec<P> = Vec::with_capacity(256);

    loop {
        while let Some(msg) = queue.pop() {
            match msg {
                EmitterMsg::Spawn(p) => {
                    particles.push(p);
                }
                EmitterMsg::Tick { now, dt } => {
                    use rayon::prelude::*;
                    particles.par_iter_mut().for_each(|p| p.tick(now, dt));
                    particles.retain(|p| !p.is_done(now));
                    let sprites: Vec<ImageSprite> =
                        particles.par_iter().flat_map_iter(|p| p.render()).collect();
                    rendered_sprites.store(Arc::new(sprites));
                }
            }
        }
        std::thread::park();
    }
}
