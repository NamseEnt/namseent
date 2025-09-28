//! Stage-wide transient & persistent modifiers extracted from former contract::mod.rs
//!
//! Responsibility:
//! - Aggregate per-stage combat/economy multipliers
//! - Track additive adjustments (bonus/penalty pairs) with net delta helpers
//! - Maintain reroll health costs
//! - Maintain temporary restrictions (disabled ranks/suits, purchase/use flags)
//! - Keep certain grants (barricade cards, shield range) persistent across stage resets
//!
//! Lifecycle:
//! - Call `reset_stage_state` at stage start; this resets transient categories but leaves `StageGrants` intact
//! - Call `clear_stage_grants` only when you intentionally want to drop persistent grants
//!
//! Design Notes:
//! - Internally grouped into small structs for clarity & future serialization friendliness
//! - Net delta helpers return signed difference (bonus - penalty) for quick UI display / logic
//! - All multipliers are multiplicative stacks (default 1.0)
//!
//! Future Ideas:
//! - Consider serde derives if saving mid-run is needed
//! - Add incremental (additive) shield / barricade accumulation helpers
//! - Introduce a generic stacking abstraction if new modifier categories grow

use crate::card::{Rank, Suit};

#[derive(Clone, Debug, Default)]
pub struct Multipliers {
    pub damage: f32,
    pub attack_speed: f32,
    pub range: f32,
    pub damage_reduction: f32,
    pub incoming_damage: f32,
    pub gold_gain: f32,
    pub enemy_health: f32,
}

#[derive(Clone, Debug, Default)]
pub struct Adjustments {
    pub card_selection_hand_max_slots_bonus: usize,
    pub card_selection_hand_max_slots_penalty: usize,
    pub card_selection_hand_max_rerolls_bonus: usize,
    pub card_selection_hand_max_rerolls_penalty: usize,
    pub shop_max_rerolls_bonus: usize,
    pub shop_max_rerolls_penalty: usize,
}

#[derive(Clone, Debug, Default)]
pub struct RerollCosts {
    pub card_selection_hand_reroll_health_cost: usize,
    pub shop_reroll_health_cost: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Restrictions {
    pub disable_item_and_upgrade_purchases: bool,
    pub disable_item_use: bool,
    pub disabled_ranks: Vec<Rank>,
    pub disabled_suits: Vec<Suit>,
}

#[derive(Clone, Debug, Default)]
pub struct StageGrants {
    pub barricade_cards_per_stage: usize,
    pub shield_per_stage_min: f32,
    pub shield_per_stage_max: f32,
}

#[derive(Clone, Debug, Default)]
pub struct StageModifiers {
    multipliers: Multipliers,
    adjustments: Adjustments,
    reroll_costs: RerollCosts,
    restrictions: Restrictions,
    stage_grants: StageGrants,
}

impl StageModifiers {
    pub fn new() -> Self {
        Self {
            multipliers: Multipliers {
                damage: 1.0,
                attack_speed: 1.0,
                range: 1.0,
                damage_reduction: 1.0,
                incoming_damage: 1.0,
                gold_gain: 1.0,
                enemy_health: 1.0,
            },
            adjustments: Adjustments::default(),
            reroll_costs: RerollCosts::default(),
            restrictions: Restrictions::default(),
            stage_grants: StageGrants::default(),
        }
    }

    pub fn reset_stage_state(&mut self) {
        self.multipliers = Multipliers {
            damage: 1.0,
            attack_speed: 1.0,
            range: 1.0,
            damage_reduction: 1.0,
            incoming_damage: 1.0,
            gold_gain: 1.0,
            enemy_health: 1.0,
        };
        self.adjustments = Adjustments::default();
        self.reroll_costs = RerollCosts::default();
        self.restrictions = Restrictions::default();
    }

    pub fn clear_stage_grants(&mut self) {
        self.stage_grants = StageGrants::default();
    }

    // ----- Getters -----
    pub fn get_damage_multiplier(&self) -> f32 {
        self.multipliers.damage
    }
    pub fn get_attack_speed_multiplier(&self) -> f32 {
        self.multipliers.attack_speed
    }
    pub fn get_range_multiplier(&self) -> f32 {
        self.multipliers.range
    }
    pub fn get_damage_reduction_multiplier(&self) -> f32 {
        self.multipliers.damage_reduction
    }
    pub fn get_incoming_damage_multiplier(&self) -> f32 {
        self.multipliers.incoming_damage
    }
    pub fn get_gold_gain_multiplier(&self) -> f32 {
        self.multipliers.gold_gain
    }
    pub fn get_enemy_health_multiplier(&self) -> f32 {
        self.multipliers.enemy_health
    }
    pub fn get_card_selection_hand_max_slots_bonus(&self) -> usize {
        self.adjustments.card_selection_hand_max_slots_bonus
    }
    pub fn get_card_selection_hand_max_slots_penalty(&self) -> usize {
        self.adjustments.card_selection_hand_max_slots_penalty
    }
    pub fn get_card_selection_hand_max_rerolls_bonus(&self) -> usize {
        self.adjustments.card_selection_hand_max_rerolls_bonus
    }
    pub fn get_card_selection_hand_max_rerolls_penalty(&self) -> usize {
        self.adjustments.card_selection_hand_max_rerolls_penalty
    }
    pub fn get_shop_max_rerolls_bonus(&self) -> usize {
        self.adjustments.shop_max_rerolls_bonus
    }
    pub fn get_shop_max_rerolls_penalty(&self) -> usize {
        self.adjustments.shop_max_rerolls_penalty
    }
    pub fn is_item_and_upgrade_purchases_disabled(&self) -> bool {
        self.restrictions.disable_item_and_upgrade_purchases
    }
    pub fn is_item_use_disabled(&self) -> bool {
        self.restrictions.disable_item_use
    }
    pub fn get_card_selection_hand_reroll_health_cost(&self) -> usize {
        self.reroll_costs.card_selection_hand_reroll_health_cost
    }
    pub fn get_shop_reroll_health_cost(&self) -> usize {
        self.reroll_costs.shop_reroll_health_cost
    }
    pub fn get_disabled_ranks(&self) -> &Vec<Rank> {
        &self.restrictions.disabled_ranks
    }
    pub fn get_disabled_suits(&self) -> &Vec<Suit> {
        &self.restrictions.disabled_suits
    }
    pub fn get_shield_per_stage_min(&self) -> f32 {
        self.stage_grants.shield_per_stage_min
    }
    pub fn get_shield_per_stage_max(&self) -> f32 {
        self.stage_grants.shield_per_stage_max
    }
    pub fn get_barricade_cards_per_stage(&self) -> usize {
        self.stage_grants.barricade_cards_per_stage
    }

