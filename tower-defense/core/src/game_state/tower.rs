use crate::{CardState, TowerStatusEffect, TowerStatusEffectKind};
use std::cmp::Reverse;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TowerSkillKind {
    NearbyTowerDamageMul { mul_raw: i64, range_radius_raw: i64 },
    NearbyTowerDamageAdd { add_raw: i64, range_radius_raw: i64 },
    NearbyMonsterSpeedMul { mul_raw: i64, range_radius_raw: i64 },
    MoneyIncomeAdd { add: u32 },
    TopCardBonus { rank: u8, bonus_damage: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerSkill {
    pub last_used_at: u64,
    pub template: TowerSkillTemplate,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerSkillTemplate {
    pub kind: TowerSkillKind,
    pub cooldown: u64,
    pub duration: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerTemplateState {
    pub kind: u8,
    pub rerolled_count: usize,
    pub shoot_interval: u64,
    pub default_attack_range_radius_raw: i64,
    pub default_damage_raw: i64,
    pub suit: Option<u8>,
    pub rank: Option<u8>,
    pub skill_templates: Vec<TowerSkillTemplate>,
    pub default_status_effects: Vec<TowerStatusEffect>,
    pub used_cards: Vec<CardState>,
}

pub const fn rank_is_face(rank: Option<u8>) -> bool {
    matches!(rank, Some(9..=11))
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerState {
    pub id: Option<u64>,
    pub left_top: [usize; 2],
    pub cooldown: u64,
    pub template: TowerTemplateState,
    pub status_effects: Vec<TowerStatusEffect>,
    pub skills: Vec<TowerSkill>,
    #[serde(default = "default_damage_multiplier_raw")]
    pub damage_multiplier_raw: i64,
    #[serde(default)]
    pub attack_range_radius_raw: i64,
    #[serde(default)]
    pub effective_shoot_interval: u64,
    #[serde(default)]
    pub on_hit_splashes: Vec<crate::DamageSplash>,
    #[serde(default)]
    pub on_attack_splashes: Vec<crate::DamageSplash>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TowerAttackOutput {
    pub area_damage_events: Vec<crate::AreaDamageEvent>,
    pub area_damage_sources: Vec<crate::AttackSourceState>,
    pub events: Vec<crate::CoreEvent>,
}

/// Result of removing a tower, carrying only authoritative data needed by
/// upgrade triggers (currently the rerolled count of the removed template).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemoveTowerOutput {
    pub rerolled_count: usize,
}

/// All four 2x2 tile coords occupied by each tower, used as route blockers.
pub(crate) fn tower_blockers(towers: &[TowerState]) -> Vec<[usize; 2]> {
    towers
        .iter()
        .flat_map(|tower| {
            let [x, y] = tower.left_top;
            [
                [x, y],
                [x.saturating_add(1), y],
                [x, y.saturating_add(1)],
                [x.saturating_add(1), y.saturating_add(1)],
            ]
        })
        .collect()
}

const fn default_damage_multiplier_raw() -> i64 {
    crate::RATIO_SCALE
}

impl TowerState {
    pub fn attack_range_raw(&self) -> i64 {
        if self.attack_range_radius_raw == 0 && self.template.kind != 0 {
            self.template.default_attack_range_radius_raw
        } else {
            self.attack_range_radius_raw
        }
    }

    pub fn shoot_interval_ticks(&self) -> u64 {
        if self.effective_shoot_interval == 0 {
            self.template.shoot_interval
        } else {
            self.effective_shoot_interval
        }
    }

    pub fn attack_damage_raw(&self) -> i64 {
        let mut damage = self.template.default_damage_raw;
        let mut multipliers = Vec::new();
        for effect in &self.status_effects {
            match effect.kind {
                TowerStatusEffectKind::DamageAdd { add_raw } => {
                    damage = damage.saturating_add(add_raw);
                }
                TowerStatusEffectKind::DamageMul { mul_raw } => multipliers.push(mul_raw),
            }
        }
        if damage <= 0 {
            return 0;
        }
        let card_polish_raw = self
            .template
            .used_cards
            .iter()
            .map(|card| card.polish_pct_raw)
            .fold(0_i64, i64::saturating_add);
        let upgrade_bonus_raw = self
            .damage_multiplier_raw
            .max(0)
            .saturating_sub(crate::RATIO_SCALE);
        multipliers.push(
            crate::RATIO_SCALE
                .saturating_add(card_polish_raw)
                .saturating_add(upgrade_bonus_raw),
        );
        crate::apply_ratio_product_raw(damage, &multipliers)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ActivatedTowerSkill {
    pub tower_id: u64,
    pub skill: TowerSkill,
}

pub(crate) fn advance_tower_cooldowns(towers: &mut [TowerState]) {
    for tower in towers {
        tower.cooldown = tower.cooldown.saturating_sub(1);
    }
}

pub(crate) fn activate_tower_skills(
    towers: &mut [TowerState],
    sim_tick: u64,
) -> Vec<ActivatedTowerSkill> {
    let mut activated = Vec::new();
    for tower in towers {
        let Some(tower_id) = tower.id else {
            continue;
        };
        for skill in &mut tower.skills {
            if sim_tick < skill.last_used_at.saturating_add(skill.template.cooldown) {
                continue;
            }
            skill.last_used_at = sim_tick;
            activated.push(ActivatedTowerSkill {
                tower_id,
                skill: skill.clone(),
            });
        }
    }
    activated
}

pub(crate) fn apply_tower_skill_activations(
    towers: &mut [TowerState],
    monsters: &mut [crate::MonsterState],
    activations: &[ActivatedTowerSkill],
    sim_tick: u64,
) {
    for activation in activations {
        let Some(caster_index) = towers
            .iter()
            .position(|tower| tower.id == Some(activation.tower_id))
        else {
            continue;
        };
        let caster_xy = tower_center_xy(&towers[caster_index]);
        let skill = &activation.skill.template;
        match skill.kind {
            TowerSkillKind::NearbyTowerDamageMul {
                mul_raw,
                range_radius_raw,
            } => {
                let effect = TowerStatusEffect {
                    kind: crate::TowerStatusEffectKind::DamageMul { mul_raw },
                    end: crate::TowerStatusEffectEnd::Time {
                        end_at: sim_tick.saturating_add(skill.duration),
                    },
                };
                for tower in towers.iter_mut() {
                    if within_range(caster_xy, tower_center_xy(tower), range_radius_raw) {
                        tower.status_effects.push(effect.clone());
                    }
                }
            }
            TowerSkillKind::NearbyTowerDamageAdd {
                add_raw,
                range_radius_raw,
            } => {
                let effect = TowerStatusEffect {
                    kind: crate::TowerStatusEffectKind::DamageAdd { add_raw },
                    end: crate::TowerStatusEffectEnd::Time {
                        end_at: sim_tick.saturating_add(skill.duration),
                    },
                };
                for tower in towers.iter_mut() {
                    if within_range(caster_xy, tower_center_xy(tower), range_radius_raw) {
                        tower.status_effects.push(effect.clone());
                    }
                }
            }
            TowerSkillKind::NearbyMonsterSpeedMul {
                mul_raw,
                range_radius_raw,
            } => {
                let effect = crate::MonsterStatusEffect {
                    kind: crate::MonsterStatusEffectKind::SpeedMul { mul_raw },
                    end_at: sim_tick.saturating_add(skill.duration),
                };
                for monster in monsters.iter_mut() {
                    if within_range(caster_xy, monster_center_xy(monster), range_radius_raw) {
                        monster.status_effects.push(effect.clone());
                    }
                }
            }
            TowerSkillKind::MoneyIncomeAdd { .. } => {}
            TowerSkillKind::TopCardBonus { bonus_damage, .. } if bonus_damage > 0 => {
                towers[caster_index].status_effects.push(TowerStatusEffect {
                    kind: crate::TowerStatusEffectKind::DamageAdd {
                        add_raw: bonus_damage.min(i64::MAX as usize) as i64,
                    },
                    end: crate::TowerStatusEffectEnd::Time {
                        end_at: sim_tick.saturating_add(skill.duration),
                    },
                });
            }
            TowerSkillKind::TopCardBonus { .. } => {}
        }
    }
}

pub(crate) fn generate_tower_attacks(
    towers: &mut [TowerState],
    monsters: &[crate::MonsterState],
    in_flight_attacks: &mut Vec<crate::InFlightAttackState>,
    next_entity_id: &mut crate::EntityIdAllocator,
    sim_tick: u64,
    disabled_ranks: &[u8],
    disabled_suits: &[u8],
) -> TowerAttackOutput {
    let mut new_attacks = Vec::new();
    let mut area_damage_events = Vec::new();
    let mut area_damage_sources = Vec::new();
    let mut events = Vec::new();

    for tower in towers.iter_mut() {
        let Some(tower_id) = tower.id else {
            continue;
        };
        if tower.cooldown > 0
            || tower
                .template
                .rank
                .is_some_and(|rank| disabled_ranks.contains(&rank))
            || tower
                .template
                .suit
                .is_some_and(|suit| disabled_suits.contains(&suit))
        {
            continue;
        }

        let tower_center = tower_center_xy(tower);
        let Some(target_index) =
            select_target_index(monsters, tower_center, tower.attack_range_raw())
        else {
            continue;
        };
        let target = &monsters[target_index];
        let target_xy = monster_center_xy(target);
        let damage_raw = tower.attack_damage_raw();
        let attack_start = new_attacks.len();
        let attack_kind = match tower_attack_kind(tower.template.kind) {
            TowerAttackKind::Direct { .. } => 0,
            TowerAttackKind::Laser => 1,
            TowerAttackKind::FullHouseRain => 2,
            TowerAttackKind::RoyalStraightFlush => 3,
        };
        let source = crate::AttackSourceState {
            tower_id,
            tower_kind: tower.template.kind,
            rank: tower.template.rank,
            suit: tower.template.suit,
        };

        if !tower.on_attack_splashes.is_empty() {
            let source_index = area_damage_sources.len();
            area_damage_sources.push(source);
            area_damage_events.push(crate::AreaDamageEvent {
                center_xy: tower_center,
                damage_raw,
                source_index,
                splashes: tower.on_attack_splashes.clone(),
            });
        }

        let key = (tower_id << 32) ^ sim_tick ^ target.id;
        match tower_attack_kind(tower.template.kind) {
            TowerAttackKind::Direct { speed_raw } => {
                tower.cooldown = tower.shoot_interval_ticks();
                new_attacks.push(crate::InFlightAttackState {
                    id: next_entity_id.allocate_raw(),
                    damage_raw,
                    source_tower: Some(source),
                    kind: crate::InFlightAttackKindState::Spatial(crate::SpatialAttackState {
                        position: tower_head_xy(tower),
                        target_monster_id: target.id,
                        velocity: [0, -speed_raw],
                        behavior: crate::SpatialAttackBehaviorState::Direct,
                        movement_remainder: 0,
                        stable_key: key,
                    }),
                    on_hit_splashes: tower.on_hit_splashes.clone(),
                });
            }
            TowerAttackKind::Laser => {
                tower.cooldown = tower.shoot_interval_ticks();
                new_attacks.push(crate::InFlightAttackState {
                    id: next_entity_id.allocate_raw(),
                    damage_raw,
                    source_tower: Some(source),
                    kind: crate::InFlightAttackKindState::Laser(crate::LaserAttackState {
                        start_xy: tower_head_xy(tower),
                        end_xy: target_xy,
                        created_at: sim_tick,
                        target_monster_id: target.id,
                    }),
                    on_hit_splashes: tower.on_hit_splashes.clone(),
                });
            }
            TowerAttackKind::FullHouseRain => {
                tower.cooldown = tower.shoot_interval_ticks();
                for projectile_index in 0..4 {
                    let projectile_key = key ^ projectile_index as u64;
                    let damage_per_projectile =
                        split_damage_evenly(damage_raw, 4, projectile_index);
                    new_attacks.push(crate::InFlightAttackState {
                        id: next_entity_id.allocate_raw(),
                        damage_raw: damage_per_projectile,
                        source_tower: Some(source),
                        kind: crate::InFlightAttackKindState::Spatial(crate::SpatialAttackState {
                            position: tower_head_xy(tower),
                            target_monster_id: target.id,
                            velocity: [0, -homing_speed_for_key(projectile_key)],
                            behavior: crate::SpatialAttackBehaviorState::Homing {
                                velocity: [0, -homing_speed_for_key(projectile_key)],
                                acceleration_raw: HOMING_ACCELERATION_RAW,
                                turn_rate_raw: homing_turn_rate_for_key(projectile_key),
                                max_speed_raw: HOMING_MAX_SPEED_RAW,
                                acceleration_remainder: 0,
                                turn_remainder: 0,
                            },
                            movement_remainder: 0,
                            stable_key: projectile_key,
                        }),
                        on_hit_splashes: tower.on_hit_splashes.clone(),
                    });
                }
            }
            TowerAttackKind::RoyalStraightFlush => {
                tower.cooldown = tower.shoot_interval_ticks();
                new_attacks.push(crate::InFlightAttackState {
                    id: next_entity_id.allocate_raw(),
                    damage_raw,
                    source_tower: Some(source),
                    kind: crate::InFlightAttackKindState::Timed(crate::TimedAttackState {
                        target_monster_id: target.id,
                        execute_at: sim_tick.saturating_add(ROYAL_STRAIGHT_FLUSH_HIT_DELAY_TICKS),
                    }),
                    on_hit_splashes: tower.on_hit_splashes.clone(),
                });
            }
        }
        let attack_ids = new_attacks[attack_start..]
            .iter()
            .map(|attack| attack.id)
            .collect::<Vec<_>>();
        let projectile_attack_ids = new_attacks[attack_start..]
            .iter()
            .filter_map(|attack| {
                matches!(attack.kind, crate::InFlightAttackKindState::Spatial(_))
                    .then_some(attack.id)
            })
            .collect::<Vec<_>>();
        events.push(crate::CoreEvent::TowerAttack {
            tower_id,
            target_id: target.id,
            attack_kind,
            attack_ids,
            projectile_attack_ids,
        });
    }

    area_damage_events.sort_by_key(|event| {
        area_damage_sources
            .get(event.source_index)
            .map_or(u64::MAX, |source| source.tower_id)
    });
    new_attacks.sort_by_key(|attack| attack.id);
    in_flight_attacks.extend(new_attacks);
    in_flight_attacks.sort_by_key(|attack| attack.id);

    TowerAttackOutput {
        area_damage_events,
        area_damage_sources,
        events,
    }
}

#[derive(Clone, Copy)]
enum TowerAttackKind {
    Direct { speed_raw: i64 },
    Laser,
    FullHouseRain,
    RoyalStraightFlush,
}

const PROJECTILE_SPEED_RAW: i64 = 12 * crate::WORLD_UNITS_PER_TILE;
const FAST_PROJECTILE_SPEED_RAW: i64 = 16 * crate::WORLD_UNITS_PER_TILE;
const HOMING_INITIAL_SPEED_MIN_RAW: i64 = 24 * crate::WORLD_UNITS_PER_TILE;
const HOMING_INITIAL_SPEED_MAX_RAW: i64 = 32 * crate::WORLD_UNITS_PER_TILE;
const HOMING_MAX_SPEED_RAW: i64 = 36 * crate::WORLD_UNITS_PER_TILE;
const HOMING_ACCELERATION_RAW: i64 = 1024 * crate::WORLD_UNITS_PER_TILE;
const HOMING_TURN_RATE_MIN_RAW: i64 = 2_000_000;
const HOMING_TURN_RATE_MAX_RAW: i64 = 8_000_000;
const ROYAL_STRAIGHT_FLUSH_HIT_DELAY_TICKS: u64 = 26;

fn tower_attack_kind(kind: u8) -> TowerAttackKind {
    match kind {
        4 | 6 | 8 | 9 => TowerAttackKind::Direct {
            speed_raw: FAST_PROJECTILE_SPEED_RAW,
        },
        5 => TowerAttackKind::Laser,
        7 => TowerAttackKind::FullHouseRain,
        10 => TowerAttackKind::RoyalStraightFlush,
        _ => TowerAttackKind::Direct {
            speed_raw: PROJECTILE_SPEED_RAW,
        },
    }
}

fn select_target_index(
    monsters: &[crate::MonsterState],
    tower_center: [i64; 2],
    range_raw: i64,
) -> Option<usize> {
    let range_squared =
        u128::from(range_raw.max(0) as u64).saturating_mul(u128::from(range_raw.max(0) as u64));
    monsters
        .iter()
        .enumerate()
        .filter(|(_, monster)| {
            distance_squared(tower_center, monster_center_xy(monster)) <= range_squared
        })
        .max_by_key(|(_, monster)| {
            (
                monster.move_on_route.route_progress_raw,
                Reverse(distance_squared(tower_center, monster_center_xy(monster))),
                Reverse(monster.id),
            )
        })
        .map(|(index, _)| index)
}

fn split_damage_evenly(damage_raw: i64, parts: usize, index: usize) -> i64 {
    if parts == 0 {
        return 0;
    }
    let parts = parts as i64;
    let base = damage_raw / parts;
    let remainder = damage_raw % parts;
    base.saturating_add(i64::from((index as i64) < remainder))
}

fn homing_speed_for_key(key: u64) -> i64 {
    let span = HOMING_INITIAL_SPEED_MAX_RAW - HOMING_INITIAL_SPEED_MIN_RAW;
    HOMING_INITIAL_SPEED_MIN_RAW + (key % (span as u64 + 1)) as i64
}

fn homing_turn_rate_for_key(key: u64) -> i64 {
    let span = HOMING_TURN_RATE_MAX_RAW - HOMING_TURN_RATE_MIN_RAW;
    HOMING_TURN_RATE_MIN_RAW + (key % (span as u64 + 1)) as i64
}

fn distance_squared(left: [i64; 2], right: [i64; 2]) -> u128 {
    let dx = i128::from(left[0]) - i128::from(right[0]);
    let dy = i128::from(left[1]) - i128::from(right[1]);
    dx.unsigned_abs()
        .saturating_mul(dx.unsigned_abs())
        .saturating_add(dy.unsigned_abs().saturating_mul(dy.unsigned_abs()))
}

fn tower_center_xy(tower: &TowerState) -> [i64; 2] {
    [
        (tower.left_top[0] as i64 + 1)
            .saturating_mul(crate::WORLD_UNITS_PER_TILE)
            .saturating_add(crate::WORLD_UNITS_PER_TILE / 2),
        (tower.left_top[1] as i64 + 1)
            .saturating_mul(crate::WORLD_UNITS_PER_TILE)
            .saturating_add(crate::WORLD_UNITS_PER_TILE / 2),
    ]
}

fn tower_head_xy(tower: &TowerState) -> [i64; 2] {
    [
        (tower.left_top[0] as i64 + 1)
            .saturating_mul(crate::WORLD_UNITS_PER_TILE)
            .saturating_add(crate::WORLD_UNITS_PER_TILE / 2),
        (tower.left_top[1] as i64 + 1) * crate::WORLD_UNITS_PER_TILE,
    ]
}

fn monster_center_xy(monster: &crate::MonsterState) -> [i64; 2] {
    [
        monster.move_on_route.map_coord[0] + crate::WORLD_UNITS_PER_TILE / 2,
        monster.move_on_route.map_coord[1] + crate::WORLD_UNITS_PER_TILE / 2,
    ]
}

fn within_range(left: [i64; 2], right: [i64; 2], radius_raw: i64) -> bool {
    let dx = i128::from(left[0]) - i128::from(right[0]);
    let dy = i128::from(left[1]) - i128::from(right[1]);
    let radius = i128::from(radius_raw).unsigned_abs();
    dx.saturating_mul(dx).saturating_add(dy.saturating_mul(dy))
        <= radius.saturating_mul(radius) as i128
}

#[cfg(test)]
mod tests {
    use super::*;

    fn monster(id: u64, map_coord: [i64; 2], route_progress_raw: i64) -> crate::MonsterState {
        crate::MonsterState {
            id,
            move_on_route: crate::MoveOnRouteState {
                route: crate::RouteState {
                    map_coords: vec![],
                    world_coords: vec![],
                    segment_lengths: vec![],
                    cumulative_lengths: vec![],
                },
                route_index: 0,
                route_progress_raw,
                map_coord,
                velocity_raw: 0,
                movement_remainder: 0,
                motion_revision: 0,
            },
            kind: 0,
            hp_raw: 100,
            max_hp_raw: 100,
            stage_progress_counted: false,
            skills: vec![],
            status_effects: vec![],
            damage_raw: 0,
            reward: 0,
        }
    }

    fn tower(kind: u8) -> TowerState {
        TowerState {
            id: Some(1),
            left_top: [0, 0],
            cooldown: 0,
            template: TowerTemplateState {
                kind,
                rerolled_count: 0,
                shoot_interval: 7,
                default_attack_range_radius_raw: 5 * crate::WORLD_UNITS_PER_TILE,
                default_damage_raw: 100,
                suit: None,
                rank: None,
                skill_templates: vec![],
                default_status_effects: vec![],
                used_cards: vec![],
            },
            status_effects: vec![],
            skills: vec![],
            damage_multiplier_raw: crate::RATIO_SCALE,
            attack_range_radius_raw: 5 * crate::WORLD_UNITS_PER_TILE,
            effective_shoot_interval: 7,
            on_hit_splashes: vec![],
            on_attack_splashes: vec![],
        }
    }

    #[test]
    fn generated_attack_uses_deterministic_target_and_cooldown() {
        let mut towers = vec![tower(1)];
        let monsters = vec![
            monster(9, [2 * crate::WORLD_UNITS_PER_TILE, 0], 10),
            monster(4, [2 * crate::WORLD_UNITS_PER_TILE, 0], 10),
        ];
        let mut attacks = Vec::new();
        let mut allocator = crate::EntityIdAllocator::from_next_id(20);

        let output = generate_tower_attacks(
            &mut towers,
            &monsters,
            &mut attacks,
            &mut allocator,
            12,
            &[],
            &[],
        );

        assert!(output.area_damage_events.is_empty());
        assert_eq!(attacks.len(), 1);
        assert_eq!(attacks[0].id, 20);
        assert_eq!(attacks[0].damage_raw, 100);
        assert_eq!(towers[0].cooldown, 7);
        assert!(matches!(
            attacks[0].kind,
            crate::InFlightAttackKindState::Spatial(crate::SpatialAttackState {
                target_monster_id: 4,
                ..
            })
        ));
    }

    #[test]
    fn full_house_generates_four_homing_attacks() {
        let mut towers = vec![tower(7)];
        let monsters = vec![monster(4, [2 * crate::WORLD_UNITS_PER_TILE, 0], 10)];
        let mut attacks = Vec::new();
        let mut allocator = crate::EntityIdAllocator::from_next_id(20);

        generate_tower_attacks(
            &mut towers,
            &monsters,
            &mut attacks,
            &mut allocator,
            12,
            &[],
            &[],
        );

        assert_eq!(attacks.len(), 4);
        assert_eq!(
            attacks.iter().map(|attack| attack.damage_raw).sum::<i64>(),
            100
        );
        assert!(attacks.iter().all(|attack| matches!(
            attack.kind,
            crate::InFlightAttackKindState::Spatial(crate::SpatialAttackState {
                behavior: crate::SpatialAttackBehaviorState::Homing { .. },
                ..
            })
        )));
    }

    #[test]
    fn generated_damage_combines_status_and_upgrade_multipliers_once() {
        let mut tower = tower(1);
        tower.damage_multiplier_raw = 2 * crate::RATIO_SCALE;
        tower.status_effects = vec![
            TowerStatusEffect {
                kind: TowerStatusEffectKind::DamageAdd { add_raw: 10 },
                end: crate::TowerStatusEffectEnd::NeverEnd,
            },
            TowerStatusEffect {
                kind: TowerStatusEffectKind::DamageMul { mul_raw: 500_000 },
                end: crate::TowerStatusEffectEnd::NeverEnd,
            },
        ];
        let monsters = vec![monster(4, [2 * crate::WORLD_UNITS_PER_TILE, 0], 10)];
        let mut attacks = Vec::new();
        let mut allocator = crate::EntityIdAllocator::from_next_id(20);

        generate_tower_attacks(
            std::slice::from_mut(&mut tower),
            &monsters,
            &mut attacks,
            &mut allocator,
            12,
            &[],
            &[],
        );

        assert_eq!(attacks[0].damage_raw, 110);
    }
}
