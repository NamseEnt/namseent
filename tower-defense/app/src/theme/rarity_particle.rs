use crate::animation::CubicBezier;
use crate::{PresentationDelta, PresentationInstant, Rarity};
use namui::*;
use rand::Rng;
use std::f32::consts::TAU;

const SPRITE_SIZE_PX: f32 = 128.0;
const LIFETIME_MIN_MS: i64 = 1600;
const LIFETIME_MAX_MS: i64 = 3600;
const MIN_DISTANCE_MULTIPLIER: f32 = 0.55;
const MAX_DISTANCE_MULTIPLIER: f32 = 1.0;
const PARTICLE_SIZE_MIN_PX: f32 = 16.0;
const PARTICLE_SIZE_MAX_PX: f32 = 48.0;
const MAX_ALPHA_MIN: f32 = 0.25;
const MAX_ALPHA_MAX: f32 = 0.75;
const ROTATION_TURNS_MIN: f32 = 0.75;
const ROTATION_TURNS_MAX: f32 = 1.5;
const SPAWN_INTERVAL: PresentationDelta = PresentationDelta::from_nanos(180_000_000);
const PARTICLE_EASE_OUT: CubicBezier = CubicBezier::new(0.15, 0.85, 0.0, 1.0);
const PARTICLE_OPACITY_EASE_IN: CubicBezier = CubicBezier::new(0.0, 0.0, 1.0, 0.75);

#[derive(Clone, Copy, PartialEq, Eq)]
enum RarityParticleKind {
    Fill,
    Stroke,
}

pub struct RarityParticle {
    xy: Xy<Px>,
    src_rect: Rect<Px>,
    start_xy: Xy<Px>,
    direction: (f32, f32),
    travel_distance: f32,
    initial_rotation_rad: f32,
    rotation_distance_rad: f32,
    created_at: Instant,
    lifetime: Duration,
    max_alpha: f32,
    max_size_px: f32,
    scale: f32,
    alpha: f32,
    rotation_rad: f32,
}

impl RarityParticle {
    fn new<R: Rng + ?Sized>(
        xy: Xy<Px>,
        radius: Px,
        rarity: Rarity,
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        let radius = radius.as_f32().max(1.0);
        let kind = if rng.gen_bool(0.5) {
            RarityParticleKind::Fill
        } else {
            RarityParticleKind::Stroke
        };
        let src_rect = rarity_particle_rect(rarity, kind);
        let angle = rng.gen_range(0.0..TAU);
        let direction = (angle.cos(), angle.sin());
        let min_distance = (radius * MIN_DISTANCE_MULTIPLIER).max(1.0);
        let max_distance = (radius * MAX_DISTANCE_MULTIPLIER).max(min_distance + 1.0);
        let lifetime_secs =
            rng.gen_range(LIFETIME_MIN_MS as f32 / 1_000.0..=LIFETIME_MAX_MS as f32 / 1_000.0);
        let travel_distance = rng.gen_range(min_distance..=max_distance);
        let max_alpha = rng.gen_range(MAX_ALPHA_MIN..=MAX_ALPHA_MAX);
        let max_size_px = rng.gen_range(PARTICLE_SIZE_MIN_PX..=PARTICLE_SIZE_MAX_PX);
        let initial_rotation_rad = rng.gen_range(0.0..TAU);
        let rotation_turns = rng.gen_range(ROTATION_TURNS_MIN..=ROTATION_TURNS_MAX)
            * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        let rotation_distance_rad = rotation_turns * TAU;

        Self {
            xy,
            src_rect,
            start_xy: xy,
            direction,
            travel_distance,
            initial_rotation_rad,
            rotation_distance_rad,
            created_at,
            lifetime: Duration::from_secs_f32(lifetime_secs),
            max_alpha,
            max_size_px,
            scale: 0.0,
            alpha: max_alpha,
            rotation_rad: initial_rotation_rad,
        }
    }

    fn progress(&self, now: Instant) -> f32 {
        ((now - self.created_at).as_secs_f32() / self.lifetime.as_secs_f32()).clamp(0.0, 1.0)
    }

    fn tick_impl(&mut self, now: Instant, _dt: Duration) {
        let progress = self.progress(now);
        let eased_progress = PARTICLE_EASE_OUT.sample(progress);
        let opacity_progress = PARTICLE_OPACITY_EASE_IN.sample(progress);
        let distance = self.travel_distance * eased_progress;
        self.xy = Xy::new(
            self.start_xy.x + px(self.direction.0 * distance),
            self.start_xy.y + px(self.direction.1 * distance),
        );
        self.alpha = self.max_alpha * (1.0 - opacity_progress);
        self.scale = self.max_size_px * eased_progress / SPRITE_SIZE_PX;
        self.rotation_rad = self.initial_rotation_rad + self.rotation_distance_rad * eased_progress;
    }

