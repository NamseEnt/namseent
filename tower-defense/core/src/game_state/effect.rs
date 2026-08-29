use crate::{MonsterState, TowerState, multiply_ratio_raw};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UserStatusEffect {
    pub kind: UserStatusEffectKind,
    pub end_at: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UserStatusEffectKind {
    DamageReduction { damage_multiply_raw: i64 },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MonsterStatusEffect {
    pub kind: MonsterStatusEffectKind,
    pub end_at: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MonsterStatusEffectKind {
    SpeedMul { mul_raw: i64 },
    Invincible,
    ImmuneToSlow,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerStatusEffect {
    pub kind: TowerStatusEffectKind,
    pub end: TowerStatusEffectEnd,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TowerStatusEffectKind {
    DamageMul { mul_raw: i64 },
    DamageAdd { add_raw: i64 },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TowerStatusEffectEnd {
    Time { end_at: u64 },
    NeverEnd,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StageModifierTowerCardState {
    pub kind: u8,
    pub suit: Option<u8>,
    pub rank: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StageModifiersState {
    pub damage_multipliers_raw: Vec<i64>,
    pub damage_reduction_multipliers_raw: Vec<i64>,
    pub incoming_damage_multipliers_raw: Vec<i64>,
    pub gold_gain_multipliers_raw: Vec<i64>,
    pub enemy_health_multipliers_raw: Vec<i64>,
    pub enemy_speed_multipliers_raw: Vec<i64>,
    pub card_selection_hand_max_slots_bonus: usize,
    pub card_selection_hand_max_slots_penalty: usize,
    pub max_dice_rerolls_bonus: usize,
    pub max_dice_rerolls_penalty: usize,
    pub reroll_health_cost: usize,
    pub disable_item_and_upgrade_purchases: bool,
    pub disable_item_use: bool,
    pub free_shop_this_stage: bool,
    pub disabled_ranks: Vec<u8>,
    pub disabled_suits: Vec<u8>,
    pub extra_tower_cards: Vec<StageModifierTowerCardState>,
    pub free_card_services: usize,
}

impl StageModifiersState {
    pub(crate) fn reset_stage_state(&mut self) {
        self.damage_multipliers_raw.clear();
        self.damage_reduction_multipliers_raw.clear();
        self.incoming_damage_multipliers_raw.clear();
        self.gold_gain_multipliers_raw.clear();
        self.enemy_health_multipliers_raw.clear();
        self.enemy_speed_multipliers_raw.clear();
        self.card_selection_hand_max_slots_bonus = 0;
        self.card_selection_hand_max_slots_penalty = 0;
        self.max_dice_rerolls_bonus = 0;
        self.max_dice_rerolls_penalty = 0;
        self.reroll_health_cost = 0;
        self.disable_item_and_upgrade_purchases = false;
        self.disable_item_use = false;
        self.free_shop_this_stage = false;
        self.disabled_ranks.clear();
        self.disabled_suits.clear();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StageModifiersObservation {
    pub damage_multiplier_raw: i64,
    pub damage_reduction_multiplier_raw: i64,
    pub incoming_damage_multiplier_raw: i64,
    pub gold_gain_multiplier_raw: i64,
    pub enemy_health_multiplier_raw: i64,
    pub enemy_speed_multiplier_raw: i64,
    pub max_hand_slots_delta: isize,
    pub max_rerolls_delta: isize,
    pub reroll_health_cost: usize,
    pub item_use_disabled: bool,
    pub purchases_disabled: bool,
    pub free_shop: bool,
}

pub(crate) fn remove_expired_tower_statuses(towers: &mut [TowerState], sim_tick: u64) -> Vec<u64> {
    let mut damage_refresh_tower_ids = Vec::new();
    for tower in towers {
        let before = tower.status_effects.len();
        tower.status_effects.retain(|effect| match effect.end {
            TowerStatusEffectEnd::Time { end_at } => sim_tick < end_at,
            TowerStatusEffectEnd::NeverEnd => true,
        });
        if tower.status_effects.len() != before
            && let Some(tower_id) = tower.id
        {
            damage_refresh_tower_ids.push(tower_id);
        }
    }
    damage_refresh_tower_ids
}

pub(crate) fn remove_expired_monster_statuses(monsters: &mut [MonsterState], sim_tick: u64) {
    for monster in monsters {
        monster
            .status_effects
            .retain(|effect| effect.end_at > sim_tick);
    }
}

pub(crate) fn remove_expired_user_status_effects(
    effects: &mut Vec<UserStatusEffect>,
    sim_tick: u64,
) {
    effects.retain(|effect| effect.end_at > sim_tick);
}

pub(crate) fn adjust_incoming_damage(
    damage_raw: i64,
    user_effects: &[UserStatusEffect],
    damage_reduction_multipliers_raw: &[i64],
    incoming_damage_multipliers_raw: &[i64],
) -> i64 {
    let mut adjusted = damage_raw;
    for effect in user_effects {
        let UserStatusEffectKind::DamageReduction {
            damage_multiply_raw,
        } = effect.kind;
        adjusted = multiply_ratio_raw(adjusted, damage_multiply_raw);
    }
    for multiplier in damage_reduction_multipliers_raw
        .iter()
        .chain(incoming_damage_multipliers_raw)
    {
        adjusted = multiply_ratio_raw(adjusted, *multiplier);
    }
    adjusted
}
