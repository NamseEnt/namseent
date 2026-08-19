//! Stage-wide transient & persistent modifiers extracted from former contract::mod.rs
//!
//! Responsibility:
//! - Aggregate per-stage combat/economy multipliers
//! - Track additive adjustments (bonus/penalty pairs) with net delta helpers
//! - Maintain reroll health costs
//! - Maintain temporary restrictions (disabled ranks/suits, purchase/use flags)
//! - Keep certain grants (rubber cone cards) persistent across stage resets
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
//! - Add incremental (additive) shield / rubber cone accumulation helpers
//! - Introduce a generic stacking abstraction if new modifier categories grow

use crate::card::{Rank, Suit};
use crate::game_state::tower::TowerKind;
use crate::*;

#[derive(Clone, Debug, Default, State)]
pub struct Multipliers {
    pub damage: RatioProduct,
    pub damage_reduction: RatioProduct,
    pub incoming_damage: RatioProduct,
    pub gold_gain: RatioProduct,
    pub enemy_health: RatioProduct,
    pub enemy_speed: RatioProduct,
}

#[derive(Clone, Debug, Default, State)]
pub struct Adjustments {
    pub card_selection_hand_max_slots_bonus: usize,
    pub card_selection_hand_max_slots_penalty: usize,
    pub max_dice_rerolls_bonus: usize,
    pub max_dice_rerolls_penalty: usize,
}

#[derive(Clone, Debug, Default, State)]
pub struct RerollCosts {
    pub reroll_health_cost: usize,
}

#[derive(Clone, Debug, Default, State)]
pub struct Restrictions {
    pub disable_item_and_upgrade_purchases: bool,
    pub disable_item_use: bool,
    pub free_shop_this_stage: bool,
    pub disabled_ranks: Vec<Rank>,
    pub disabled_suits: Vec<Suit>,
}

#[derive(Clone, Debug, Default, State)]
pub struct StageGrants {
    pub extra_tower_cards: Vec<(TowerKind, Option<Suit>, Option<Rank>)>,
    pub free_card_services: usize,
}

#[derive(Clone, Debug, State)]
pub struct StageModifiers {
    multipliers: Multipliers,
    adjustments: Adjustments,
    reroll_costs: RerollCosts,
    restrictions: Restrictions,
    stage_grants: StageGrants,
}

impl Default for StageModifiers {
    fn default() -> Self {
        Self::new()
    }
}

impl StageModifiers {
    pub fn new() -> Self {
        Self {
            multipliers: Multipliers {
                damage: RatioProduct::one(),
                damage_reduction: RatioProduct::one(),
                incoming_damage: RatioProduct::one(),
                gold_gain: RatioProduct::one(),
                enemy_health: RatioProduct::one(),
                enemy_speed: RatioProduct::one(),
            },
            adjustments: Adjustments::default(),
            reroll_costs: RerollCosts::default(),
            restrictions: Restrictions::default(),
            stage_grants: StageGrants::default(),
        }
    }

    pub fn reset_stage_state(&mut self) {
        self.multipliers = Multipliers {
            damage: RatioProduct::one(),
            damage_reduction: RatioProduct::one(),
            incoming_damage: RatioProduct::one(),
            gold_gain: RatioProduct::one(),
            enemy_health: RatioProduct::one(),
            enemy_speed: RatioProduct::one(),
        };
        self.adjustments = Adjustments::default();
        self.reroll_costs = RerollCosts::default();
        self.restrictions = Restrictions::default();
    }

    // ----- Getters -----
    pub fn get_damage_multiplier(&self) -> FixedRatio {
        self.multipliers.damage.combined_ratio()
    }
    pub fn get_damage_reduction_multiplier(&self) -> FixedRatio {
        self.multipliers.damage_reduction.combined_ratio()
    }
    pub fn get_incoming_damage_multiplier(&self) -> FixedRatio {
        self.multipliers.incoming_damage.combined_ratio()
    }
    pub fn get_gold_gain_multiplier(&self) -> FixedRatio {
        self.multipliers.gold_gain.combined_ratio()
    }
    pub fn get_enemy_health_multiplier(&self) -> FixedRatio {
        self.multipliers.enemy_health.combined_ratio()
    }
    pub fn get_enemy_speed_multiplier(&self) -> FixedRatio {
        self.multipliers.enemy_speed.combined_ratio()
    }
    pub(crate) fn damage_reduction_multipliers(&self) -> &[FixedRatio] {
        self.multipliers.damage_reduction.factors()
    }
    pub(crate) fn incoming_damage_multipliers(&self) -> &[FixedRatio] {
        self.multipliers.incoming_damage.factors()
    }
    pub(crate) fn gold_gain_multipliers(&self) -> &[FixedRatio] {
        self.multipliers.gold_gain.factors()
    }
    pub(crate) fn enemy_health_multipliers(&self) -> &RatioProduct {
        &self.multipliers.enemy_health
    }
    pub fn get_max_hand_slots_bonus(&self) -> usize {
        self.adjustments.card_selection_hand_max_slots_bonus
    }
    pub fn get_max_hand_slots_penalty(&self) -> usize {
        self.adjustments.card_selection_hand_max_slots_penalty
    }
    pub fn get_max_hand_slots_delta(&self) -> isize {
        self.adjustments.card_selection_hand_max_slots_bonus as isize
            - self.adjustments.card_selection_hand_max_slots_penalty as isize
    }

