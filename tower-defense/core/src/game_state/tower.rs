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
    pub immediate_damage_events: Vec<(u64, crate::DamageHit)>,
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

/// Combines card polish and an upgrade-only damage bonus into the single
/// ratio factor `attack_damage_raw`/`TowerTemplateState::effective_damage_raw`
/// apply to base damage. `upgrade_bonus_raw` must exclude card polish -
/// callers that already have a combined "upgrade + polish" bonus (e.g.
/// `UpgradeCollection::tower_damage_bonus_raw`) must not pass it here, or
/// polish gets applied twice.
fn combine_damage_factor_raw(card_polish_raw: i64, upgrade_bonus_raw: i64) -> i64 {
    let multiplier_raw = crate::RATIO_SCALE.saturating_add(upgrade_bonus_raw).max(0);
    let normalized_upgrade_bonus_raw = multiplier_raw.saturating_sub(crate::RATIO_SCALE);
    crate::RATIO_SCALE
        .saturating_add(card_polish_raw)
        .saturating_add(normalized_upgrade_bonus_raw)
}

impl TowerTemplateState {
    /// Authoritative on-attack splash effects derived from this template's
    /// `used_cards` engravings (e.g. Cactus). Shared by placement
    /// (`place_tower_with_template`) and observation
    /// (`tower_template_observation`) so both compute splash semantics from
    /// the same source of truth.
    pub fn derived_on_attack_splashes(&self) -> Vec<crate::DamageSplash> {
        if self.used_cards.iter().any(|card| card.engraving == Some(2)) {
            vec![crate::DamageSplash {
                radius_raw: 2 * crate::WORLD_UNITS_PER_TILE,
                damage_pct_raw: 300_000,
            }]
        } else {
            Vec::new()
        }
    }

    /// Authoritative on-hit splash effects derived from this template. No
    /// current engraving derives an on-hit splash, so this is empty.
    pub fn derived_on_hit_splashes(&self) -> Vec<crate::DamageSplash> {
        Vec::new()
    }

    pub fn card_polish_raw(&self) -> i64 {
        self.used_cards
            .iter()
            .map(|card| card.polish_pct_raw)
            .fold(0_i64, i64::saturating_add)
    }

