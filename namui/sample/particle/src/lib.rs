use namui::*;
use namui_prebuilt::*;

static ROCKETS: particle::Emitter<RocketParticle> = particle::Emitter::new();
static FLAMES: particle::Emitter<FlameParticle> = particle::Emitter::new();

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        let (spawned, set_spawned) = ctx.state(|| false);

        if !*spawned {
            set_spawned.set(true);
            ROCKETS.spawn(RocketParticle {
                xy: Xy::new(100.px(), 500.px()),
                v_xy: Xy::new(0.px(), -100.px()),
                a_xy: Xy::new(0.px(), 9.8.px()),
                ttl: 2.sec(),
            });
        }

        ctx.add(&ROCKETS);
        ctx.add(&FLAMES);

        ctx.add(simple_rect(
            Wh::new(1000.px(), 1000.px()),
            Color::BLACK,
            0.px(),
            Color::BLACK,
        ));
    })
}

const GRAVITY: Xy<Px> = Xy::new(px(0.), px(9.8));

struct RocketParticle {
    xy: Xy<Px>,
    v_xy: Xy<Px>,
    a_xy: Xy<Px>,
    ttl: Duration,
}

impl particle::Particle for RocketParticle {
    fn tick(&mut self, _now: Instant, dt: Duration) {
        self.ttl -= dt;
        if self.ttl <= Duration::ZERO {
            FLAMES.spawn(FlameParticle::new(self.xy));
            return;
        }
        self.xy += self.v_xy * dt.as_secs_f32();
        self.v_xy += self.a_xy * dt.as_secs_f32();
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

struct FlameParticle {
    xy: Xy<Px>,
    v_xy: Xy<Px>,
    a_xy: Xy<Px>,
    ttl: Duration,
}

impl FlameParticle {
    fn new(xy: Xy<Px>) -> Self {
        Self {
            xy,
            v_xy: Xy::zero(),
            a_xy: GRAVITY,
            ttl: Duration::from_secs(3),
        }
    }
}

impl particle::Particle for FlameParticle {
    fn tick(&mut self, _now: Instant, dt: Duration) {
        self.ttl -= dt;
        self.xy += self.v_xy * dt.as_secs_f32();
        self.v_xy += self.a_xy * dt.as_secs_f32();
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