    fn render_impl(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.alpha <= 0.0 || self.scale <= 0.0 {
            return sprites;
        }

        let color = Color::WHITE.with_alpha((self.alpha * 255.0).round() as u8);
        sprites.push(centered_rotated_sprite(
            self.src_rect,
            self.xy.x,
            self.xy.y,
            self.scale,
            self.rotation_rad,
            Some(color),
        ));
        sprites
    }
}

impl namui::particle::Particle for RarityParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        self.render_impl()
    }

    fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.lifetime
    }
}

pub struct RarityParticleEffect {
    pub xy: Xy<Px>,
    pub radius: Px,
    pub rarity: Rarity,
    pub strength: f32,
    pub enabled: bool,
    pub presentation_instant: PresentationInstant,
}

impl RarityParticleEffect {
    pub fn from_wh(
        wh: Wh<Px>,
        rarity: Rarity,
        strength: f32,
        enabled: bool,
        presentation_instant: PresentationInstant,
    ) -> Self {
        Self {
            xy: wh.to_xy() * 0.5,
            radius: (wh.width + wh.height) * 0.25,
            rarity,
            strength,
            enabled,
            presentation_instant,
        }
    }
}

impl Component for RarityParticleEffect {
    fn render(self, ctx: &RenderCtx) {
        let (emitter, _) = ctx.state(namui::particle::Emitter::<RarityParticle>::new);
        let (last_particle_spawn, set_last_particle_spawn) =
            ctx.state(|| None::<PresentationInstant>);

        if self.enabled {
            let should_spawn = match *last_particle_spawn {
                None => true,
                Some(last_spawn) => {
                    self.presentation_instant.delta_since(last_spawn) >= SPAWN_INTERVAL
                }
            };

            if should_spawn {
                let xy = self.xy;
                let radius = self.radius;
                let strength = self.strength;
                let rarity = self.rarity;
                let created_at = self.presentation_instant.as_namui();
                let expected_particle_count = strength.clamp(0.0, 2.0);
                let whole_particle_count = expected_particle_count.floor() as usize;
                let fractional_particle_count = expected_particle_count.fract();

                emitter.spawn_batch(move |particles| {
                    let mut rng = rand::thread_rng();
                    let particle_count = whole_particle_count
                        + usize::from(rng.gen_bool(fractional_particle_count as f64));
                    for _ in 0..particle_count {
                        particles.push(RarityParticle::new(
                            xy, radius, rarity, created_at, &mut rng,
                        ));
                    }
                });
                set_last_particle_spawn.set(Some(self.presentation_instant));
            }
        }

        ctx.interval("rarity particle emitter", Duration::from_millis(16), |dt| {
            emitter.tick(self.presentation_instant.as_namui(), dt)
        });

        ctx.add(namui::particle::RenderEmitter {
            emitter: &*emitter,
            image: crate::asset::image::ui::particle::RARITY,
            sprite_colors_blend_mode: BlendMode::Modulate,
            paint: Some(Paint::new(Color::WHITE).set_blend_mode(BlendMode::Screen)),
        });
    }
}

fn rarity_particle_rect(rarity: Rarity, kind: RarityParticleKind) -> Rect<Px> {
    let (column, row) = match (rarity, kind) {
        (Rarity::Common, RarityParticleKind::Fill) => (0.0, 0.0),
        (Rarity::Common, RarityParticleKind::Stroke) => (1.0, 0.0),
        (Rarity::Rare, RarityParticleKind::Fill) => (2.0, 0.0),
        (Rarity::Rare, RarityParticleKind::Stroke) => (3.0, 0.0),
        (Rarity::Epic, RarityParticleKind::Fill) => (0.0, 1.0),
        (Rarity::Epic, RarityParticleKind::Stroke) => (1.0, 1.0),
        (Rarity::Legendary, RarityParticleKind::Fill) => (2.0, 1.0),
        (Rarity::Legendary, RarityParticleKind::Stroke) => (3.0, 1.0),
    };
    Rect::Xywh {
        x: px(column * SPRITE_SIZE_PX),
        y: px(row * SPRITE_SIZE_PX),
        width: px(SPRITE_SIZE_PX),
        height: px(SPRITE_SIZE_PX),
    }
}

fn centered_rotated_sprite(
    src_rect: Rect<Px>,
    cx: Px,
    cy: Px,
    scale: f32,
    angle_rad: f32,
    color: Option<Color>,
) -> ImageSprite {
    let sw = src_rect.width().as_f32();
    let sh = src_rect.height().as_f32();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    let scos = scale * cos_a;
    let ssin = scale * sin_a;
    ImageSprite {
        src_rect,
        xform: RSXform {
            scos,
            ssin,
            tx: cx - px(scos * sw / 2.0 - ssin * sh / 2.0),
            ty: cy - px(ssin * sw / 2.0 + scos * sh / 2.0),
        },
        color,
    }
}
