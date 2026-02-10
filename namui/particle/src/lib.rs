use namui_hooks::*;
use namui_rendering_tree::*;
use namui_type::*;
use orx_parallel::*;
use std::sync::{
    Arc, Mutex, OnceLock,
    atomic::{AtomicBool, Ordering},
    mpsc::{Receiver, Sender},
};

pub trait Particle: Send + Sync + 'static + Sized {
    fn tick(&mut self, now: Instant, dt: Duration);
    fn render(&self) -> RenderingTree;
    fn is_done(&self, now: Instant) -> bool;
}

pub struct Emitter<P: Particle> {
    inner: OnceLock<EmitterInner<P>>,
}

struct EmitterInner<P: Particle> {
    spawn_tx: Sender<P>,
    rendered_tree: Arc<Mutex<Arc<RenderingTree>>>,
    stop_flag: Arc<AtomicBool>,
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
            let rendered_tree = Arc::new(Mutex::new(Arc::new(RenderingTree::Empty)));
            let stop_flag = Arc::new(AtomicBool::new(false));

            let rendered_tree_clone = rendered_tree.clone();
            let stop_flag_clone = stop_flag.clone();

            std::thread::spawn(move || {
                tick_thread_main(spawn_rx, &rendered_tree_clone, &stop_flag_clone);
            });

            EmitterInner {
                spawn_tx,
                rendered_tree,
                stop_flag,
            }
        });
    }

    fn latest_tree(&self) -> Arc<RenderingTree> {
        self.inner
            .get()
            .map(|inner| inner.rendered_tree.lock().unwrap().clone())
            .unwrap_or_else(|| Arc::new(RenderingTree::Empty))
    }
}

unsafe impl<P: Particle> Sync for Emitter<P> {}

impl<P: Particle> Component for &Emitter<P> {
    fn render(self, ctx: &RenderCtx) {
        self.init();

        let tree = self.latest_tree();
        ctx.add(Arc::unwrap_or_clone(tree));
    }
}

fn tick_thread_main<P: Particle>(
    rx: Receiver<P>,
    rendered_tree: &Mutex<Arc<RenderingTree>>,
    stop_flag: &AtomicBool,
) {
    let mut particles: Vec<P> = Vec::with_capacity(65536);
    let mut last_now = Instant::now();

    loop {
        if stop_flag.load(Ordering::Acquire) {
            break;
        }

        let now = Instant::now();
        let dt = now - last_now;

        while let Ok(p) = rx.try_recv() {
            particles.push(p);
        }

        particles.par_mut().for_each(|p| p.tick(now, dt));

        particles.retain(|p| !p.is_done(now));

        let tree = if particles.is_empty() {
            RenderingTree::Empty
        } else {
            let trees: Vec<RenderingTree> = particles.par().map(|p| p.render()).collect();
            RenderingTree::Children(trees)
        };
        *rendered_tree.lock().unwrap() = Arc::new(tree);

        last_now = now;
        std::thread::sleep(std::time::Duration::from_millis(8));
    }
}
