use crate::MapCoordF32;
use crate::game_state::TILE_PX_SIZE;
use crate::game_state::{
    field_area_effect::FieldAreaEffectKind,
    field_particle::{
        FieldParticle,
        particle::{IconParticle, IconParticleBehavior},
    },
};
use crate::icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize};
use namui::*;
use rand::Rng;

const TOWER_BUFF_ICON_SIZE: f32 = 36.0;
const TOWER_BUFF_FADE_DURATION_MS: i64 = 4000;
const TOWER_BUFF_MIN_SPEED: f32 = 8.0;
const TOWER_BUFF_MAX_SPEED: f32 = 15.0;
const TOWER_BUFF_INITIAL_OPACITY: f32 = 0.8;

const MIN_EMIT_INTERVAL_MS: i64 = 400;
const MAX_EMIT_INTERVAL_MS: i64 = 750;
const PARTICLE_COUNT_PER_EMIT: usize = 1;
const DEFAULT_EMITTER_DURATION_SECONDS: f32 = 1.0;

pub struct TowerStatusEffectEmitter {
    tower_xy: MapCoordF32,
    buff_kind: FieldAreaEffectKind,
    next_emit_time: Instant,
    created_at: Instant,
    duration: Duration,
}

impl TowerStatusEffectEmitter {
    pub fn new(
        now: Instant,
        tower_xy: MapCoordF32,
        buff_kind: FieldAreaEffectKind,
        duration: Duration,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let initial_delay =
            Duration::from_millis(rng.gen_range(MIN_EMIT_INTERVAL_MS..=MAX_EMIT_INTERVAL_MS));

        Self {
            tower_xy,
            buff_kind,
            next_emit_time: now + initial_delay,
            created_at: now,
            duration,
        }
    }

    pub fn new_with_default_duration(
        now: Instant,
        tower_xy: MapCoordF32,
        buff_kind: FieldAreaEffectKind,
    ) -> Self {
        Self::new(
            now,
            tower_xy,
            buff_kind,
            Duration::from_secs_f32(DEFAULT_EMITTER_DURATION_SECONDS),
        )
    }

    fn create_tower_buff_icon(&self) -> Icon {
        let (icon_kind, attribute_icon) = match &self.buff_kind {
            FieldAreaEffectKind::TowerAttackPowerPlusBuffOverTime { .. }
            | FieldAreaEffectKind::TowerAttackPowerMultiplyBuffOverTime { .. } => {
                (IconKind::AttackDamage, IconKind::Up)
            }
            FieldAreaEffectKind::TowerAttackSpeedPlusBuffOverTime { .. }
            | FieldAreaEffectKind::TowerAttackSpeedMultiplyBuffOverTime { .. } => {
                (IconKind::AttackSpeed, IconKind::Up)
            }
            FieldAreaEffectKind::TowerAttackRangePlusBuffOverTime { .. } => {
                (IconKind::AttackRange, IconKind::Up)
            }
            _ => (IconKind::AttackDamage, IconKind::Up),
        };

        Icon {
            kind: icon_kind,
            size: IconSize::Custom {
                size: px(TOWER_BUFF_ICON_SIZE),
            },
            attributes: vec![IconAttribute {
                icon_kind: attribute_icon,
                position: IconAttributePosition::BottomRight,
            }],
            wh: Wh::single(px(TOWER_BUFF_ICON_SIZE)),
            opacity: TOWER_BUFF_INITIAL_OPACITY,
        }
    }

    fn map_coord_to_pixel_f32(&self, coord: MapCoordF32) -> Xy<f32> {
        let tile_size = crate::game_state::TILE_PX_SIZE;
        let pixel = tile_size.to_xy() * coord;
        Xy {
            x: pixel.x.as_f32(),
            y: pixel.y.as_f32(),
        }
    }

    fn create_fade_rise_particle(&self, now: Instant) -> FieldParticle {
        let mut rng = rand::thread_rng();
        let tower_pixel = self.map_coord_to_pixel_f32(self.tower_xy);

        let offset_range = 0.75;
        let offset_x = TILE_PX_SIZE.width.as_f32() * rng.gen_range(-offset_range..=offset_range);
        let offset_y = TILE_PX_SIZE.height.as_f32() * rng.gen_range(-offset_range..=offset_range);

        let position = Xy {
            x: tower_pixel.x + offset_x,
            y: tower_pixel.y + offset_y,
        };

        let buff_icon = self.create_tower_buff_icon();

        let behavior = IconParticleBehavior::FadeRise {
            duration: Duration::from_millis(TOWER_BUFF_FADE_DURATION_MS),
            speed: rng.gen_range(TOWER_BUFF_MIN_SPEED..=TOWER_BUFF_MAX_SPEED),
            created_at: now,
            initial_opacity: TOWER_BUFF_INITIAL_OPACITY,
        };

        let icon_particle = IconParticle {
            icon: buff_icon,
            xy: Xy::new(px(position.x), px(position.y)),
            rotation: 0.0.deg(),
            behavior,
        };

        FieldParticle::Icon {
            particle: icon_particle,
        }
    }

    fn calculate_next_emit_time(&mut self, now: Instant) {
        let mut rng = rand::thread_rng();
        let interval =
            Duration::from_millis(rng.gen_range(MIN_EMIT_INTERVAL_MS..=MAX_EMIT_INTERVAL_MS));
        self.next_emit_time = now + interval;
    }

    pub fn emit(&mut self, now: Instant, _dt: Duration) -> Vec<FieldParticle> {
        if now < self.next_emit_time || self.is_done(now) {
            return vec![];
        }

        let mut particles = Vec::with_capacity(PARTICLE_COUNT_PER_EMIT);

        for _ in 0..PARTICLE_COUNT_PER_EMIT {
            particles.push(self.create_fade_rise_particle(now));
        }

        self.calculate_next_emit_time(now);

        particles
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }
}
