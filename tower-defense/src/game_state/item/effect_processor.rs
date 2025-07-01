use crate::{
    MapCoordF32,
    card::{Rank, Suit},
    game_state::{
        GameState,
        field_area_effect::{FieldAreaEffect, FieldAreaEffectKind},
        field_particle::{FieldParticleKind, emit_field_particle},
        item::{check_point_is_in_linear_area, linear_area_rect_points},
        monster::{MonsterStatusEffect, MonsterStatusEffectKind},
        quest::{QuestTriggerEvent, on_quest_trigger_event},
        schedule::CountBasedSchedule,
        tower::{TowerStatusEffect, TowerStatusEffectEnd, TowerStatusEffectKind},
        upgrade::UpgradeState,
        user_status_effect::{UserStatusEffect, UserStatusEffectKind},
    },
};
use namui::*;

const INVALID_EFFECT_TEMPLATE_MESSAGE: &str = "Invalid status effect template for target type";

#[derive(Clone, Debug)]
pub enum ItemEffectKind {
    InstantStatus {
        target_type: EffectTargetType,
        effect: StatusEffectTemplate,
        area: EffectArea,
    },
    FieldArea {
        effect: FieldAreaEffectKind,
        schedule: CountBasedSchedule,
    },
    Direct {
        effect: DirectEffectKind,
    },
}

#[derive(Clone, Debug)]
pub enum EffectTargetType {
    Tower,
    Monster,
    User,
}

#[derive(Clone, Debug)]
pub enum EffectArea {
    Point,
    Circle { xy: MapCoordF32, radius: f32 },
    Line { xy: MapCoordF32, thickness: f32 },
}

#[derive(Clone, Debug)]
pub enum StatusEffectTemplate {
    TowerDamageAdd { add: f32, duration: Duration },
    TowerDamageMultiply { multiply: f32, duration: Duration },
    TowerAttackSpeedAdd { add: f32, duration: Duration },
    TowerAttackSpeedMultiply { multiply: f32, duration: Duration },
    TowerAttackRangeAdd { add: f32, duration: Duration },
    MonsterSpeedMultiply { multiply: f32, duration: Duration },
    UserDamageReduction { multiply: f32, duration: Duration },
}

#[derive(Clone, Debug)]
pub enum DirectEffectKind {
    Heal {
        amount: f32,
    },
    Shield {
        amount: f32,
    },
    ExtraReroll,
    EarnGold {
        amount: usize,
    },
    RoundDamage {
        rank: crate::card::Rank,
        suit: crate::card::Suit,
        damage: f32,
        xy: MapCoordF32,
        radius: f32,
    },
    LinearDamage {
        rank: crate::card::Rank,
        suit: crate::card::Suit,
        damage: f32,
        center_xy: MapCoordF32,
        target_xy: MapCoordF32,
        thickness: f32,
    },
}

pub fn process_item_effect(game_state: &mut GameState, effect_kind: ItemEffectKind) {
    match effect_kind {
        ItemEffectKind::InstantStatus {
            target_type,
            effect,
            area,
        } => {
            apply_instant_status_effect(game_state, target_type, effect, area);
        }
        ItemEffectKind::FieldArea { effect, schedule } => {
            create_field_area_effect(game_state, effect, schedule);
        }
        ItemEffectKind::Direct { effect } => {
            apply_direct_effect(game_state, effect);
        }
    }
}

fn apply_instant_status_effect(
    game_state: &mut GameState,
    target_type: EffectTargetType,
    effect_template: StatusEffectTemplate,
    area: EffectArea,
) {
    match target_type {
        EffectTargetType::Tower => {
            if let Some((xy, radius)) = validate_circle_area(area) {
                let status_effect = create_tower_status_effect(effect_template, game_state.now());
                add_tower_status_effect_in_round_area(game_state, xy, radius, status_effect);
            }
        }
        EffectTargetType::Monster => {
            if let Some((xy, radius)) = validate_circle_area(area) {
                let status_effect = create_monster_status_effect(effect_template, game_state.now());
                add_monster_status_effect_in_round_area(game_state, xy, radius, status_effect);
            }
        }
        EffectTargetType::User => {
            let status_effect = create_user_status_effect(effect_template, game_state.now());
            game_state.user_status_effects.push(status_effect);
        }
    }
}

