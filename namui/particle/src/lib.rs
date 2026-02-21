use arc_swap::ArcSwap;
use namui_hooks::*;
use namui_rendering_tree::*;
use namui_type::*;
use std::sync::{
    Arc, OnceLock,
    mpsc::{Receiver, Sender},
};

pub trait Particle: Send + Sync + 'static + Sized {
    fn tick(&mut self, now: Instant, dt: Duration);
    fn render(&self) -> Option<ImageSprite>;
    fn is_done(&self, now: Instant) -> bool;
}

pub struct Emitter<P: Particle> {
    inner: OnceLock<EmitterInner<P>>,
}

struct EmitterInner<P: Particle> {
    spawn_tx: Sender<P>,
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
        let _ = inner.spawn_tx.send(particle);
    }

    fn init(&self) {
        self.inner.get_or_init(|| {
            let (spawn_tx, spawn_rx) = std::sync::mpsc::channel();
            let rendered_sprites = Arc::new(ArcSwap::from_pointee(Vec::new()));

            let rendered_sprites_clone = rendered_sprites.clone();

            tokio::spawn(async move {
                tick_task_main(spawn_rx, rendered_sprites_clone).await;
            });

            EmitterInner {
                spawn_tx,
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

async fn tick_task_main<P: Particle>(
    rx: Receiver<P>,
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
    let mut last_now = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now - last_now;

        while let Ok(p) = rx.try_recv() {
            particles.push(p);
        }

        if !particles.is_empty() {
            let (tx, oneshot_rx) = tokio::sync::oneshot::channel();
            rayon::spawn(move || {
                use rayon::prelude::*;
                particles.par_iter_mut().for_each(|p| p.tick(now, dt));
                particles.retain(|p| !p.is_done(now));
                let sprites: Vec<ImageSprite> = particles
                    .par_iter()
                    .filter_map(|p| p.render())
                    .collect();
                let _ = tx.send((particles, sprites));
            });
            match oneshot_rx.await {
                Ok((returned_particles, sprites)) => {
                    particles = returned_particles;
                    rendered_sprites.store(Arc::new(sprites));
                }
                Err(_) => {
                    particles = Vec::new();
                }
            }
        } else {
            rendered_sprites.store(Arc::new(vec![]));
        }

        last_now = now;
        tokio::time::sleep(std::time::Duration::from_millis(8)).await;
    }
}
