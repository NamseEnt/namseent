use super::*;
use crate::*;

/// Headed projection/cache of the core upgrade collection.
///
/// The raw core collection is authoritative; this typed state exists for
/// selection presentation, render caches, and legacy serialization only.
#[derive(Debug, Clone, State, Default)]
pub struct UpgradeState {
    pub upgrades: Vec<UpgradeWithId>,
    pub revision: usize,
    cache: UpgradeCache,
}

/// Derived headed cache retained for legacy UI and persistence compatibility.
#[derive(Debug, Clone, State)]
pub struct UpgradeCache {
    pub max_hp_plus: HealthDelta,
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
            max_hp_plus: HealthDelta::ZERO,
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
    pub(crate) fn to_core_state(&self) -> td_core::UpgradeCacheState {
        td_core::UpgradeCacheState {
            max_hp_plus_raw: self.max_hp_plus.raw(),
            shop_slot_expand: self.shop_slot_expand,
            dice_chance_plus: self.dice_chance_plus,
            shop_item_price_minus: self.shop_item_price_minus,
            shorten_straight_flush_to_4_cards: self.shorten_straight_flush_to_4_cards,
            skip_rank_for_straight: self.skip_rank_for_straight,
            treat_suits_as_same: self.treat_suits_as_same,
            removed_number_rank_count: self.removed_number_rank_count,
            clear_shield_on_stage_start: self.clear_shield_on_stage_start,
        }
    }

    pub(crate) fn from_core_state(state: td_core::UpgradeCacheState) -> Self {
        Self {
            max_hp_plus: HealthDelta::from_raw(state.max_hp_plus_raw),
            shop_slot_expand: state.shop_slot_expand,
            dice_chance_plus: state.dice_chance_plus,
            shop_item_price_minus: state.shop_item_price_minus,
            shorten_straight_flush_to_4_cards: state.shorten_straight_flush_to_4_cards,
            skip_rank_for_straight: state.skip_rank_for_straight,
            treat_suits_as_same: state.treat_suits_as_same,
            removed_number_rank_count: state.removed_number_rank_count,
            clear_shield_on_stage_start: state.clear_shield_on_stage_start,
        }
    }

    pub fn from_state(state: &UpgradeState) -> Self {
        Self::from_core_state(state.to_core_state().cache_state())
    }
}

impl UpgradeState {
    /// Encodes the headed projection for a core observation or legacy codec.
    pub(crate) fn to_core_state(&self) -> td_core::UpgradeCollectionState {
        td_core::UpgradeCollectionState {
            upgrades: self
                .upgrades
                .iter()
                .copied()
                .map(UpgradeWithId::to_core_state)
                .collect(),
            revision: self.revision,
        }
    }

    pub(crate) fn from_core_state(state: td_core::UpgradeCollectionState) -> Option<Self> {
        let upgrades = state
            .upgrades
            .into_iter()
            .map(UpgradeWithId::from_core_state)
            .collect::<Option<Vec<_>>>()?;
        let mut ids = Vec::with_capacity(upgrades.len());
        for upgrade in &upgrades {
            if ids.contains(&upgrade.id) {
                return None;
            }
            ids.push(upgrade.id);
        }
        let mut restored = Self {
            upgrades,
            revision: state.revision,
            cache: UpgradeCache::default(),
        };
        restored.rebuild_cache();
        restored.normalize_cache_from_core();
        Some(restored)
    }

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

    pub(crate) fn normalize_cache_from_core(&mut self) {
        self.cache = UpgradeCache::from_core_state(self.cache.to_core_state());
    }

    pub fn clear_shield_on_stage_start(&self) -> bool {
        self.cache().clear_shield_on_stage_start
    }

    pub fn max_hp_plus(&self) -> HealthDelta {
        self.cache().max_hp_plus
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn cache_raw_state_round_trip_preserves_derived_payload() {
        let cache = UpgradeCache {
            max_hp_plus: HealthDelta::from_raw(1_250),
            shop_slot_expand: 2,
            dice_chance_plus: 3,
            shop_item_price_minus: 4,
            shorten_straight_flush_to_4_cards: true,
            skip_rank_for_straight: true,
            treat_suits_as_same: false,
            removed_number_rank_count: 5,
            clear_shield_on_stage_start: false,
        };

        let raw = cache.to_core_state();
        let restored = UpgradeCache::from_core_state(raw.clone());

        assert_eq!(restored.to_core_state(), raw);
    }

    #[test]
    fn upgrade_discriminants_have_lossless_core_mapping() {
        for discriminant in UpgradeDiscriminants::iter() {
            let raw = discriminant.to_core_raw();
            assert_eq!(UpgradeDiscriminants::from_core_raw(raw), Some(discriminant));
        }
        assert_eq!(UpgradeDiscriminants::from_core_raw(37), None);
    }

    #[test]
    fn upgrade_entry_identities_use_core_mapping() {
        let state = UpgradeState::with_upgrades(vec![
            Upgrade::Apple(AppleUpgrade),
            Upgrade::Banana(BananaUpgrade),
        ]);

        assert_eq!(
            state
                .to_core_state()
                .upgrades
                .into_iter()
                .map(|entry| entry.kind)
                .collect::<Vec<_>>(),
            vec![0, 1]
        );
    }

    #[test]
    fn upgrade_payloads_round_trip_through_core_state() {
        let state = UpgradeState::with_upgrades(vec![
            Upgrade::Backpack(BackpackUpgrade { add: 2 }),
            Upgrade::Crock(CrockUpgrade { current_step: 3 }),
            Upgrade::IceCream(IceCreamUpgrade {
                damage_bonus_pct: FixedRatio::from_raw(1_250_000),
                waves_remaining: 4,
            }),
            Upgrade::MembershipCard(MembershipCardUpgrade {
                pending_free_shop: true,
            }),
            Upgrade::NameTag(NameTagUpgrade {
                damage_bonus_pct: FixedRatio::from_integer(2),
                target_tower_id: Some(TowerId::from_raw(17)),
            }),
            Upgrade::Popcorn(PopcornUpgrade {
                max_multiplier: FixedRatio::from_raw(1_500_000),
                duration: 5,
                waves_remaining: 2,
                active_stage_damage_bonus: FixedRatio::from_raw(250_000),
            }),
            Upgrade::Resolution(ResolutionUpgrade {
                damage_bonus_pct_per_reroll: FixedRatio::from_raw(125_000),
                stored_rerolls: 6,
            }),
        ]);

        let raw = state.to_core_state();
        let restored = UpgradeState::from_core_state(raw.clone()).expect("valid upgrade state");

        assert_eq!(restored.to_core_state(), raw);
        assert_eq!(restored.upgrades.len(), 7);
    }
}