fn create_tower_status_effect(template: StatusEffectTemplate, now: Instant) -> TowerStatusEffect {
    let end_time_factory = |duration| create_status_effect_end_time(now, duration);

    match template {
        StatusEffectTemplate::TowerDamageAdd { add, duration } => TowerStatusEffect {
            kind: TowerStatusEffectKind::DamageAdd { add },
            end_at: end_time_factory(duration),
        },
        StatusEffectTemplate::TowerDamageMultiply { multiply, duration } => TowerStatusEffect {
            kind: TowerStatusEffectKind::DamageMul { mul: multiply },
            end_at: end_time_factory(duration),
        },
        StatusEffectTemplate::TowerAttackSpeedAdd { add, duration } => TowerStatusEffect {
            kind: TowerStatusEffectKind::AttackSpeedAdd { add },
            end_at: end_time_factory(duration),
        },
        StatusEffectTemplate::TowerAttackSpeedMultiply { multiply, duration } => {
            TowerStatusEffect {
                kind: TowerStatusEffectKind::AttackSpeedMul { mul: multiply },
                end_at: end_time_factory(duration),
            }
        }
        StatusEffectTemplate::TowerAttackRangeAdd { add, duration } => TowerStatusEffect {
            kind: TowerStatusEffectKind::AttackRangeAdd { add },
            end_at: end_time_factory(duration),
        },
        _ => panic!("{}: tower", INVALID_EFFECT_TEMPLATE_MESSAGE),
    }
}

fn create_monster_status_effect(
    template: StatusEffectTemplate,
    now: Instant,
) -> MonsterStatusEffect {
    match template {
        StatusEffectTemplate::MonsterSpeedMultiply { multiply, duration } => MonsterStatusEffect {
            kind: MonsterStatusEffectKind::SpeedMul { mul: multiply },
            end_at: now + duration,
        },
        _ => panic!("{}: monster", INVALID_EFFECT_TEMPLATE_MESSAGE),
    }
}

fn create_user_status_effect(template: StatusEffectTemplate, now: Instant) -> UserStatusEffect {
    match template {
        StatusEffectTemplate::UserDamageReduction { multiply, duration } => UserStatusEffect {
            kind: UserStatusEffectKind::DamageReduction {
                damage_multiply: multiply,
            },
            end_at: now + duration,
        },
        _ => panic!("{}: user", INVALID_EFFECT_TEMPLATE_MESSAGE),
    }
}

fn create_field_area_effect(
    game_state: &mut GameState,
    effect: FieldAreaEffectKind,
    schedule: CountBasedSchedule,
) {
    let field_area_effect = FieldAreaEffect::new(effect.clone(), schedule);
    emit_field_particle(
        game_state,
        FieldParticleKind::FieldAreaEffect {
            field_area_effect: &field_area_effect,
        },
    );
    game_state.field_area_effects.push(field_area_effect);
}

fn apply_direct_effect(game_state: &mut GameState, effect: DirectEffectKind) {
    match effect {
        DirectEffectKind::Heal { amount } => {
            game_state.hp = (game_state.hp + amount).min(crate::game_state::MAX_HP);
        }
        DirectEffectKind::Shield { amount } => {
            game_state.shield += amount;
        }
        DirectEffectKind::ExtraReroll => {
            game_state.left_reroll_chance += 1;
        }
        DirectEffectKind::EarnGold { amount } => {
            game_state.earn_gold(amount);
        }
        DirectEffectKind::RoundDamage {
            rank,
            suit,
            damage,
            xy,
            radius,
        } => {
            apply_instant_round_damage(game_state, rank, suit, damage, xy, radius);
        }
        DirectEffectKind::LinearDamage {
            rank,
            suit,
            damage,
            center_xy,
            target_xy,
            thickness,
        } => {
            apply_instant_linear_damage(
                game_state, rank, suit, damage, center_xy, target_xy, thickness,
            );
        }
    }
}

fn add_tower_status_effect_in_round_area(
    game_state: &mut GameState,
    xy: MapCoordF32,
    radius: f32,
    status_effect: TowerStatusEffect,
) {
    for tower in game_state.towers.iter_mut() {
        if xy.distance(tower.center_xy_f32()) <= radius {
            tower.status_effects.push(status_effect.clone());
        }
    }
}

