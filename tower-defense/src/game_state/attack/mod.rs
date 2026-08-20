pub mod laser;

use super::projectile::{
    HOMING_ACCELERATION, HOMING_DIRECT_ACCELERATION_MULTIPLIER, HOMING_MAX_SPEED,
    HOMING_SWITCH_TO_DIRECT_DISTANCE, ProjectileBehavior, ProjectileKind,
    ProjectileTargetIndicator, ProjectileTrail, homing_speed_for_key, homing_turn_rate_for_key,
    move_direct,
};
use crate::SimTick;
use crate::card::Suit;
use crate::game_state::TILE_PX_SIZE;
use crate::{AttackId, Damage, MonsterId, TowerId};
use crate::{
    FixedRatio, RatioProduct, WorldCoord, WorldDistance, WorldSpeed, WorldVec,
    game_state::card::Rank,
};
use namui::*;

/// 데미지를 가한 타워의 신원 정보. rank와 suit는 optional로 둬서
/// 일부 타워에서 값이 없을 때도 안전하게 처리할 수 있다.
#[derive(Clone, Copy, Debug, PartialEq, State)]
pub struct TowerInfo {
    pub id: TowerId,
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
    pub fn kind_for(&self, key: u64) -> ProjectileKind {
        match self {
            Self::Trash => ProjectileKind::deterministic_trash(key),
            Self::Girl => ProjectileKind::deterministic_girl(key),
            Self::Cards => ProjectileKind::Cards00,
            Self::Heart => ProjectileKind::Heart00,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttackType {
    Projectile {
        speed: WorldSpeed,
        trail: ProjectileTrail,
        projectile_group: ProjectileGroup,
        hit_effect: ProjectileHitEffect,
    },
    Laser,
    FullHouseRain {
        tower_xy: WorldCoord,
    },
    RoyalStraightFlush {
        target_xy: WorldCoord,
    },
}

// ── Unified in-flight attack ──────────────────────────────────────────────────

#[derive(Clone, State)]
pub struct SpatialAttack {
    pub xy: WorldCoord,
    pub target_indicator: ProjectileTargetIndicator,
    pub velocity: WorldVec,
    pub projectile_kind: ProjectileKind,
    pub trail: ProjectileTrail,
    pub behavior: ProjectileBehavior,
    pub hit_effect: ProjectileHitEffect,
    pub movement_remainder: i64,
    pub stable_key: u64,
}

impl SpatialAttack {
    pub fn new_direct(
        xy: WorldCoord,
        target_indicator: ProjectileTargetIndicator,
        key: u64,
        projectile_kind: ProjectileKind,
        speed: WorldSpeed,
        trail: ProjectileTrail,
        hit_effect: ProjectileHitEffect,
    ) -> Self {
        let initial_direction = WorldVec::new(0, -speed.raw());
        Self {
            xy,
            target_indicator,
            velocity: initial_direction,
            projectile_kind,
            trail,
            behavior: ProjectileBehavior::Direct,
            hit_effect,
            movement_remainder: 0,
            stable_key: key,
        }
    }

    pub fn new_homing(
        xy: WorldCoord,
        target_indicator: ProjectileTargetIndicator,
        key: u64,
        projectile_kind: ProjectileKind,
        trail: ProjectileTrail,
        hit_effect: ProjectileHitEffect,
    ) -> Self {
        let initial_speed = homing_speed_for_key(key);
        let turn_rate = homing_turn_rate_for_key(key);
        let initial_velocity = WorldVec::new(0, -initial_speed.raw());
        Self {
            xy,
            target_indicator,
            velocity: initial_velocity,
            projectile_kind,
            trail,
            behavior: ProjectileBehavior::Homing {
                velocity: initial_velocity,
                acceleration: HOMING_ACCELERATION,
                turn_rate,
                max_speed: HOMING_MAX_SPEED,
                acceleration_remainder: 0,
                turn_remainder: 0,
            },
            hit_effect,
            movement_remainder: 0,
            stable_key: key,
        }
    }

    pub(crate) fn move_by(&mut self, dest_xy: WorldCoord) {
        let speed = WorldSpeed::from_raw(self.velocity.length().raw());
        move_direct(
            &mut self.xy,
            &mut self.velocity,
            &mut self.movement_remainder,
            dest_xy,
            speed,
        );
    }

    pub(crate) fn move_homing(&mut self, dest_xy: WorldCoord) {
        if let ProjectileBehavior::Homing {
            velocity,
            acceleration,
            turn_rate,
            max_speed,
            acceleration_remainder,
            turn_remainder,
        } = &mut self.behavior
        {
            let distance = (dest_xy - self.xy).length();
            if distance.is_zero() {
                self.xy = dest_xy;
                *velocity = WorldVec::ZERO;
                self.velocity = WorldVec::ZERO;
                return;
            }
            let direct_steering = distance <= HOMING_SWITCH_TO_DIRECT_DISTANCE;
            let effective_acceleration = if direct_steering {
                RatioProduct::one()
                    .with(HOMING_DIRECT_ACCELERATION_MULTIPLIER)
                    .apply_raw(acceleration.raw())
            } else {
                acceleration.raw()
            };
            let full_acceleration =
                effective_acceleration as i128 + *acceleration_remainder as i128;
            let acceleration_per_tick =
                full_acceleration / crate::world::SIM_TICKS_PER_SECOND as i128;
            *acceleration_remainder =
                (full_acceleration % crate::world::SIM_TICKS_PER_SECOND as i128) as i64;
            let acceleration_per_tick = acceleration_per_tick.clamp(0, i64::MAX as i128) as i64;
            let speed = WorldSpeed::from_raw(
                velocity
                    .length()
                    .raw()
                    .saturating_add(acceleration_per_tick)
                    .min(max_speed.raw()),
            );
            let desired = (dest_xy - self.xy)
                .scaled_by_distance(WorldDistance::from_raw(speed.raw()), distance);
            if direct_steering {
                *velocity = desired;
            } else {
                let turn_numerator = turn_rate.raw() as i128 + *turn_remainder as i128;
                let turn_per_tick = turn_numerator / crate::world::SIM_TICKS_PER_SECOND as i128;
                *turn_remainder =
                    (turn_numerator % crate::world::SIM_TICKS_PER_SECOND as i128) as i64;
                let turn_per_tick = FixedRatio::from_raw(
                    turn_per_tick.clamp(0, FixedRatio::ONE.raw() as i128) as i64,
                );
                *velocity += (desired - *velocity).scaled_by_ratio(turn_per_tick);
            }
            let current_speed = WorldSpeed::from_raw(velocity.length().raw());
            let numerator = current_speed.raw() as i128 + self.movement_remainder as i128;
            let step = (numerator / crate::world::SIM_TICKS_PER_SECOND as i128) as i64;
            self.movement_remainder =
                (numerator % crate::world::SIM_TICKS_PER_SECOND as i128) as i64;
            self.xy += velocity.scaled_by_distance(
                WorldDistance::from_raw(step),
                WorldDistance::from_raw(current_speed.raw()),
            );
            self.velocity = *velocity;
        }
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;

    #[test]
    fn homing_uses_direct_steering_inside_switch_distance() {
        let mut attack = SpatialAttack::new_homing(
            WorldCoord::ZERO,
            ProjectileTargetIndicator::from_id(crate::MonsterId::from_raw(1)),
            7,
            ProjectileKind::Cards00,
            ProjectileTrail::None,
            ProjectileHitEffect::CardBurst,
        );

        attack.move_homing(WorldCoord::from_tile(1, 0));

        assert!(attack.velocity.x > 0);
        assert_eq!(attack.velocity.y, 0);
        assert!(attack.xy.x > 0);
        assert_eq!(attack.xy.y, 0);
    }
}

impl Component for &SpatialAttack {
    fn render(self, ctx: &RenderCtx) {
        render_projectile_sprite(
            ctx,
            self.projectile_kind,
            Xy::new(self.velocity.x as f32, self.velocity.y as f32),
        );
    }
}

pub(crate) struct RenderProjectileSnapshot {
    pub(crate) projectile_kind: ProjectileKind,
    pub(crate) direction: Xy<f32>,
}

impl Component for RenderProjectileSnapshot {
    fn render(self, ctx: &RenderCtx) {
        render_projectile_sprite(ctx, self.projectile_kind, self.direction);
    }
}

fn render_projectile_sprite(ctx: &RenderCtx, projectile_kind: ProjectileKind, direction: Xy<f32>) {
    let projectile_wh = TILE_PX_SIZE * Wh::new(0.4, 0.4);
    let image = projectile_kind.image();

    ctx.rotate(direction.atan2()).add(namui::image(ImageParam {
        rect: Rect::from_xy_wh(projectile_wh.to_xy() * -0.5, projectile_wh),
        image,
        style: ImageStyle {
            fit: ImageFit::Contain,
            paint: None,
        },
    }));
}

#[derive(Clone, Copy, PartialEq, Eq, State)]
pub struct TimedAttack {
    pub target_monster_id: MonsterId,
    pub execute_at: SimTick,
}

#[derive(Clone, State)]
pub enum InFlightAttackKind {
    Spatial(SpatialAttack),
    Timed(TimedAttack),
    Laser(laser::LaserBeam),
}

#[derive(Clone, State)]
pub struct InFlightAttack {
    pub id: AttackId,
    pub damage: Damage,
    pub source_tower: Option<TowerInfo>,
    pub kind: InFlightAttackKind,
    pub on_hit_splashes: Vec<crate::card::EngravingSplash>,
}

impl InFlightAttack {
    pub fn with_on_hit_splashes(mut self, splashes: Vec<crate::card::EngravingSplash>) -> Self {
        self.on_hit_splashes = splashes;
        self
    }

    pub(crate) fn new_spatial(
        id: AttackId,
        spatial: SpatialAttack,
        damage: Damage,
        source_tower: Option<TowerInfo>,
    ) -> Self {
        Self {
            id,
            damage,
            source_tower,
            kind: InFlightAttackKind::Spatial(spatial),
            on_hit_splashes: Vec::new(),
        }
    }

    pub(crate) fn new_timed(
        id: AttackId,
        target_monster_id: MonsterId,
        execute_at: SimTick,
        damage: Damage,
        source_tower: Option<TowerInfo>,
    ) -> Self {
        Self {
            id,
            damage,
            source_tower,
            kind: InFlightAttackKind::Timed(TimedAttack {
                target_monster_id,
                execute_at,
            }),
            on_hit_splashes: Vec::new(),
        }
    }

    pub(crate) fn new_laser(
        id: AttackId,
        beam: laser::LaserBeam,
        damage: Damage,
        source_tower: Option<TowerInfo>,
    ) -> Self {
        Self {
            id,
            damage,
            source_tower,
            kind: InFlightAttackKind::Laser(beam),
            on_hit_splashes: Vec::new(),
        }
    }
}