    pub fn get_max_rerolls_bonus(&self) -> usize {
        self.adjustments.max_dice_rerolls_bonus
    }
    pub fn get_max_rerolls_penalty(&self) -> usize {
        self.adjustments.max_dice_rerolls_penalty
    }
    pub fn get_max_rerolls_delta(&self) -> isize {
        self.adjustments.max_dice_rerolls_bonus as isize
            - self.adjustments.max_dice_rerolls_penalty as isize
    }
    pub fn is_item_and_upgrade_purchases_disabled(&self) -> bool {
        self.restrictions.disable_item_and_upgrade_purchases
    }
    pub fn is_item_use_disabled(&self) -> bool {
        self.restrictions.disable_item_use
    }
    pub fn is_free_shop_this_stage(&self) -> bool {
        self.restrictions.free_shop_this_stage
    }
    pub fn get_reroll_health_cost(&self) -> usize {
        self.reroll_costs.reroll_health_cost
    }
    pub fn get_disabled_ranks(&self) -> &Vec<Rank> {
        &self.restrictions.disabled_ranks
    }
    pub fn get_disabled_suits(&self) -> &Vec<Suit> {
        &self.restrictions.disabled_suits
    }
    pub fn drain_extra_tower_cards(&mut self) -> Vec<(TowerKind, Option<Suit>, Option<Rank>)> {
        std::mem::take(&mut self.stage_grants.extra_tower_cards)
    }

    pub fn drain_free_card_services(&mut self) -> usize {
        std::mem::take(&mut self.stage_grants.free_card_services)
    }

    // Net deltas (for testing)
    #[cfg(test)]
    pub fn get_card_selection_hand_max_slots_delta(&self) -> isize {
        self.adjustments.card_selection_hand_max_slots_bonus as isize
            - self.adjustments.card_selection_hand_max_slots_penalty as isize
    }

    #[cfg(test)]
    pub fn clear_stage_grants(&mut self) {
        self.stage_grants = StageGrants::default();
    }

    // ----- Mutators -----
    pub fn apply_damage_multiplier(&mut self, m: FixedRatio) {
        self.multipliers.damage.push(m);
    }
    pub fn apply_damage_reduction_multiplier(&mut self, m: FixedRatio) {
        self.multipliers.damage_reduction.push(m);
    }
    pub fn apply_incoming_damage_multiplier(&mut self, m: FixedRatio) {
        self.multipliers.incoming_damage.push(m);
    }
    pub fn apply_gold_gain_multiplier(&mut self, m: FixedRatio) {
        self.multipliers.gold_gain.push(m);
    }
    pub fn apply_enemy_health_multiplier(&mut self, m: FixedRatio) {
        self.multipliers.enemy_health.push(m);
    }

    pub fn apply_enemy_speed_multiplier(&mut self, m: FixedRatio) {
        self.multipliers.enemy_speed.push(m);
    }

    pub fn apply_max_hand_slots_bonus(&mut self, v: usize) {
        self.adjustments.card_selection_hand_max_slots_bonus += v;
    }
    pub fn apply_max_hand_slots_penalty(&mut self, v: usize) {
        self.adjustments.card_selection_hand_max_slots_penalty += v;
    }
    pub fn apply_max_rerolls_bonus(&mut self, v: usize) {
        self.adjustments.max_dice_rerolls_bonus += v;
    }
    pub fn apply_max_rerolls_penalty(&mut self, v: usize) {
        self.adjustments.max_dice_rerolls_penalty += v;
    }

    pub fn disable_item_and_upgrade_purchases(&mut self) {
        self.restrictions.disable_item_and_upgrade_purchases = true;
    }
    pub fn disable_item_use(&mut self) {
        self.restrictions.disable_item_use = true;
    }
    pub fn set_free_shop_this_stage(&mut self, enabled: bool) {
        self.restrictions.free_shop_this_stage = enabled;
    }
    pub fn apply_reroll_health_cost(&mut self, v: usize) {
        self.reroll_costs.reroll_health_cost += v;
    }

    pub fn disable_rank(&mut self, rank: Rank) {
        if !self.restrictions.disabled_ranks.contains(&rank) {
            self.restrictions.disabled_ranks.push(rank);
        }
    }
    pub fn disable_suit(&mut self, suit: Suit) {
        if !self.restrictions.disabled_suits.contains(&suit) {
            self.restrictions.disabled_suits.push(suit);
        }
    }

    pub fn enqueue_extra_tower_card(
        &mut self,
        kind: TowerKind,
        suit: Option<Suit>,
        rank: Option<Rank>,
    ) {
        self.stage_grants.extra_tower_cards.push((kind, suit, rank));
    }

    pub fn enqueue_free_card_service(&mut self) {
        self.stage_grants.free_card_services += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplier_stacks_are_order_independent_and_round_only_when_applied() {
        let factors = [
            FixedRatio::from_raw(500_001),
            FixedRatio::from_raw(500_001),
            FixedRatio::from_raw(771_635),
        ];
        let mut first = StageModifiers::new();
        let mut second = StageModifiers::new();
        for factor in factors {
            first.apply_enemy_health_multiplier(factor);
        }
        for factor in factors.into_iter().rev() {
            second.apply_enemy_health_multiplier(factor);
        }

        assert_eq!(
            first.get_enemy_health_multiplier(),
            second.get_enemy_health_multiplier()
        );
        assert_eq!(first.get_enemy_health_multiplier().raw(), 192_910);

        let base = Health::from_raw(1_086);
        let first_scaled = base.scaled_by_product(first.enemy_health_multipliers());
        let second_scaled = base.scaled_by_product(second.enemy_health_multipliers());
        assert_eq!(first_scaled, second_scaled);
        assert_eq!(first_scaled.raw(), 209);
    }
}