fn add_monster_status_effect_in_round_area(
    game_state: &mut GameState,
    xy: MapCoordF32,
    radius: f32,
    status_effect: MonsterStatusEffect,
) {
    for monster in game_state.monsters.iter_mut() {
        if xy.distance(monster.xy()) <= radius {
            monster.status_effects.push(status_effect.clone());
        }
    }
}

fn apply_instant_round_damage(
    game_state: &mut GameState,
    rank: Rank,
    suit: Suit,
    damage: f32,
    xy: MapCoordF32,
    radius: f32,
) {
    let mut total_damage = damage;
    apply_rank_and_suit_upgrades(&game_state.upgrade_state, rank, suit, &mut total_damage);

    let result = process_monster_damage(game_state, total_damage, |monster| {
        monster.xy().distance(xy) <= radius
    });

    result.finalize(game_state);
}

fn apply_instant_linear_damage(
    game_state: &mut GameState,
    rank: Rank,
    suit: Suit,
    damage: f32,
    center_xy: MapCoordF32,
    target_xy: MapCoordF32,
    thickness: f32,
) {
    let mut total_damage = damage;
    apply_rank_and_suit_upgrades(&game_state.upgrade_state, rank, suit, &mut total_damage);

    let points = linear_area_rect_points(center_xy, target_xy, thickness);
    let result = process_monster_damage(game_state, total_damage, |monster| {
        check_point_is_in_linear_area(&points, monster.xy())
    });

    result.finalize(game_state);
}

fn apply_rank_and_suit_upgrades(
    upgrade_state: &UpgradeState,
    rank: Rank,
    suit: Suit,
    damage: &mut f32,
) {
    use crate::game_state::upgrade::TowerUpgradeTarget;

    let rank_upgrades = upgrade_state
        .tower_upgrade_states
        .get(&TowerUpgradeTarget::Rank { rank });
    let suit_upgrades = upgrade_state
        .tower_upgrade_states
        .get(&TowerUpgradeTarget::Suit { suit });

    for upgrade in rank_upgrades.iter().chain(suit_upgrades.iter()) {
        *damage += upgrade.damage_plus;
    }

    for upgrade in rank_upgrades.iter().chain(suit_upgrades.iter()) {
        *damage *= upgrade.damage_multiplier;
    }
}

struct DamageApplicationResult {
    total_damage_dealt: f32,
    total_gold_earned: usize,
}

impl DamageApplicationResult {
    fn new() -> Self {
        Self {
            total_damage_dealt: 0.0,
            total_gold_earned: 0,
        }
    }

    fn add_damage(&mut self, damage: f32) {
        self.total_damage_dealt += damage;
    }

    fn add_gold(&mut self, gold: usize) {
        self.total_gold_earned += gold;
    }

    fn finalize(self, game_state: &mut GameState) {
        if self.total_gold_earned > 0 {
            game_state.earn_gold(self.total_gold_earned);
        }

        if self.total_damage_dealt > 0.0 {
            on_quest_trigger_event(
                game_state,
                QuestTriggerEvent::DealDamageWithItem {
                    damage: self.total_damage_dealt,
                },
            );
        }
    }
}

fn process_monster_damage<F>(
    game_state: &mut GameState,
    damage: f32,
    area_filter: F,
) -> DamageApplicationResult
where
    F: Fn(&crate::game_state::monster::Monster) -> bool,
{
    let mut result = DamageApplicationResult::new();

    game_state.monsters.retain_mut(|monster| {
        if !area_filter(monster) {
            return true;
        }

        monster.get_damage(damage);
        result.add_damage(damage);

        if monster.dead() {
            let earn = monster.reward + game_state.upgrade_state.gold_earn_plus;
            result.add_gold(earn);
            return false;
        }

        true
    });

    result
}

fn validate_circle_area(area: EffectArea) -> Option<(MapCoordF32, f32)> {
    match area {
        EffectArea::Circle { xy, radius } => Some((xy, radius)),
        _ => None,
    }
}

fn create_status_effect_end_time(now: Instant, duration: Duration) -> TowerStatusEffectEnd {
    TowerStatusEffectEnd::Time {
        end_at: now + duration,
    }
}
