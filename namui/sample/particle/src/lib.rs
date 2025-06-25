use namui::*;
use namui_prebuilt::*;

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        let (system, set_system) = ctx.state(|| {
            Some(particle::System::new(vec![Emitter::Rocket {
                emitter: RocketEmitter::new(),
            }]))
        });

        if let Some(system) = system.as_ref() {
            system.render(ctx, namui::time::now());

            if system.is_done(namui::time::now()) {
                set_system.set(None);

                println!("done, new system created with fire_and_forget");
                particle::fire_and_forget(particle::System::new(vec![Emitter::Rocket {
                    emitter: RocketEmitter::new(),
                }]));
            }
        }

        ctx.add(simple_rect(
            Wh::new(1000.px(), 1000.px()),
            Color::BLACK,
            0.px(),
            Color::BLACK,
        ));
    })
}

const GRAVITY: Xy<Px> = Xy::new(px(0.), px(9.8));

struct RocketEmitter {
    acc_time: Duration,
    num_emitted: usize,
}
impl RocketEmitter {
    fn new() -> Self {
        Self {
            acc_time: Duration::ZERO,
            num_emitted: 0,
        }
    }
    fn emit(&mut self, _now: Instant, dt: Duration) -> Vec<Particle> {
        self.acc_time += dt;
        if self.acc_time < Duration::from_secs(3) {
            return vec![];
        }
        self.num_emitted += 1;
        self.acc_time = Duration::ZERO;
        vec![Particle::Rocket {
            particle: RocketParticle {
                xy: Xy::new(100.px(), 500.px()),
                v_xy: Xy::new(0.px(), -100.px()),
                a_xy: Xy::new(0.px(), 9.8.px()),
                ttl: 2.sec(),
            },
        }]
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.num_emitted >= 2
    }
}
struct RocketParticle {
    xy: Xy<Px>,
    v_xy: Xy<Px>,
    a_xy: Xy<Px>,
    ttl: Duration,
}
impl RocketParticle {
    fn tick(&mut self, _now: Instant, dt: Duration) -> Vec<Emitter> {
        self.ttl -= dt;
        if self.ttl <= Duration::ZERO {
            return vec![Emitter::Flame {
                emitter: FlameEmitter::new(self.xy),
            }];
        }

        self.xy += self.v_xy * dt.as_secs_f32();
        self.v_xy += self.a_xy * dt.as_secs_f32();

        vec![]
    }

    fn render(&self) -> RenderingTree {
        let path = Path::new().add_rect(Rect::from_xy_wh(Xy::zero(), Wh::single(4.px())));
        let paint = Paint::new(Color::RED).set_style(PaintStyle::Fill);

        namui::translate(self.xy.x, self.xy.y, namui::path(path, paint))
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.ttl <= Duration::ZERO
    }
}
struct FlameEmitter {
    xy: Xy<Px>,
    emitted: bool,
}
impl FlameEmitter {
    fn new(xy: Xy<Px>) -> Self {
        Self { xy, emitted: false }
    }
    fn emit(&mut self, _now: Instant, _dt: Duration) -> Vec<Particle> {
        if self.emitted {
            return vec![];
        }

        self.emitted = true;

        const NUM_FLAME_PARTICLES: usize = 50;
        (0..NUM_FLAME_PARTICLES)
            .map(|i| {
                let degree = (i as f32 / NUM_FLAME_PARTICLES as f32) * 2.0 * std::f32::consts::PI;
                let v_xy = Xy::new(degree.cos().px(), degree.sin().px()) * 100.0;
                Particle::Flame {
                    particle: FlameParticle {
                        xy: self.xy,
                        v_xy,
                        a_xy: GRAVITY,
                        ttl: Duration::from_secs(3),
                    },
                }
            })
            .collect()
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.emitted
    }
}
struct FlameParticle {
    xy: Xy<Px>,
    v_xy: Xy<Px>,
    a_xy: Xy<Px>,
    ttl: Duration,
}
impl FlameParticle {
    fn tick(&mut self, _now: Instant, dt: Duration) -> Vec<Emitter> {
        self.ttl -= dt;
        self.xy += self.v_xy * dt.as_secs_f32();
        self.v_xy += self.a_xy * dt.as_secs_f32();

        vec![]
    }

    fn render(&self) -> RenderingTree {
        let path = Path::new().add_rect(Rect::from_xy_wh(Xy::zero(), Wh::single(2.px())));
        let paint = Paint::new(Color::from_u8(
            255,
            255,
            0,
            (self.ttl.as_secs_f32() / 3.0 * 255.0) as u8,
        ))
        .set_style(PaintStyle::Fill);

        namui::translate(self.xy.x, self.xy.y, namui::path(path, paint))
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.ttl <= Duration::ZERO
    }
}

enum Emitter {
    Rocket { emitter: RocketEmitter },
    Flame { emitter: FlameEmitter },
}

enum Particle {
    Rocket { particle: RocketParticle },
    Flame { particle: FlameParticle },
}

impl particle::Emitter<Particle> for Emitter {
    fn emit(&mut self, now: Instant, dt: Duration) -> Vec<Particle> {
        match self {
            Emitter::Rocket { emitter } => emitter.emit(now, dt),
            Emitter::Flame { emitter } => emitter.emit(now, dt),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            Emitter::Rocket { emitter } => emitter.is_done(now),
            Emitter::Flame { emitter } => emitter.is_done(now),
        }
    }
}

impl particle::Particle<Emitter> for Particle {
    fn tick(&mut self, now: Instant, dt: Duration) -> Vec<Emitter> {
        match self {
            Particle::Rocket { particle } => particle.tick(now, dt),
            Particle::Flame { particle } => particle.tick(now, dt),
        }
    }

    fn render(&self) -> RenderingTree {
        match self {
            Particle::Rocket { particle } => particle.render(),
            Particle::Flame { particle } => particle.render(),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            Particle::Rocket { particle } => particle.is_done(now),
            Particle::Flame { particle } => particle.is_done(now),
        }
    }
}
