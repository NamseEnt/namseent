pub mod laser;

use super::projectile::{
    HOMING_ACCELERATION_TILE, HOMING_DIRECT_ACCELERATION_MULTIPLIER, HOMING_INITIAL_SPEED_MAX_TILE,
    HOMING_INITIAL_SPEED_MIN_TILE, HOMING_MAX_SPEED_TILE, HOMING_SWITCH_TO_DIRECT_DISTANCE_TILE,
    HOMING_TURN_RATE_MAX_TILE, HOMING_TURN_RATE_MIN_TILE, ProjectileBehavior, ProjectileKind,
    ProjectileTargetIndicator, ProjectileTrail, random_rotation_speed,
};
use crate::game_state::TILE_PX_SIZE;
use crate::game_state::card::Suit;
use crate::{MapCoordF32, game_state::card::Rank};
use namui::*;
use rand::Rng;
use std::sync::atomic::{AtomicU64, Ordering};

/// 데미지를 가한 타워의 신원 정보. rank와 suit는 optional로 둬서
/// 일부 타워에서 값이 없을 때도 안전하게 처리할 수 있다.
#[derive(Clone, Copy, Debug, PartialEq, State)]
pub struct TowerInfo {
    pub id: usize,
    pub kind: crate::game_state::tower::TowerKind,
    pub rank: Option<Rank>,
    pub suit: Option<Suit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum ProjectileHitEffect {
    TrashBounce,
    CardBurst,
    SparkleBurst,
    HeartBurst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectileGroup {
    Trash,
    Girl,
    Cards,
    Heart,
}

impl ProjectileGroup {
    pub fn random_kind(&self) -> ProjectileKind {
        match self {
            Self::Trash => ProjectileKind::random_trash(),
            Self::Girl => ProjectileKind::random_girl(),
            Self::Cards => ProjectileKind::random_cards(),
            Self::Heart => ProjectileKind::random_heart(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttackType {
    Projectile {
        speed: Per<f32, Duration>,
        trail: ProjectileTrail,
        projectile_group: ProjectileGroup,
        hit_effect: ProjectileHitEffect,
    },
    Laser,
    FullHouseRain {
        tower_xy: (f32, f32),
    },
    RoyalStraightFlush {
        target_xy: (f32, f32),
    },
}

// ── Unified in-flight attack ──────────────────────────────────────────────────

#[derive(Clone, State)]
pub struct SpatialAttack {
    pub xy: MapCoordF32,
    pub target_indicator: ProjectileTargetIndicator,
    pub velocity: Xy<f32>,
    pub projectile_kind: ProjectileKind,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub trail: ProjectileTrail,
    pub behavior: ProjectileBehavior,
    pub hit_effect: ProjectileHitEffect,
}

impl SpatialAttack {
    pub fn new_direct(
        xy: MapCoordF32,
        target_indicator: ProjectileTargetIndicator,
        projectile_kind: ProjectileKind,
        speed: Per<f32, Duration>,
        trail: ProjectileTrail,
        hit_effect: ProjectileHitEffect,
    ) -> Self {
        let speed_scalar = speed * Duration::from_secs(1);
        let initial_direction = Xy::new(0.0f32, -1.0f32);
        Self {
            xy,
            target_indicator,
            velocity: initial_direction * speed_scalar,
            projectile_kind,
            rotation: 0.0.deg(),
            rotation_speed: random_rotation_speed(),
            trail,
            behavior: ProjectileBehavior::Direct,
            hit_effect,
        }
    }

    pub fn new_homing(
        xy: MapCoordF32,
        target_indicator: ProjectileTargetIndicator,
        projectile_kind: ProjectileKind,
        trail: ProjectileTrail,
        hit_effect: ProjectileHitEffect,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let initial_speed =
            rng.gen_range(HOMING_INITIAL_SPEED_MIN_TILE..=HOMING_INITIAL_SPEED_MAX_TILE);
        let turn_rate = rng.gen_range(HOMING_TURN_RATE_MIN_TILE..=HOMING_TURN_RATE_MAX_TILE);
        let initial_velocity = Xy::new(0.0f32, -initial_speed);
        Self {
            xy,
            target_indicator,
            velocity: initial_velocity,
            projectile_kind,
            rotation: 0.0.deg(),
            rotation_speed: random_rotation_speed(),
            trail,
            behavior: ProjectileBehavior::Homing {
                velocity: initial_velocity,
                acceleration: HOMING_ACCELERATION_TILE,
                turn_rate,
                max_speed: HOMING_MAX_SPEED_TILE,
            },
            hit_effect,
        }
    }

    pub(crate) fn move_by(&mut self, dt: Duration, dest_xy: MapCoordF32) {
        let direction = (dest_xy - self.xy).normalize();
        let speed = self.velocity.length();
        self.xy += direction * speed * dt.as_secs_f32();
        self.velocity = direction * speed;
        self.rotation += self.rotation_speed * dt.as_secs_f32();
    }

    pub(crate) fn move_homing(&mut self, dt: Duration, dest_xy: MapCoordF32) {
        let dt_secs = dt.as_secs_f32();
        if let ProjectileBehavior::Homing {
            velocity,
            acceleration,
            turn_rate,
            max_speed,
        } = &mut self.behavior
        {
            let distance_to_target = (dest_xy - self.xy).length();
            let desired_dir = (dest_xy - self.xy).normalize();

            if distance_to_target > HOMING_SWITCH_TO_DIRECT_DISTANCE_TILE {
                let mut speed = velocity.length();
                let acceleration = *acceleration;
                let turn_rate = *turn_rate;
                let max_speed = *max_speed;
                speed = (speed + acceleration * dt_secs).min(max_speed);
                let target_velocity = desired_dir * speed;
                let t = (turn_rate * dt_secs).clamp(0.0, 1.0);
                *velocity += (target_velocity - *velocity) * t;
                self.xy += *velocity * dt_secs;
            } else {
                let acceleration = *acceleration;
                let max_speed = *max_speed;
                let mut speed = velocity.length();
                speed = (speed + acceleration * dt_secs * HOMING_DIRECT_ACCELERATION_MULTIPLIER)
                    .min(max_speed);
                *velocity = desired_dir * speed;
                self.xy += *velocity * dt_secs;
            }

            self.velocity = *velocity;
        }

        self.rotation += self.rotation_speed * dt.as_secs_f32();
    }
}

impl Component for &SpatialAttack {
    fn render(self, ctx: &RenderCtx) {
        let projectile_wh = TILE_PX_SIZE * Wh::new(0.4, 0.4);
        let image = self.projectile_kind.image();

        ctx.rotate(self.rotation).add(namui::image(ImageParam {
            rect: Rect::from_xy_wh(projectile_wh.to_xy() * -0.5, projectile_wh),
            image,
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint: None,
            },
        }));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, State)]
pub struct TimedAttack {
    pub target_monster_id: usize,
    pub execute_at: Instant,
    pub hit_sound: HitSound,
}

/// 지연 타격 시 재생할 사운드. TimedAttack이 직접 소유하여 호출부 하드코딩을 제거.
#[derive(Clone, Copy, PartialEq, Eq, State)]
pub enum HitSound {
    KnifeSlash,
}

#[derive(Clone, State)]
pub enum InFlightAttackKind {
    Spatial(SpatialAttack),
    Timed(TimedAttack),
    Laser(laser::LaserBeam),
}

static NEXT_IN_FLIGHT_ATTACK_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, State)]
pub struct InFlightAttack {
    pub id: u64,
    pub damage: f32,
    pub source_tower: Option<TowerInfo>,
    pub kind: InFlightAttackKind,
}

impl InFlightAttack {
    pub fn new_spatial(
        spatial: SpatialAttack,
        damage: f32,
        source_tower: Option<TowerInfo>,
    ) -> Self {
        Self {
            id: NEXT_IN_FLIGHT_ATTACK_ID.fetch_add(1, Ordering::Relaxed),
            damage,
            source_tower,
            kind: InFlightAttackKind::Spatial(spatial),
        }
    }

    pub fn new_timed(
        target_monster_id: usize,
        execute_at: Instant,
        damage: f32,
        source_tower: Option<TowerInfo>,
        hit_sound: HitSound,
    ) -> Self {
        Self {
            id: NEXT_IN_FLIGHT_ATTACK_ID.fetch_add(1, Ordering::Relaxed),
            damage,
            source_tower,
            kind: InFlightAttackKind::Timed(TimedAttack {
                target_monster_id,
                execute_at,
                hit_sound,
            }),
        }
    }

    pub fn new_laser(beam: laser::LaserBeam, damage: f32, source_tower: Option<TowerInfo>) -> Self {
        Self {
            id: NEXT_IN_FLIGHT_ATTACK_ID.fetch_add(1, Ordering::Relaxed),
            damage,
            source_tower,
            kind: InFlightAttackKind::Laser(beam),
        }
    }
}
