use namui_hooks::*;
use namui_rendering_tree::*;
use namui_type::*;
use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

pub trait Emitter<P> {
    fn emit(&mut self, now: Instant, dt: Duration) -> Vec<P>;
    fn is_done(&self, now: Instant) -> bool;
}

pub trait Particle<E> {
    fn tick(&mut self, now: Instant, dt: Duration) -> Vec<E>;
    fn render(&self) -> RenderingTree;
    fn is_done(&self, now: Instant) -> bool;
}

#[derive(State)]
pub struct System<E, P>
where
    E: State,
    P: State,
{
    _emitter: std::marker::PhantomData<E>,
    _particle: std::marker::PhantomData<P>,
    initial_emitters: RefCell<Vec<E>>,
    is_done: Arc<AtomicBool>,
}

impl<E, P> System<E, P>
where
    E: Emitter<P> + State,
    P: Particle<E> + State,
{
    pub fn new(emitters: Vec<E>) -> Self {
        Self {
            _emitter: std::marker::PhantomData,
            _particle: std::marker::PhantomData,
            initial_emitters: emitters.into(),
            is_done: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn render(&self, ctx: &ComposeCtx, now: Instant) {
        ctx.add(SystemComponent {
            now,
            initial_emitters: &self.initial_emitters,
            system_is_done: &self.is_done,
            _p: std::marker::PhantomData,
        });
    }

    pub fn is_done(&self, _now: Instant) -> bool {
        self.is_done.load(Ordering::Acquire)
    }
}

struct SystemComponent<'a, E, P> {
    now: Instant,
    initial_emitters: &'a RefCell<Vec<E>>,
    system_is_done: &'a Arc<AtomicBool>,
    _p: std::marker::PhantomData<P>,
}

impl<E, P> Component for SystemComponent<'_, E, P>
where
    E: Emitter<P> + State,
    P: Particle<E> + State,
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            now,
            initial_emitters,
            system_is_done,
            ..
        } = self;
        #[derive(State)]
        struct State<E, P>
        where
            E: bincode::Encode + bincode::Decode<()>,
            P: bincode::Encode + bincode::Decode<()>,
        {
            emitters: Vec<E>,
            particles: Vec<P>,
            last_now: Instant,
        }
        let (state, set_state) = ctx.state(|| State {
            emitters: initial_emitters.replace(vec![]),
            particles: Vec::<P>::with_capacity(65536),
            last_now: Instant::now(),
        });

        ctx.attach_event(|_| {
            let system_is_done = system_is_done.clone();
            set_state.mutate(move |state| {
                let &mut State {
                    ref mut emitters,
                    ref mut particles,
                    ref mut last_now,
                } = state;
                let dt = now - *last_now;

                emitters.retain_mut(|emitter| {
                    particles.extend(emitter.emit(now, dt));
                    !emitter.is_done(now)
                });

                let new_emitters = particles
                    .iter_mut()
                    .flat_map(|particle| particle.tick(now, dt));
                emitters.extend(new_emitters);

                particles.retain_mut(|particle| !particle.is_done(now));

                if emitters.is_empty() && particles.is_empty() {
                    system_is_done.store(true, Ordering::Release);
                }
            });
        });

        for particle in state.particles.iter() {
            ctx.add(particle.render());
        }
    }
}
