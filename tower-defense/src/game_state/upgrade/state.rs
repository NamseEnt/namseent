use super::*;
use crate::{game_state::upgrade::tower::TowerUpgradeDamageBonus, *};

#[derive(Debug, Clone, State, Default)]
pub struct UpgradeState {
    pub upgrades: Vec<UpgradeWithId>,
    pub revision: usize,
    cache: UpgradeCache,
}

#[derive(Debug, Clone, State)]
pub struct UpgradeCache {
    pub max_hp_plus: usize,
    pub gold_earn_plus: usize,
    pub shop_slot_expand: usize,
    pub dice_chance_plus: usize,
    pub shop_item_price_minus: usize,
    pub shorten_straight_flush_to_4_cards: bool,
    pub skip_rank_for_straight: bool,
    pub treat_suits_as_same: bool,
    pub removed_number_rank_count: usize,
    pub clear_shield_on_stage_start: bool,
}

impl Default for UpgradeCache {
    fn default() -> Self {
        UpgradeCache {
            max_hp_plus: 0,
            gold_earn_plus: 0,
            shop_slot_expand: 0,
            dice_chance_plus: 0,
            shop_item_price_minus: 0,
            shorten_straight_flush_to_4_cards: false,
            skip_rank_for_straight: false,
            treat_suits_as_same: false,
            removed_number_rank_count: 0,
            clear_shield_on_stage_start: true,
        }
    }
}

impl UpgradeCache {
    pub fn from_state(state: &UpgradeState) -> Self {
        let mut cache = UpgradeCache::default();

        for upgrade in &state.upgrades {
            cache.max_hp_plus += upgrade.max_hp_plus() as usize;
            cache.gold_earn_plus += upgrade.gold_earn_plus();
            cache.shop_slot_expand += upgrade.shop_slot_expand();
            cache.dice_chance_plus += upgrade.dice_chance_plus();
            cache.shop_item_price_minus += upgrade.shop_item_price_minus();
            cache.shorten_straight_flush_to_4_cards |= upgrade.shorten_straight_flush_to_4_cards();
            cache.skip_rank_for_straight |= upgrade.skip_rank_for_straight();
            cache.treat_suits_as_same |= upgrade.treat_suits_as_same();
            cache.removed_number_rank_count += upgrade.removed_number_rank_count();
            cache.clear_shield_on_stage_start &= upgrade.clear_shield_on_stage_start();
        }

        cache
    }
}

impl UpgradeState {
    pub fn cache(&self) -> &UpgradeCache {
        &self.cache
    }

    pub fn with_upgrades(upgrades: Vec<Upgrade>) -> Self {
        let mut state = UpgradeState {
            upgrades: upgrades.into_iter().map(Upgrade::with_unique_id).collect(),
            ..Default::default()
        };
        state.cache = UpgradeCache::from_state(&state);
        state
    }

    pub(crate) fn rebuild_cache(&mut self) {
        self.cache = UpgradeCache::from_state(self);
    }

    pub fn clear_shield_on_stage_start(&self) -> bool {
        self.cache().clear_shield_on_stage_start
    }

    pub fn max_hp_plus(&self) -> usize {
        self.cache().max_hp_plus
    }

    pub fn gold_earn_plus(&self) -> usize {
        self.cache().gold_earn_plus
    }

    pub fn shop_slot_expand(&self) -> usize {
        self.cache().shop_slot_expand
    }

    pub fn dice_chance_plus(&self) -> usize {
        self.cache().dice_chance_plus
    }

    pub fn shop_item_price_minus(&self) -> usize {
        self.cache().shop_item_price_minus
    }

    pub fn shorten_straight_flush_to_4_cards(&self) -> bool {
        self.cache().shorten_straight_flush_to_4_cards
    }

    pub fn skip_rank_for_straight(&self) -> bool {
        self.cache().skip_rank_for_straight
    }

    pub fn treat_suits_as_same(&self) -> bool {
        self.cache().treat_suits_as_same
    }

    pub fn removed_number_rank_count(&self) -> usize {
        self.cache().removed_number_rank_count
    }

    pub fn tower_upgrade_damage_bonuses(&self) -> Vec<TowerUpgradeDamageBonus> {
        self.upgrades
            .iter()
            .filter_map(|upgrade| upgrade.tower_upgrade_damage_bonus())
            .map(|(target, bonus_pct)| TowerUpgradeDamageBonus { target, bonus_pct })
            .collect()
    }
}
