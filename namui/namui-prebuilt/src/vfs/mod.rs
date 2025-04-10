use namui::*;

// pub struct ParticleEffect {
//     pub xy: Xy<Px>,
// }

// fn test() {}

// pub struct Emitter<'a, Done, ParticleFn>
// where
//     Done: FnOnce(),
//     ParticleFn: Fn(ParticleFnParam) -> Particle,
// {
//     pub shape: EmitterShape,
//     pub emission_phases: &'a [EmissionPhase],
//     pub looping: bool,
//     pub now: Instant,
//     pub on_emission_done: Done,
//     pub particle_fn: ParticleFn,
// }

// impl<Done, ParticleFn> Component for Emitter<'_, Done, ParticleFn>
// where
//     Done: FnOnce(),
//     ParticleFn: Fn(ParticleFnParam) -> Particle,
// {
//     fn render(self, ctx: &RenderCtx) {
//         let Self {
//             shape,
//             emission_phases,
//             looping,
//             now,
//             on_emission_done,
//             particle_fn,
//         } = self;

//         let (last_time, set_last_time) = ctx.state(|| now);
//         let (counter, set_counter) = ctx.state(|| PhaseCounter {
//             index: 0,
//             elapsed: Duration::ZERO,
//         });
//         let (particles, set_particles) = ctx.state(Vec::new);

//         ctx.effect("update last_emit_time", || set_last_time.set(now));

//         ctx.effect("emit particle", || {
//             if emission_phases.get(counter.index).is_none() {
//                 return;
//             }

//             let mut counter = counter.clone_inner();
//             let new_particles = emit_particles(
//                 now - *last_time,
//                 emission_phases,
//                 &mut counter,
//                 looping,
//                 on_emission_done,
//             );

//             set_counter.set(counter);
//             if !new_particles.is_empty() {
//                 set_particles.mutate(|particles| particles.extend(new_particles));
//             }
//         });

//         for particle in particles.iter() {
//             // ctx.add
//         }
//     }
// }

// fn emit_particles<Done: FnOnce()>(
//     mut dt: Duration,
//     emission_phases: &[EmissionPhase],
//     counter: &mut PhaseCounter,
//     looping: bool,
//     on_emission_done: Done,
// ) -> Vec<ParticleDef> {
//     let mut new_particles = vec![];

//     while let Some(phase) = emission_phases.get(counter.index) {
//         let dt_in_phase = (phase.duration - counter.elapsed).min(dt);
//         dt -= dt_in_phase;

//         let expected_particle_count = dt_in_phase.as_secs_f32() * phase.particles_per_second;
//         let particle_count = (namui::rand_random::<f32>() * expected_particle_count) as usize;

//         // Emit particles
//         new_particles.reserve(particle_count);
//         for _ in 0..particle_count {
//             new_particles.push(ParticleDef {});
//         }

//         if dt <= Duration::ZERO {
//             counter.elapsed += dt_in_phase;
//             break;
//         }

//         counter.index += 1;
//         counter.elapsed = Duration::ZERO;

//         if counter.index < emission_phases.len() {
//             continue;
//         }

//         if looping {
//             counter.index = 0;
//             continue;
//         }

//         on_emission_done();
//         break;
//     }
//     new_particles
// }

// #[derive(Debug, Clone, Copy)]
// struct PhaseCounter {
//     index: usize,
//     elapsed: Duration,
// }

// #[derive(Debug)]
// pub struct EmissionPhase {
//     pub particles_per_second: f32,
//     pub duration: Duration,
// }

// #[derive(Debug)]
// pub enum EmitterShape {
//     Circle {
//         center: Xy<Px>,
//         radius: Px,
//         border_only: bool,
//     },
//     Polygon {
//         points: Vec<Xy<Px>>,
//         border_only: bool,
//     },
// }

// pub struct ParticleDef {
//     start_at: Instant,
//     end_at: Instant,
//     xy: Xy<Px>,
//     scale: Xy<f32>,
//     rotation: Angle,
// }

// pub struct ParticleFnParam {
//     pub start_particle: ParticleDef,
//     pub duration: Duration,
// }

// pub struct Particle {
//     xy: Xy<Px>,
//     scale: Xy<f32>,
//     rotation: Angle,
// }

// enum ParticleSprite {}

pub trait Emitter<P> {
    fn emit(&mut self, now: Instant, dt: Duration) -> Vec<P>;
    fn is_done(&self, now: Instant) -> bool;
}

pub trait Particle<E> {
    fn tick(&mut self, now: Instant, dt: Duration, on_new_emit: impl Fn(E));
    fn render(&self, ctx: &RenderCtx);
    fn is_done(&self, now: Instant) -> bool;
}

pub struct System<E, P> {
    emitters: Vec<E>,
    particles: Vec<P>,
}

impl<E, P> System<E, P>
where
    E: Emitter<P>,
    P: Particle<E>,
{
    pub fn new(emitters: Vec<E>) -> Self {
        Self {
            emitters,
            particles: vec![],
        }
    }
}

pub fn test() {}