    /// Authoritative deterministic effective attack damage for this
    /// template, given an explicit upgrade-only bonus (card polish is read
    /// from `used_cards` and applied internally - do not fold polish into
    /// `upgrade_bonus_raw`). This mirrors `TowerState::attack_damage_raw`
    /// minus runtime-only status effects, which do not exist before
    /// placement.
    pub fn effective_damage_raw(&self, upgrade_bonus_raw: i64) -> i64 {
        if self.default_damage_raw <= 0 {
            return 0;
        }
        let factor = combine_damage_factor_raw(self.card_polish_raw(), upgrade_bonus_raw);
        crate::apply_ratio_product_raw(self.default_damage_raw, &[factor])
    }
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
        let card_polish_raw = self.template.card_polish_raw();
        let upgrade_bonus_raw = self
            .damage_multiplier_raw
            .max(0)
            .saturating_sub(crate::RATIO_SCALE);
        multipliers.push(combine_damage_factor_raw(
            card_polish_raw,
            upgrade_bonus_raw,
        ));
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
    let mut immediate_damage_events = Vec::new();
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
        let mut attack_end_xy = target_xy;
        let damage_raw = tower.attack_damage_raw();
        let attack_start = new_attacks.len();
        let attack_kind = match tower_attack_kind(tower.template.kind) {
            TowerAttackKind::Direct { .. } => 0,
            TowerAttackKind::FullHouseRain => 2,
            TowerAttackKind::RoyalStraightFlush => 3,
            TowerAttackKind::InstantDamage
            | TowerAttackKind::ConeDamage
            | TowerAttackKind::StraightDamage
            | TowerAttackKind::FlushDamage => 4,
        };
        let source = crate::AttackSourceState {
            tower_id,
            tower_kind: tower.template.kind,
            rank: tower.template.rank,
            suit: tower.template.suit,
        };
        let mut tower_attack_hit_events = Vec::new();

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
            TowerAttackKind::InstantDamage => {
                tower.cooldown = tower.shoot_interval_ticks();
                let source_index = area_damage_sources.len();
                area_damage_sources.push(source);
                immediate_damage_events.push((
                    target.id,
                    crate::DamageHit {
                        target_index,
                        damage_raw,
                        at_xy: target_xy,
                        source_index,
                        splashes: tower.on_hit_splashes.clone(),
                    },
                ));
            }
            TowerAttackKind::ConeDamage => {
                tower.cooldown = tower.shoot_interval_ticks();
                let source_index = area_damage_sources.len();
                area_damage_sources.push(source);
                let aim_xy = if target_xy == tower_center {
                    [tower_center[0], tower_center[1].saturating_sub(1)]
                } else {
                    target_xy
                };
                let range_raw = tower.attack_range_raw().max(0);
                for (target_index, monster) in monsters.iter().enumerate() {
                    let monster_xy = monster_center_xy(monster);
                    if distance_squared(tower_center, monster_xy)
                        > (range_raw as u128).saturating_mul(range_raw as u128)
                        || !within_cone(tower_center, aim_xy, monster_xy)
                    {
                        continue;
                    }
                    immediate_damage_events.push((
                        monster.id,
                        crate::DamageHit {
                            target_index,
                            damage_raw,
                            at_xy: monster_xy,
                            source_index,
                            splashes: tower.on_hit_splashes.clone(),
                        },
                    ));
                    tower_attack_hit_events.push(crate::CoreEvent::TowerAttackHit {
                        tower_id,
                        target_id: monster.id,
                        tower_kind: tower.template.kind,
                        start_xy: tower_head_xy(tower),
                        position: monster_xy,
                    });
                }
            }
            TowerAttackKind::StraightDamage => {
                tower.cooldown = tower.shoot_interval_ticks();
                let source_index = area_damage_sources.len();
                area_damage_sources.push(source);
                let start_xy = tower_head_xy(tower);
                let range_raw = tower.attack_range_raw().max(0);
                attack_end_xy = straight_end_xy(start_xy, target_xy, range_raw);
                for (target_index, monster) in monsters.iter().enumerate() {
                    let monster_xy = monster_center_xy(monster);
                    if !within_straight(start_xy, target_xy, monster_xy, range_raw) {
                        continue;
                    }
                    immediate_damage_events.push((
                        monster.id,
                        crate::DamageHit {
                            target_index,
                            damage_raw,
                            at_xy: monster_xy,
                            source_index,
                            splashes: tower.on_hit_splashes.clone(),
                        },
                    ));
                    tower_attack_hit_events.push(crate::CoreEvent::TowerAttackHit {
                        tower_id,
                        target_id: monster.id,
                        tower_kind: tower.template.kind,
                        start_xy,
                        position: monster_xy,
                    });
                }
            }
            TowerAttackKind::FlushDamage => {
                tower.cooldown = tower.shoot_interval_ticks();
                let source_index = area_damage_sources.len();
                area_damage_sources.push(source);
                let radius_raw = 3 * crate::WORLD_UNITS_PER_TILE;
                let radius_squared = (radius_raw as u128).saturating_mul(radius_raw as u128);
                for (target_index, monster) in monsters.iter().enumerate() {
                    let monster_xy = monster_center_xy(monster);
                    if distance_squared(target_xy, monster_xy) > radius_squared {
                        continue;
                    }
                    immediate_damage_events.push((
                        monster.id,
                        crate::DamageHit {
                            target_index,
                            damage_raw,
                            at_xy: monster_xy,
                            source_index,
                            splashes: tower.on_hit_splashes.clone(),
                        },
                    ));
                    tower_attack_hit_events.push(crate::CoreEvent::TowerAttackHit {
                        tower_id,
                        target_id: monster.id,
                        tower_kind: tower.template.kind,
                        start_xy: target_xy,
                        position: monster_xy,
                    });
                }
            }
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
            TowerAttackKind::FullHouseRain => {
                tower.cooldown = tower.shoot_interval_ticks();
                attack_end_xy = tower_center;
                let range_raw = tower.attack_range_raw().max(0);
                let range_squared = (range_raw as u128).saturating_mul(range_raw as u128);
                let source_index = area_damage_sources.len();
                area_damage_sources.push(source);
                for (target_index, monster) in monsters.iter().enumerate() {
                    let monster_xy = monster_center_xy(monster);
                    if distance_squared(tower_center, monster_xy) > range_squared {
                        continue;
                    }
                    immediate_damage_events.push((
                        monster.id,
                        crate::DamageHit {
                            target_index,
                            damage_raw,
                            at_xy: monster_xy,
                            source_index,
                            splashes: tower.on_hit_splashes.clone(),
                        },
                    ));
                    tower_attack_hit_events.push(crate::CoreEvent::TowerAttackHit {
                        tower_id,
                        target_id: monster.id,
                        tower_kind: tower.template.kind,
                        start_xy: tower_head_xy(tower),
                        position: monster_xy,
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
            tower_kind: tower.template.kind,
            start_xy: tower_head_xy(tower),
            end_xy: attack_end_xy,
            attack_ids,
            projectile_attack_ids,
            attack_range_raw: tower.attack_range_raw(),
        });
        events.extend(tower_attack_hit_events);
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
        immediate_damage_events,
        events,
    }
}

#[derive(Clone, Copy)]
enum TowerAttackKind {
    Direct { speed_raw: i64 },
    InstantDamage,
    ConeDamage,
    StraightDamage,
    FlushDamage,
    FullHouseRain,
    RoyalStraightFlush,
}

const PROJECTILE_SPEED_RAW: i64 = 12 * crate::WORLD_UNITS_PER_TILE;
const FAST_PROJECTILE_SPEED_RAW: i64 = 16 * crate::WORLD_UNITS_PER_TILE;
const ROYAL_STRAIGHT_FLUSH_HIT_DELAY_TICKS: u64 = 26;
const HIGH_TOWER_KIND: u8 = 1;
const ONE_PAIR_TOWER_KIND: u8 = 2;
const TWO_PAIR_TOWER_KIND: u8 = 3;
const THREE_OF_A_KIND_TOWER_KIND: u8 = 4;
const STRAIGHT_TOWER_KIND: u8 = 5;

fn tower_attack_kind(kind: u8) -> TowerAttackKind {
    match kind {
        HIGH_TOWER_KIND | ONE_PAIR_TOWER_KIND | TWO_PAIR_TOWER_KIND => {
            TowerAttackKind::InstantDamage
        }
        THREE_OF_A_KIND_TOWER_KIND => TowerAttackKind::ConeDamage,
        8 | 9 => TowerAttackKind::Direct {
            speed_raw: FAST_PROJECTILE_SPEED_RAW,
        },
        STRAIGHT_TOWER_KIND => TowerAttackKind::StraightDamage,
        6 => TowerAttackKind::FlushDamage,
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

fn within_cone(origin: [i64; 2], aim: [i64; 2], target: [i64; 2]) -> bool {
    let direction_x = i128::from(aim[0]) - i128::from(origin[0]);
    let direction_y = i128::from(aim[1]) - i128::from(origin[1]);
    let target_x = i128::from(target[0]) - i128::from(origin[0]);
    let target_y = i128::from(target[1]) - i128::from(origin[1]);
    let direction_length_squared = distance_squared(origin, aim);
    let target_length_squared = distance_squared(origin, target);

    if target_length_squared == 0 {
        return true;
    }
    if direction_length_squared == 0 {
        return false;
    }

    let dot = direction_x
        .saturating_mul(target_x)
        .saturating_add(direction_y.saturating_mul(target_y));
    if dot <= 0 {
        return false;
    }

    let dot_squared = dot.unsigned_abs().saturating_mul(dot.unsigned_abs());
    dot_squared.saturating_mul(1_000_000)
        >= direction_length_squared
            .saturating_mul(target_length_squared)
            .saturating_mul(933_013)
}

fn straight_end_xy(start: [i64; 2], aim: [i64; 2], range_raw: i64) -> [i64; 2] {
    let dx = (aim[0] - start[0]) as f64;
    let dy = (aim[1] - start[1]) as f64;
    let length = dx.hypot(dy);
    if length <= f64::EPSILON || range_raw <= 0 {
        return start;
    }

    let range_ratio = range_raw as f64 / length;
    [
        (start[0] as f64 + dx * range_ratio).round() as i64,
        (start[1] as f64 + dy * range_ratio).round() as i64,
    ]
}

fn within_straight(start: [i64; 2], aim: [i64; 2], target: [i64; 2], range_raw: i64) -> bool {
    let direction_x = i128::from(aim[0]) - i128::from(start[0]);
    let direction_y = i128::from(aim[1]) - i128::from(start[1]);
    let target_x = i128::from(target[0]) - i128::from(start[0]);
    let target_y = i128::from(target[1]) - i128::from(start[1]);
    let direction_length_squared = direction_x
        .unsigned_abs()
        .saturating_mul(direction_x.unsigned_abs())
        .saturating_add(
            direction_y
                .unsigned_abs()
                .saturating_mul(direction_y.unsigned_abs()),
        );
    if direction_length_squared == 0 {
        return false;
    }

    let dot = direction_x
        .saturating_mul(target_x)
        .saturating_add(direction_y.saturating_mul(target_y));
    if dot <= 0 {
        return false;
    }
    let dot_squared = dot.unsigned_abs().saturating_mul(dot.unsigned_abs());
    let range = range_raw.max(0) as u128;
    if dot_squared
        > range
            .saturating_mul(range)
            .saturating_mul(direction_length_squared)
    {
        return false;
    }

    let cross = direction_x
        .saturating_mul(target_y)
        .saturating_sub(direction_y.saturating_mul(target_x));
    let cross_squared = cross.unsigned_abs().saturating_mul(cross.unsigned_abs());
    let half_width = (crate::WORLD_UNITS_PER_TILE / 4) as u128;
    cross_squared
        <= half_width
            .saturating_mul(half_width)
            .saturating_mul(direction_length_squared)
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
        (tower.left_top[0] as i64 + 1).saturating_mul(crate::WORLD_UNITS_PER_TILE),
        (tower.left_top[1] as i64 + 1).saturating_mul(crate::WORLD_UNITS_PER_TILE),
    ]
}

fn tower_head_xy(tower: &TowerState) -> [i64; 2] {
    let center_xy = tower_center_xy(tower);
    [
        center_xy[0],
        center_xy[1].saturating_sub(crate::WORLD_UNITS_PER_TILE / 4),
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
        let mut towers = vec![tower(0)];
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
    fn high_tower_generates_immediate_damage_without_in_flight_projectile() {
        let mut towers = vec![tower(1)];
        let monsters = vec![monster(4, [2 * crate::WORLD_UNITS_PER_TILE, 0], 10)];
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

        assert!(attacks.is_empty());
        assert_eq!(towers[0].cooldown, 7);
        assert_eq!(output.immediate_damage_events.len(), 1);
        assert_eq!(output.immediate_damage_events[0].0, 4);
        assert_eq!(output.immediate_damage_events[0].1.damage_raw, 100);
        assert!(output.events.iter().any(|event| matches!(
            event,
            crate::CoreEvent::TowerAttack {
                tower_id: 1,
                tower_kind: 1,
                attack_kind: 4,
                start_xy: [start_x, start_y],
                ..
            } if *start_x == 3 * crate::WORLD_UNITS_PER_TILE / 2
                && *start_y == 5 * crate::WORLD_UNITS_PER_TILE / 4
        )));
    }

    #[test]
    fn three_of_a_kind_hits_all_monsters_in_its_range_and_30_degree_cone() {
        let tile = crate::WORLD_UNITS_PER_TILE;
        let mut attack_tower = tower(THREE_OF_A_KIND_TOWER_KIND);
        attack_tower.left_top = [0, 5];
        let mut towers = vec![attack_tower];
        let monsters = vec![
            monster(4, [2 * tile, 5 * tile], 10),
            monster(5, [3 * tile, 4 * tile], 9),
            monster(6, [4 * tile, 3 * tile], 8),
            monster(7, [0, 3 * tile], 7),
            monster(8, [0, 6 * tile], 6),
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

        assert!(attacks.is_empty());
        assert_eq!(
            output
                .immediate_damage_events
                .iter()
                .map(|(target_id, _)| *target_id)
                .collect::<Vec<_>>(),
            vec![4, 5]
        );
        let hit_events = output
            .events
            .iter()
            .filter_map(|event| match event {
                crate::CoreEvent::TowerAttackHit {
                    target_id,
                    position,
                    ..
                } => Some((*target_id, *position)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            hit_events,
            vec![
                (4, monster_center_xy(&monsters[0])),
                (5, monster_center_xy(&monsters[1])),
            ]
        );
        assert!(
            output
                .immediate_damage_events
                .iter()
                .all(|(_, hit)| hit.damage_raw == 100)
        );
        assert_eq!(towers[0].cooldown, 7);
    }

    #[test]
    fn straight_hits_monsters_inside_its_half_tile_width_and_range() {
        let tile = crate::WORLD_UNITS_PER_TILE;
        let mut towers = vec![tower(STRAIGHT_TOWER_KIND)];
        let monsters = vec![
            monster(4, [2 * tile, tile / 4], 10),
            monster(5, [4 * tile, tile / 4], 9),
            monster(6, [4 * tile, tile / 2], 8),
            monster(7, [4 * tile, tile / 2 + 1], 7),
            monster(8, [0, tile / 4], 6),
            monster(9, [11 * tile / 2 + 1, tile / 4], 5),
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

        assert!(attacks.is_empty());
        assert_eq!(towers[0].cooldown, 7);
        assert_eq!(
            output
                .immediate_damage_events
                .iter()
                .map(|(target_id, _)| *target_id)
                .collect::<Vec<_>>(),
            vec![4, 5, 6]
        );
        assert!(
            output
                .immediate_damage_events
                .iter()
                .all(|(_, hit)| hit.damage_raw == 100)
        );
        assert!(output.events.iter().any(|event| matches!(
            event,
            crate::CoreEvent::TowerAttack {
                tower_kind: STRAIGHT_TOWER_KIND,
                start_xy,
                end_xy,
                ..
            } if *start_xy == [tile, 3 * tile / 4]
                && *end_xy == [6 * tile, 3 * tile / 4]
        )));
    }

    #[test]
    fn flush_hits_all_monsters_within_three_tiles_of_selected_target_immediately() {
        let tile = crate::WORLD_UNITS_PER_TILE;
        let mut attack_tower = tower(6);
        attack_tower.attack_range_radius_raw = 10 * tile;
        attack_tower.template.default_attack_range_radius_raw = 10 * tile;
        let mut towers = vec![attack_tower];
        let monsters = vec![
            monster(4, [2 * tile, 0], 10),
            monster(5, [4 * tile, 0], 9),
            monster(6, [5 * tile, 0], 8),
            monster(7, [6 * tile, 0], 7),
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

        let target_xy = monster_center_xy(&monsters[0]);
        assert!(attacks.is_empty());
        assert_eq!(towers[0].cooldown, 7);
        assert_eq!(
            output
                .immediate_damage_events
                .iter()
                .map(|(target_id, _)| *target_id)
                .collect::<Vec<_>>(),
            vec![4, 5, 6]
        );
        assert!(
            output
                .events
                .iter()
                .filter_map(|event| match event {
                    crate::CoreEvent::TowerAttackHit {
                        target_id,
                        start_xy,
                        ..
                    } => Some((*target_id, *start_xy)),
                    _ => None,
                })
                .all(|(_, start_xy)| start_xy == target_xy)
        );
        assert_eq!(
            output
                .immediate_damage_events
                .iter()
                .map(|(_, hit)| hit.damage_raw)
                .collect::<Vec<_>>(),
            vec![100; 3]
        );
    }

    #[test]
    fn three_of_a_kind_cone_includes_15_degree_boundary_and_excludes_outside_angle() {
        assert!(within_cone([0, 0], [0, -1_000], [267, -1_000]));
        assert!(!within_cone([0, 0], [0, -1_000], [268, -1_000]));
        assert!(!within_cone([0, 0], [0, -100], [0, 1]));
    }

    #[test]
    fn immediate_damage_towers_apply_damage_in_the_same_simulation_tick() {
        for tower_kind in [
            HIGH_TOWER_KIND,
            ONE_PAIR_TOWER_KIND,
            TWO_PAIR_TOWER_KIND,
            THREE_OF_A_KIND_TOWER_KIND,
            STRAIGHT_TOWER_KIND,
            7,
        ] {
            let mut state = crate::CoreState::new_initial(crate::GameConfig::default_config(), 7);
            let mut target = monster(4, [2 * crate::WORLD_UNITS_PER_TILE, 0], 10);
            target.move_on_route.route = state.route.clone();
            target.hp_raw = 500;
            target.max_hp_raw = 500;
            state.monsters.push(target);
            state.towers.push(tower(tower_kind));

            state.step_sim_tick();

            assert_eq!(state.monsters()[0].hp_raw, 400);
            assert!(state.in_flight_attacks().is_empty());
        }
    }

    #[test]
    fn full_house_damages_monsters_in_tower_range_immediately() {
        let tile = crate::WORLD_UNITS_PER_TILE;
        let mut attack_tower = tower(7);
        attack_tower.attack_range_radius_raw = 3 * tile;
        attack_tower.template.default_attack_range_radius_raw = 3 * tile;
        let mut towers = vec![attack_tower];
        let monsters = vec![
            monster(4, [2 * tile, 0], 10),
            monster(5, [3 * tile, 0], 9),
            monster(6, [4 * tile, 0], 8),
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

        assert!(attacks.is_empty());
        assert_eq!(towers[0].cooldown, 7);
        assert_eq!(
            output
                .immediate_damage_events
                .iter()
                .map(|(target_id, _)| *target_id)
                .collect::<Vec<_>>(),
            vec![4, 5]
        );
        assert!(
            output
                .immediate_damage_events
                .iter()
                .all(|(_, hit)| hit.damage_raw == 100)
        );
    }

    #[test]
    fn generated_damage_combines_status_and_upgrade_multipliers_once() {
        let mut tower = tower(0);
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
