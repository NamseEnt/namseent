pub mod laser;

use super::projectile::{
    HOMING_ACCELERATION, HOMING_MAX_SPEED, ProjectileBehavior, ProjectileKind,
    ProjectileTargetIndicator, ProjectileTrail, homing_speed_for_key, homing_turn_rate_for_key,
};
use crate::SimTick;
use crate::card::Suit;
use crate::game_state::TILE_PX_SIZE;
use crate::{AttackId, Damage, FixedRatio, MonsterId, TowerId, WorldAcceleration};
use crate::{WorldCoord, WorldSpeed, WorldVec, game_state::card::Rank};
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

impl TowerInfo {
    pub(crate) fn to_core_state(self) -> td_core::AttackSourceState {
        td_core::AttackSourceState {
            tower_id: self.id.raw(),
            tower_kind: self.kind.to_core_raw(),
            rank: self.rank.map(|rank| rank.ordinal() as u8),
            suit: self.suit.map(crate::card::suit_to_core_raw),
        }
    }

    pub(crate) fn from_core_state(state: td_core::AttackSourceState) -> Option<Self> {
        Some(Self {
            id: TowerId::from_raw(state.tower_id),
            kind: crate::game_state::tower::TowerKind::from_core_raw(state.tower_kind)?,
            rank: match state.rank {
                Some(rank) => Some(crate::card::rank_from_core_raw(rank)?),
                None => None,
            },
            suit: match state.suit {
                Some(suit) => Some(crate::card::suit_from_core_raw(suit)?),
                None => None,
            },
        })
    }
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
    pub(crate) fn to_core_state(&self) -> td_core::SpatialAttackState {
        let behavior = match self.behavior {
            ProjectileBehavior::Direct => td_core::SpatialAttackBehaviorState::Direct,
            ProjectileBehavior::Homing {
                velocity,
                acceleration,
                turn_rate,
                max_speed,
                acceleration_remainder,
                turn_remainder,
            } => td_core::SpatialAttackBehaviorState::Homing {
                velocity: [velocity.x, velocity.y],
                acceleration_raw: acceleration.raw(),
                turn_rate_raw: turn_rate.raw(),
                max_speed_raw: max_speed.raw(),
                acceleration_remainder,
                turn_remainder,
            },
        };
        td_core::SpatialAttackState {
            position: [self.xy.x, self.xy.y],
            target_monster_id: self.target_indicator.id().raw(),
            velocity: [self.velocity.x, self.velocity.y],
            behavior,
            movement_remainder: self.movement_remainder,
            stable_key: self.stable_key,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_core_state(
        state: td_core::SpatialAttackState,
        projectile_kind: ProjectileKind,
        trail: ProjectileTrail,
        hit_effect: ProjectileHitEffect,
    ) -> Self {
        let behavior = match state.behavior {
            td_core::SpatialAttackBehaviorState::Direct => ProjectileBehavior::Direct,
            td_core::SpatialAttackBehaviorState::Homing {
                velocity,
                acceleration_raw,
                turn_rate_raw,
                max_speed_raw,
                acceleration_remainder,
                turn_remainder,
            } => ProjectileBehavior::Homing {
                velocity: WorldVec::new(velocity[0], velocity[1]),
                acceleration: WorldAcceleration::from_raw(acceleration_raw),
                turn_rate: FixedRatio::from_raw(turn_rate_raw),
                max_speed: WorldSpeed::from_raw(max_speed_raw),
                acceleration_remainder,
                turn_remainder,
            },
        };
        Self {
            xy: WorldCoord::new(state.position[0], state.position[1]),
            target_indicator: ProjectileTargetIndicator::from_id(MonsterId::from_raw(
                state.target_monster_id,
            )),
            velocity: WorldVec::new(state.velocity[0], state.velocity[1]),
            projectile_kind,
            trail,
            behavior,
            hit_effect,
            movement_remainder: state.movement_remainder,
            stable_key: state.stable_key,
        }
    }
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

    #[cfg(test)]
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
            let mut position = [self.xy.x, self.xy.y];
            let mut raw_velocity = [velocity.x, velocity.y];
            td_core::advance_homing_projectile(
                &mut position,
                &mut raw_velocity,
                acceleration_remainder,
                turn_remainder,
                &mut self.movement_remainder,
                td_core::HomingProjectileParams {
                    acceleration_raw: acceleration.raw(),
                    turn_rate_raw: turn_rate.raw(),
                    max_speed_raw: max_speed.raw(),
                    target: [dest_xy.x, dest_xy.y],
                    ticks_per_second: crate::world::SIM_TICKS_PER_SECOND as u64,
                    direct_switch_distance_raw:
                        crate::game_state::projectile::HOMING_SWITCH_TO_DIRECT_DISTANCE.raw(),
                    direct_acceleration_multiplier_raw:
                        crate::game_state::projectile::HOMING_DIRECT_ACCELERATION_MULTIPLIER.raw(),
                },
            );
            self.xy = WorldCoord::new(position[0], position[1]);
            *velocity = WorldVec::new(raw_velocity[0], raw_velocity[1]);
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

        let raw = attack.to_core_state();
        let restored = SpatialAttack::from_core_state(
            raw,
            ProjectileKind::Cards00,
            ProjectileTrail::None,
            ProjectileHitEffect::CardBurst,
        );
        assert_eq!(restored.xy, attack.xy);
        assert_eq!(restored.velocity, attack.velocity);
        assert_eq!(restored.target_indicator.id(), attack.target_indicator.id());
        assert_eq!(restored.movement_remainder, attack.movement_remainder);
        assert_eq!(restored.stable_key, attack.stable_key);
    }

    #[test]
    fn tower_info_round_trips_through_attack_source_state() {
        let source = TowerInfo {
            id: crate::TowerId::from_raw(9),
            kind: crate::game_state::tower::TowerKind::High,
            rank: Some(crate::game_state::card::Rank::Ace),
            suit: Some(crate::card::Suit::Spades),
        };
        let restored = TowerInfo::from_core_state(source.to_core_state()).unwrap();
        assert_eq!(restored, source);
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

impl TimedAttack {
    pub(crate) fn to_core_state(self) -> td_core::TimedAttackState {
        td_core::TimedAttackState {
            target_monster_id: self.target_monster_id.raw(),
            execute_at: self.execute_at.ticks(),
        }
    }
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
    pub(crate) fn to_core_state(&self) -> td_core::InFlightAttackState {
        let kind = match &self.kind {
            InFlightAttackKind::Spatial(spatial) => {
                td_core::InFlightAttackKindState::Spatial(spatial.to_core_state())
            }
            InFlightAttackKind::Timed(timed) => {
                td_core::InFlightAttackKindState::Timed(timed.to_core_state())
            }
            InFlightAttackKind::Laser(laser) => {
                td_core::InFlightAttackKindState::Laser(laser.to_core_state())
            }
        };
        td_core::InFlightAttackState {
            id: self.id.raw(),
            damage_raw: self.damage.raw(),
            source_tower: self.source_tower.map(TowerInfo::to_core_state),
            kind,
            on_hit_splashes: self
                .on_hit_splashes
                .iter()
                .copied()
                .map(crate::card::EngravingSplash::to_core_state)
                .collect(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_core_state(
        state: td_core::InFlightAttackState,
        spatial_presentation: Option<(ProjectileKind, ProjectileTrail, ProjectileHitEffect)>,
    ) -> Option<Self> {
        let kind = match state.kind {
            td_core::InFlightAttackKindState::Spatial(spatial) => {
                let (projectile_kind, trail, hit_effect) = spatial_presentation?;
                InFlightAttackKind::Spatial(SpatialAttack::from_core_state(
                    spatial,
                    projectile_kind,
                    trail,
                    hit_effect,
                ))
            }
            td_core::InFlightAttackKindState::Timed(timed) => {
                InFlightAttackKind::Timed(TimedAttack {
                    target_monster_id: MonsterId::from_raw(timed.target_monster_id),
                    execute_at: SimTick::from_ticks(timed.execute_at),
                })
            }
            td_core::InFlightAttackKindState::Laser(laser) => {
                InFlightAttackKind::Laser(laser::LaserBeam::new(
                    WorldCoord::new(laser.start_xy[0], laser.start_xy[1]),
                    WorldCoord::new(laser.end_xy[0], laser.end_xy[1]),
                    SimTick::from_ticks(laser.created_at),
                    MonsterId::from_raw(laser.target_monster_id),
                ))
            }
        };
        Some(Self {
            id: AttackId::from_raw(state.id),
            damage: Damage::from_raw(state.damage_raw),
            source_tower: match state.source_tower {
                Some(source) => Some(TowerInfo::from_core_state(source)?),
                None => None,
            },
            kind,
            on_hit_splashes: state
                .on_hit_splashes
                .into_iter()
                .map(crate::card::EngravingSplash::from_core_state)
                .collect(),
        })
    }
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
