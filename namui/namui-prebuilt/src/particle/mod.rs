use arc_swap::{ArcSwap, ArcSwapOption};
use namui::{rayon::prelude::*, *};
use std::sync::Arc;

pub trait Emitter<P> {
    fn emit(&mut self, now: Instant, dt: Duration) -> Vec<P>;
    fn is_done(&self, now: Instant) -> bool;
}

pub trait Particle<E> {
    fn tick(&mut self, now: Instant, dt: Duration) -> Vec<E>;
    fn render(&self) -> RenderingTree;
    fn is_done(&self, now: Instant) -> bool;
}

pub struct System<E, P> {
    _emitter: std::marker::PhantomData<E>,
    _particle: std::marker::PhantomData<P>,
    initial_emitters: ArcSwapOption<Vec<E>>,
}

impl<E, P> System<E, P>
where
    E: Emitter<P> + 'static + Send + Sync,
    P: Particle<E> + 'static + Send,
{
    pub fn new(emitters: Vec<E>) -> Self {
        Self {
            _emitter: std::marker::PhantomData,
            _particle: std::marker::PhantomData,
            initial_emitters: ArcSwapOption::new(Some(Arc::new(emitters))),
        }
    }
    pub fn render(&self, ctx: &ComposeCtx, now: Instant) {
        ctx.add(SystemComponent {
            now,
            initial_emitters: &self.initial_emitters,
            _p: std::marker::PhantomData,
        });
    }
}

struct SystemComponent<'a, E, P> {
    now: Instant,
    initial_emitters: &'a ArcSwapOption<Vec<E>>,
    _p: std::marker::PhantomData<P>,
}

impl<E, P> Component for SystemComponent<'_, E, P>
where
    E: Emitter<P> + 'static + Send + Sync,
    P: Particle<E> + 'static + Send,
{
    fn render(self, ctx: &RenderCtx) {
        let (req_tx, set_req_tx) = ctx.state(|| None);
        let rendering_trees_list = ctx.memo(|| Arc::new(ArcSwap::<_>::new(Default::default())));

        ctx.async_effect("run system on thread pool", (), {
            |()| {
                let (req_tx, mut req_rx) = namui::tokio::sync::watch::channel(self.now);
                set_req_tx.set(Some(req_tx));

                let rendering_trees_list = rendering_trees_list.clone_inner();
                let mut emitters: Vec<E> =
                    Arc::into_inner(self.initial_emitters.swap(None).unwrap()).unwrap();
                let mut particles: Vec<P> = Vec::with_capacity(65536);
                let mut last_now = self.now;

                async move {
                    while req_rx.changed().await.is_ok() {
                        let now = *req_rx.borrow_and_update();
                        let dt = now - last_now;

                        // NOTE: Assume emitters are not too many, so no need to use multithreading
                        emitters.retain_mut(|emitter| {
                            particles.extend(emitter.emit(now, dt));
                            !emitter.is_done(now)
                        });

                        let new_emitters = particles
                            .par_iter_mut()
                            .flat_map(|particle| particle.tick(now, dt))
                            .collect_vec_list();
                        emitters.extend(new_emitters.into_iter().flatten());

                        particles.retain_mut(|particle| !particle.is_done(now));

                        rendering_trees_list.store(Arc::new(
                            particles
                                .par_iter_mut()
                                .map(|particle| particle.render())
                                .collect_vec_list(),
                        ));

                        last_now = now;
                    }
                }
            }
        });

        ctx.attach_event(|_| {
            if let Some(req_tx) = req_tx.as_ref() {
                _ = req_tx.send(self.now);
            }
        });

        for rendering_tree in rendering_trees_list.load().iter().flatten().cloned() {
            ctx.add(rendering_tree);
        }
    }
}
