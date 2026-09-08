use crate::{
    HomingProjectileParams, MonsterDamageResult, MonsterState, SIM_TICKS_PER_SECOND,
    advance_direct_projectile, advance_homing_projectile, segment_hits_point, vector_length_raw,
};

pub const PROJECTILE_COLLISION_RADIUS_RAW: i64 = 100_000;
pub const HOMING_SWITCH_TO_DIRECT_DISTANCE_RAW: i64 = 4 * crate::WORLD_UNITS_PER_TILE;
pub const HOMING_DIRECT_ACCELERATION_MULTIPLIER_RAW: i64 = 100_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TimedAttackState {
    pub target_monster_id: u64,
    pub execute_at: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LaserAttackState {
    pub start_xy: [i64; 2],
    pub end_xy: [i64; 2],
    pub created_at: u64,
    pub target_monster_id: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AttackSourceState {
    pub tower_id: u64,
    pub tower_kind: u8,
    pub rank: Option<u8>,
    pub suit: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SpatialAttackBehaviorState {
    Direct,
    Homing {
        velocity: [i64; 2],
        acceleration_raw: i64,
        turn_rate_raw: i64,
        max_speed_raw: i64,
        acceleration_remainder: i64,
        turn_remainder: i64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SpatialAttackState {
    pub position: [i64; 2],
    pub target_monster_id: u64,
    pub velocity: [i64; 2],
    pub behavior: SpatialAttackBehaviorState,
    pub movement_remainder: i64,
    pub stable_key: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InFlightAttackKindState {
    Spatial(SpatialAttackState),
    Timed(TimedAttackState),
    Laser(LaserAttackState),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InFlightAttackState {
    pub id: u64,
    pub damage_raw: i64,
    pub source_tower: Option<AttackSourceState>,
    pub kind: InFlightAttackKindState,
    pub on_hit_splashes: Vec<DamageSplash>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DamageSplash {
    pub radius_raw: i64,
    pub damage_pct_raw: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DamageHit {
    pub target_index: usize,
    pub damage_raw: i64,
    pub at_xy: [i64; 2],
    pub source_index: usize,
    pub splashes: Vec<DamageSplash>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DamageHitResult {
    pub target_index: usize,
    pub damage: MonsterDamageResult,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AreaDamageEvent {
    pub center_xy: [i64; 2],
    pub damage_raw: i64,
    pub source_index: usize,
    pub splashes: Vec<DamageSplash>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedAttack {
    pub attack: InFlightAttackState,
    pub target_index: usize,
    pub at_xy: [i64; 2],
}

pub(crate) fn timed_attack_is_due(attack: TimedAttackState, sim_tick: u64) -> bool {
    attack.execute_at <= sim_tick
}

pub(crate) fn damage_hit_sort_key(monster_ids: &[u64], target_index: usize) -> (u64, usize) {
    (
        monster_ids.get(target_index).copied().unwrap_or(u64::MAX),
        target_index,
    )
}

pub(crate) fn advance_in_flight_attacks_with_events(
    attacks: &mut Vec<InFlightAttackState>,
    monsters: &[MonsterState],
    sim_tick: u64,
    events: &mut Vec<crate::CoreEvent>,
) -> Vec<Vec<ResolvedAttack>> {
    attacks.sort_by_key(|attack| attack.id);
    let monster_index_by_id = monsters
        .iter()
        .enumerate()
        .map(|(index, monster)| (monster.id, index))
        .collect::<std::collections::HashMap<_, _>>();
    let mut remaining = Vec::with_capacity(attacks.len());
    let mut timed_hits = Vec::new();
    let mut laser_hits = Vec::new();
    let mut spatial_hits = Vec::new();

    for mut attack in attacks.drain(..) {
        match &mut attack.kind {
            InFlightAttackKindState::Timed(timed) => {
                if !timed_attack_is_due(*timed, sim_tick) {
                    remaining.push(attack);
                    continue;
                }
                let Some(&target_index) = monster_index_by_id.get(&timed.target_monster_id) else {
                    continue;
                };
                let position = monster_center_xy(&monsters[target_index]);
                events.push(crate::CoreEvent::TimedAttackExecuted { position });
                timed_hits.push(ResolvedAttack {
                    attack,
                    target_index,
                    at_xy: position,
                });
            }
            InFlightAttackKindState::Laser(laser) => {
                let Some(&target_index) = monster_index_by_id.get(&laser.target_monster_id) else {
                    continue;
                };
                laser_hits.push(ResolvedAttack {
                    attack,
                    target_index,
                    at_xy: monster_center_xy(&monsters[target_index]),
                });
            }
            InFlightAttackKindState::Spatial(spatial) => {
                let Some(&target_index) = monster_index_by_id.get(&spatial.target_monster_id)
                else {
                    continue;
                };
                let target_xy = monster_center_xy(&monsters[target_index]);
                let start_xy = spatial.position;
                match &mut spatial.behavior {
                    SpatialAttackBehaviorState::Direct => {
                        let speed_raw = vector_length_raw(spatial.velocity);
                        advance_direct_projectile(
                            &mut spatial.position,
                            &mut spatial.velocity,
                            &mut spatial.movement_remainder,
                            target_xy,
                            speed_raw,
                            SIM_TICKS_PER_SECOND,
                        );
                    }
                    SpatialAttackBehaviorState::Homing {
                        velocity,
                        acceleration_raw,
                        turn_rate_raw,
                        max_speed_raw,
                        acceleration_remainder,
                        turn_remainder,
                    } => {
                        advance_homing_projectile(
                            &mut spatial.position,
                            velocity,
                            acceleration_remainder,
                            turn_remainder,
                            &mut spatial.movement_remainder,
                            HomingProjectileParams {
                                acceleration_raw: *acceleration_raw,
                                turn_rate_raw: *turn_rate_raw,
                                max_speed_raw: *max_speed_raw,
                                target: target_xy,
                                ticks_per_second: SIM_TICKS_PER_SECOND,
                                direct_switch_distance_raw: HOMING_SWITCH_TO_DIRECT_DISTANCE_RAW,
                                direct_acceleration_multiplier_raw:
                                    HOMING_DIRECT_ACCELERATION_MULTIPLIER_RAW,
                            },
                        );
                        spatial.velocity = *velocity;
                    }
                }
                let end_xy = spatial.position;
                if !segment_hits_point(start_xy, end_xy, target_xy, PROJECTILE_COLLISION_RADIUS_RAW)
                    && end_xy != target_xy
                {
                    events.push(crate::CoreEvent::ProjectileMoved {
                        attack_id: attack.id,
                        start_xy,
                        end_xy,
                    });
                    remaining.push(attack);
                    continue;
                }
                events.push(crate::CoreEvent::ProjectileHit {
                    attack_id: attack.id,
                    position: target_xy,
                });
                spatial_hits.push(ResolvedAttack {
                    attack,
                    target_index,
                    at_xy: target_xy,
                });
            }
        }
    }

    *attacks = remaining;
    vec![timed_hits, laser_hits, spatial_hits]
}

fn monster_center_xy(monster: &MonsterState) -> [i64; 2] {
    [
        monster.move_on_route.map_coord[0].saturating_add(crate::WORLD_UNITS_PER_TILE / 2),
        monster.move_on_route.map_coord[1].saturating_add(crate::WORLD_UNITS_PER_TILE / 2),
    ]
}