    // Net deltas
    pub fn get_card_selection_hand_max_slots_delta(&self) -> isize {
        self.adjustments.card_selection_hand_max_slots_bonus as isize
            - self.adjustments.card_selection_hand_max_slots_penalty as isize
    }
    pub fn get_card_selection_hand_max_rerolls_delta(&self) -> isize {
        self.adjustments.card_selection_hand_max_rerolls_bonus as isize
            - self.adjustments.card_selection_hand_max_rerolls_penalty as isize
    }
    pub fn get_shop_max_rerolls_delta(&self) -> isize {
        self.adjustments.shop_max_rerolls_bonus as isize
            - self.adjustments.shop_max_rerolls_penalty as isize
    }

    // ----- Mutators -----
    pub fn apply_damage_multiplier(&mut self, m: f32) {
        self.multipliers.damage *= m;
    }
    pub fn apply_attack_speed_multiplier(&mut self, m: f32) {
        self.multipliers.attack_speed *= m;
    }
    pub fn apply_range_multiplier(&mut self, m: f32) {
        self.multipliers.range *= m;
    }
    pub fn apply_damage_reduction_multiplier(&mut self, m: f32) {
        self.multipliers.damage_reduction *= m;
    }
    pub fn apply_incoming_damage_multiplier(&mut self, m: f32) {
        self.multipliers.incoming_damage *= m;
    }
    pub fn apply_gold_gain_multiplier(&mut self, m: f32) {
        self.multipliers.gold_gain *= m;
    }
    pub fn apply_enemy_health_multiplier(&mut self, m: f32) {
        self.multipliers.enemy_health *= m;
    }

    pub fn apply_card_selection_hand_max_slots_bonus(&mut self, v: usize) {
        self.adjustments.card_selection_hand_max_slots_bonus += v;
    }
    pub fn apply_card_selection_hand_max_slots_penalty(&mut self, v: usize) {
        self.adjustments.card_selection_hand_max_slots_penalty += v;
    }
    pub fn apply_card_selection_hand_max_rerolls_bonus(&mut self, v: usize) {
        self.adjustments.card_selection_hand_max_rerolls_bonus += v;
    }
    pub fn apply_card_selection_hand_max_rerolls_penalty(&mut self, v: usize) {
        self.adjustments.card_selection_hand_max_rerolls_penalty += v;
    }
    pub fn apply_shop_max_rerolls_bonus(&mut self, v: usize) {
        self.adjustments.shop_max_rerolls_bonus += v;
    }
    pub fn apply_shop_max_rerolls_penalty(&mut self, v: usize) {
        self.adjustments.shop_max_rerolls_penalty += v;
    }

    pub fn disable_item_and_upgrade_purchases(&mut self) {
        self.restrictions.disable_item_and_upgrade_purchases = true;
    }
    pub fn disable_item_use(&mut self) {
        self.restrictions.disable_item_use = true;
    }
    pub fn apply_card_selection_hand_reroll_health_cost(&mut self, v: usize) {
        self.reroll_costs.card_selection_hand_reroll_health_cost += v;
    }
    pub fn apply_shop_reroll_health_cost(&mut self, v: usize) {
        self.reroll_costs.shop_reroll_health_cost += v;
    }

    pub fn disable_rank(&mut self, rank: Rank) {
        if !self.restrictions.disabled_ranks.contains(&rank) {
            self.restrictions.disabled_ranks.push(rank);
        }
    }
    pub fn is_rank_disabled(&self, rank: Rank) -> bool {
        self.restrictions.disabled_ranks.contains(&rank)
    }
    pub fn disable_suit(&mut self, suit: Suit) {
        if !self.restrictions.disabled_suits.contains(&suit) {
            self.restrictions.disabled_suits.push(suit);
        }
    }
    pub fn is_suit_disabled(&self, suit: Suit) -> bool {
        self.restrictions.disabled_suits.contains(&suit)
    }

    pub fn set_barricade_cards_per_stage(&mut self, c: usize) {
        self.stage_grants.barricade_cards_per_stage = c;
    }
    pub fn set_shield_per_stage(&mut self, min_a: f32, max_a: f32) {
        self.stage_grants.shield_per_stage_min = min_a;
        self.stage_grants.shield_per_stage_max = max_a;
    }
}
