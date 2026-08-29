mod behaviors;
mod definition;

use rand::seq::SliceRandom;

use definition::definition;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UpgradeCacheState {
    pub max_hp_plus_raw: i64,
    pub shop_slot_expand: usize,
    pub dice_chance_plus: usize,
    pub shop_item_price_minus: usize,
    pub shorten_straight_flush_to_4_cards: bool,
    pub skip_rank_for_straight: bool,
    pub treat_suits_as_same: bool,
    pub removed_number_rank_count: usize,
    pub clear_shield_on_stage_start: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UpgradeEntryIdentityState {
    pub id: u64,
    pub kind: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UpgradeEntryState {
    pub id: u64,
    pub kind: u8,
    pub scalar_values: Vec<u64>,
    pub ratio_values_raw: Vec<i64>,
    pub bool_values: Vec<bool>,
    pub optional_ids: Vec<Option<u64>>,
}

impl UpgradeEntryState {
    /// Parses the persisted kind at the boundary where the entry enters core
    /// upgrade semantics.
    pub fn upgrade_kind(&self) -> Result<crate::UpgradeKind, crate::CommandError> {
        crate::UpgradeKind::from_raw(self.kind)
            .ok_or(crate::CommandError::InvalidUpgradeKind { raw: self.kind })
    }

    /// Returns the persisted kind without interpreting it.
    pub const fn upgrade_kind_raw(&self) -> u8 {
        self.kind
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UpgradeCollectionState {
    pub upgrades: Vec<UpgradeEntryState>,
    pub revision: usize,
}

impl UpgradeCollectionState {
    pub fn cache_state(&self) -> UpgradeCacheState {
        let contributions = self
            .upgrades
            .iter()
            .map(|upgrade| {
                let kind = upgrade
                    .upgrade_kind()
                    .expect("persisted upgrade kind must be valid before cache calculation");
                (definition(kind).cache)(upgrade)
            })
            .fold(
                UpgradeCacheContribution {
                    clear_shield_on_stage_start: true,
                    ..UpgradeCacheContribution::default()
                },
                |mut total, value| {
                    total.max_hp_plus_raw =
                        total.max_hp_plus_raw.saturating_add(value.max_hp_plus_raw);
                    total.shop_slot_expand = total
                        .shop_slot_expand
                        .saturating_add(value.shop_slot_expand);
                    total.dice_chance_plus = total
                        .dice_chance_plus
                        .saturating_add(value.dice_chance_plus);
                    total.shop_item_price_minus = total
                        .shop_item_price_minus
                        .saturating_add(value.shop_item_price_minus);
                    total.shorten_straight_flush_to_4_cards |=
                        value.shorten_straight_flush_to_4_cards;
                    total.skip_rank_for_straight |= value.skip_rank_for_straight;
                    total.treat_suits_as_same |= value.treat_suits_as_same;
                    total.clear_shield_on_stage_start &= value.clear_shield_on_stage_start;
                    total
                },
            );
        UpgradeCacheState {
            max_hp_plus_raw: contributions.max_hp_plus_raw,
            shop_slot_expand: contributions.shop_slot_expand,
            dice_chance_plus: contributions.dice_chance_plus,
            shop_item_price_minus: contributions.shop_item_price_minus,
            shorten_straight_flush_to_4_cards: contributions.shorten_straight_flush_to_4_cards,
            skip_rank_for_straight: contributions.skip_rank_for_straight,
            treat_suits_as_same: contributions.treat_suits_as_same,
            removed_number_rank_count: 0,
            clear_shield_on_stage_start: contributions.clear_shield_on_stage_start,
        }
    }

    pub fn shorten_straight_flush_to_4_cards(&self) -> bool {
        self.cache_state().shorten_straight_flush_to_4_cards
    }

    pub fn skip_rank_for_straight(&self) -> bool {
        self.cache_state().skip_rank_for_straight
    }

    pub fn treat_suits_as_same(&self) -> bool {
        self.cache_state().treat_suits_as_same
    }

    pub fn tower_damage_bonus_raw(&self, tower: &crate::TowerState) -> i64 {
        let upgrade_bonus = self.tower_upgrade_bonus_raw(tower);
        let polish_bonus = tower
            .template
            .used_cards
            .iter()
            .map(|card| card.polish_pct_raw)
            .fold(0_i64, i64::saturating_add);
        upgrade_bonus.saturating_add(polish_bonus)
    }

    pub fn tower_damage_bonus_raw_for_template(&self, template: &crate::TowerTemplateState) -> i64 {
        let upgrade_bonus = self.tower_upgrade_bonus_raw_for_template(template);
        let polish_bonus = template
            .used_cards
            .iter()
            .map(|card| card.polish_pct_raw)
            .fold(0_i64, i64::saturating_add);
        upgrade_bonus.saturating_add(polish_bonus)
    }

    pub fn tower_upgrade_bonus_raw(&self, tower: &crate::TowerState) -> i64 {
        self.upgrades
            .iter()
            .map(|upgrade| {
                let kind = upgrade
                    .upgrade_kind()
                    .expect("persisted upgrade kind must be valid before bonus calculation");
                (definition(kind).tower_bonus)(upgrade, tower)
            })
            .fold(0_i64, i64::saturating_add)
    }

    pub fn tower_upgrade_bonus_raw_for_template(
        &self,
        template: &crate::TowerTemplateState,
    ) -> i64 {
        self.upgrades
            .iter()
            .map(|upgrade| {
                let kind = upgrade
                    .upgrade_kind()
                    .expect("persisted upgrade kind must be valid before bonus calculation");
                (definition(kind).tower_bonus_for_template)(upgrade, template)
            })
            .fold(0_i64, i64::saturating_add)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct UpgradeCacheContribution {
    pub(crate) max_hp_plus_raw: i64,
    pub(crate) shop_slot_expand: usize,
    pub(crate) dice_chance_plus: usize,
    pub(crate) shop_item_price_minus: usize,
    pub(crate) shorten_straight_flush_to_4_cards: bool,
    pub(crate) skip_rank_for_straight: bool,
    pub(crate) treat_suits_as_same: bool,
    pub(crate) clear_shield_on_stage_start: bool,
}

fn trigger_definition(kind: crate::UpgradeKind) -> definition::UpgradeTriggerDefinition {
    definition(kind).triggers
}

pub fn generated_upgrade(kind: crate::UpgradeKind) -> crate::UpgradeEntryState {
    let mut upgrade = crate::UpgradeEntryState {
        id: 0,
        kind: kind.raw(),
        scalar_values: Vec::new(),
        ratio_values_raw: Vec::new(),
        bool_values: Vec::new(),
        optional_ids: Vec::new(),
    };
    let definition = definition(kind).generate_payload;
    definition(&mut upgrade);
    upgrade
}

/// Generates an upgrade after decoding a persisted or wire raw kind.
pub fn generated_upgrade_raw(raw: u8) -> Result<crate::UpgradeEntryState, crate::CommandError> {
    let kind =
        crate::UpgradeKind::from_raw(raw).ok_or(crate::CommandError::InvalidUpgradeKind { raw })?;
    Ok(generated_upgrade(kind))
}

/// Returns the definition rarity for a validated upgrade kind.
pub fn upgrade_rarity(kind: crate::UpgradeKind) -> crate::Rarity {
    definition(kind).rarity
}

/// Returns the definition rarity after decoding a persisted or wire raw kind.
pub fn upgrade_rarity_raw(raw: u8) -> Result<crate::Rarity, crate::CommandError> {
    let kind =
        crate::UpgradeKind::from_raw(raw).ok_or(crate::CommandError::InvalidUpgradeKind { raw })?;
    Ok(upgrade_rarity(kind))
}

fn current_and_max(core: &crate::CoreState, kind: crate::UpgradeKind) -> Option<(usize, usize)> {
    (definition(kind).current_and_max)(core)
}

pub fn generate_boss_reward_option(core: &mut crate::CoreState) -> crate::UpgradeEntryState {
    let mut rng = core.rng.next_rng(
        crate::deterministic_rng::domain::REWARD_UPGRADE,
        &[core.progress.stage as u64],
    );
    let kinds = crate::UpgradeKind::ALL.to_vec();
    let kind = kinds
        .choose_weighted(&mut rng, |kind| {
            if current_and_max(core, *kind).is_some_and(|(current, max)| current >= max) {
                return 0.0_f32;
            }
            match upgrade_rarity(*kind) {
                crate::Rarity::Common => 5.0_f32,
                crate::Rarity::Rare => 10.0_f32,
                crate::Rarity::Epic => 25.0_f32,
                crate::Rarity::Legendary => 50.0_f32,
            }
        })
        .expect("at least one upgrade reward must be eligible");
    generated_upgrade(*kind)
}

pub(crate) fn generate_boss_reward_options(
    core: &mut crate::CoreState,
) -> Vec<crate::UpgradeEntryState> {
    (0..3).map(|_| generate_boss_reward_option(core)).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpgradeAcquireRecovery {
    None,
    Amount(i64),
    ToFull,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpgradeAcquireOutput {
    pub recovery: UpgradeAcquireRecovery,
    pub additional_shop_slots: usize,
}

impl crate::CoreState {
    pub fn acquire_upgrade(
        &mut self,
        upgrade: UpgradeEntryState,
    ) -> Result<UpgradeAcquireOutput, crate::CommandError> {
        let kind = upgrade.upgrade_kind()?;
        let recovery = upgrade_acquire_recovery(kind);
        let additional_shop_slots = (definition(kind).acquire)(self, upgrade);

        self.refresh_upgrade_damage_multipliers();
        self.bump_upgrade_revision();
        Ok(UpgradeAcquireOutput {
            recovery,
            additional_shop_slots,
        })
    }

    pub fn apply_upgrade_recovery(&mut self, recovery: UpgradeAcquireRecovery) {
        match recovery {
            UpgradeAcquireRecovery::None => {}
            UpgradeAcquireRecovery::Amount(amount) => {
                self.hp_raw = self.hp_raw.saturating_add(amount).min(self.max_hp_raw());
            }
            UpgradeAcquireRecovery::ToFull => {
                self.hp_raw = self.max_hp_raw();
            }
        }
    }

    pub fn trigger_monster_death_upgrades(&mut self) {
        let mut gold = 0_usize;
        let mut healing = 0_i64;
        for index in 0..self.upgrades.upgrades.len() {
            let kind = self.upgrades.upgrades[index]
                .upgrade_kind()
                .expect("stored upgrade kind must be valid");
            (trigger_definition(kind).monster_death)(self, index, &mut gold, &mut healing);
        }
        if gold > 0 {
            self.earn_gold(gold);
        }
        if healing > 0 {
            self.hp_raw = self.hp_raw.saturating_add(healing).min(self.max_hp_raw());
        }
    }

    pub fn trigger_gold_earned_upgrades(&mut self) {
        let mut changed = false;
        for index in 0..self.upgrades.upgrades.len() {
            let kind = self.upgrades.upgrades[index]
                .upgrade_kind()
                .expect("stored upgrade kind must be valid");
            changed |= (trigger_definition(kind).gold_earned)(self, index);
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    pub fn trigger_gold_spent_upgrades(&mut self) {
        self.trigger_gold_earned_upgrades();
    }

    pub fn trigger_card_reroll_upgrades(&mut self) {
        let mut changed = false;
        for index in 0..self.upgrades.upgrades.len() {
            let kind = self.upgrades.upgrades[index]
                .upgrade_kind()
                .expect("stored upgrade kind must be valid");
            changed |= (trigger_definition(kind).card_rerolled)(self, index);
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    pub fn trigger_shop_purchase_upgrades(&mut self, item_purchase: bool) {
        let mut changed = false;
        for index in 0..self.upgrades.upgrades.len() {
            let kind = self.upgrades.upgrades[index]
                .upgrade_kind()
                .expect("stored upgrade kind must be valid");
            changed |= (trigger_definition(kind).shop_purchase)(self, index, item_purchase);
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    pub fn trigger_tower_placed_upgrades(
        &mut self,
        tower_id: u64,
        is_face: bool,
        tower_template: &crate::TowerTemplateState,
    ) {
        let mut changed = false;
        let mut camera_reward: usize = 0;
        for index in 0..self.upgrades.upgrades.len() {
            let kind = self.upgrades.upgrades[index]
                .upgrade_kind()
                .expect("stored upgrade kind must be valid");
            changed |= (trigger_definition(kind).tower_placed)(
                self,
                index,
                tower_id,
                is_face,
                tower_template,
                &mut camera_reward,
            );
        }
        if camera_reward > 0 {
            self.progress.gold = self.progress.gold.saturating_add(camera_reward);
            self.metrics.total_gold_earned =
                self.metrics.total_gold_earned.saturating_add(camera_reward);
            self.trigger_gold_earned_upgrades();
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    pub fn trigger_tower_removed_upgrades(&mut self, rerolled_count: usize) {
        for index in 0..self.upgrades.upgrades.len() {
            let kind = self.upgrades.upgrades[index]
                .upgrade_kind()
                .expect("stored upgrade kind must be valid");
            (trigger_definition(kind).tower_removed)(self, index, rerolled_count);
        }
    }

    pub fn trigger_stage_start_upgrades(&mut self, stage: usize) {
        let mut changed = false;
        for index in 0..self.upgrades.upgrades.len() {
            let kind = self.upgrades.upgrades[index]
                .upgrade_kind()
                .expect("stored upgrade kind must be valid");
            changed |= (trigger_definition(kind).stage_start)(self, index, stage);
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    pub fn trigger_stage_end_upgrades(
        &mut self,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) {
        let mut changed = false;
        let mut index = 0;
        while index < self.upgrades.upgrades.len() {
            let kind = self.upgrades.upgrades[index]
                .upgrade_kind()
                .expect("stored upgrade kind must be valid");
            let (upgrade_changed, gold_reward) =
                (trigger_definition(kind).stage_end)(self, index, perfect_clear, gold, item_count);
            changed |= upgrade_changed;
            if gold_reward > 0 {
                self.progress.gold = self.progress.gold.saturating_add(gold_reward);
                self.metrics.total_gold_earned =
                    self.metrics.total_gold_earned.saturating_add(gold_reward);
                self.trigger_gold_earned_upgrades();
            }
            index += 1;
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    fn bump_upgrade_revision(&mut self) {
        self.upgrades.revision = self.upgrades.revision.wrapping_add(1);
    }

    fn refresh_upgrade_damage_multipliers(&mut self) {
        for tower in &mut self.towers {
            let bonus_raw = self.upgrades.tower_damage_bonus_raw(tower);
            tower.damage_multiplier_raw = crate::RATIO_SCALE.saturating_add(bonus_raw).max(0);
        }
    }

    fn next_upgrade_id(&self) -> u64 {
        self.upgrades
            .upgrades
            .iter()
            .map(|upgrade| upgrade.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1)
    }
}

fn upgrade_acquire_recovery(kind: crate::UpgradeKind) -> UpgradeAcquireRecovery {
    (definition(kind).recovery)()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tower(id: u64, rerolled_count: usize, polish_pct_raw: i64) -> crate::TowerState {
        crate::TowerState {
            id: Some(id),
            left_top: [0, 0],
            cooldown: 0,
            template: crate::TowerTemplateState {
                kind: 1,
                rerolled_count,
                shoot_interval: 60,
                default_attack_range_radius_raw: 1,
                default_damage_raw: 1,
                suit: Some(0),
                rank: Some(11),
                skill_templates: Vec::new(),
                default_status_effects: Vec::new(),
                used_cards: vec![crate::CardState {
                    id: 1,
                    suit: 0,
                    rank: 11,
                    polish_pct_raw,
                    engraving: None,
                }],
            },
            status_effects: Vec::new(),
            skills: Vec::new(),
            damage_multiplier_raw: crate::RATIO_SCALE,
            attack_range_radius_raw: 1,
            effective_shoot_interval: 60,
            on_hit_splashes: Vec::new(),
            on_attack_splashes: Vec::new(),
        }
    }

    #[test]
    fn tower_damage_bonus_sums_upgrade_payloads_and_card_polish_once() {
        let upgrades = UpgradeCollectionState {
            upgrades: vec![
                UpgradeEntryState {
                    id: 1,
                    kind: 7,
                    scalar_values: Vec::new(),
                    ratio_values_raw: vec![125_000],
                    bool_values: Vec::new(),
                    optional_ids: Vec::new(),
                },
                UpgradeEntryState {
                    id: 2,
                    kind: 20,
                    scalar_values: Vec::new(),
                    ratio_values_raw: vec![250_000],
                    bool_values: Vec::new(),
                    optional_ids: vec![Some(9)],
                },
                UpgradeEntryState {
                    id: 3,
                    kind: 22,
                    scalar_values: vec![2],
                    ratio_values_raw: vec![50_000],
                    bool_values: Vec::new(),
                    optional_ids: Vec::new(),
                },
            ],
            revision: 0,
        };
        let tower = tower(9, 0, 75_000);

        assert_eq!(
            upgrades.tower_damage_bonus_raw(&tower),
            125_000 + 250_000 + 100_000 + 75_000
        );
    }

    #[test]
    fn tower_damage_bonus_excludes_no_reroll_bonus_after_reroll() {
        let upgrades = UpgradeCollectionState {
            upgrades: vec![UpgradeEntryState {
                id: 1,
                kind: 7,
                scalar_values: Vec::new(),
                ratio_values_raw: vec![125_000],
                bool_values: Vec::new(),
                optional_ids: Vec::new(),
            }],
            revision: 0,
        };

        assert_eq!(upgrades.tower_damage_bonus_raw(&tower(1, 1, 0)), 0);
    }

    #[test]
    fn template_damage_observation_matches_placed_tower_for_previewable_upgrades() {
        let upgrades = UpgradeCollectionState {
            upgrades: vec![
                UpgradeEntryState {
                    id: 1,
                    kind: 7,
                    scalar_values: Vec::new(),
                    ratio_values_raw: vec![125_000],
                    bool_values: Vec::new(),
                    optional_ids: Vec::new(),
                },
                UpgradeEntryState {
                    id: 2,
                    kind: 22,
                    scalar_values: vec![2],
                    ratio_values_raw: vec![50_000],
                    bool_values: Vec::new(),
                    optional_ids: Vec::new(),
                },
            ],
            revision: 0,
        };
        let placed = tower(9, 0, 75_000);

        assert_eq!(
            upgrades.tower_damage_bonus_raw(&placed),
            upgrades.tower_damage_bonus_raw_for_template(&placed.template)
        );
    }

    #[test]
    fn cache_observation_matches_legacy_upgrade_effect_values() {
        let upgrades = UpgradeCollectionState {
            upgrades: vec![
                generated_upgrade(crate::UpgradeKind::Apple),
                generated_upgrade(crate::UpgradeKind::Backpack),
                generated_upgrade(crate::UpgradeKind::DiceBundle),
                generated_upgrade(crate::UpgradeKind::EnergyDrink),
                generated_upgrade(crate::UpgradeKind::FourLeafClover),
                generated_upgrade(crate::UpgradeKind::Rabbit),
                generated_upgrade(crate::UpgradeKind::BlackWhite),
                generated_upgrade(crate::UpgradeKind::Spanner),
            ],
            revision: 0,
        };
        let cache = upgrades.cache_state();

        assert_eq!(cache.max_hp_plus_raw, 4_000);
        assert_eq!(cache.shop_slot_expand, 1);
        assert_eq!(cache.dice_chance_plus, 1);
        assert_eq!(cache.shop_item_price_minus, 5);
        assert!(cache.shorten_straight_flush_to_4_cards);
        assert!(cache.skip_rank_for_straight);
        assert!(cache.treat_suits_as_same);
        assert!(!cache.clear_shield_on_stage_start);
    }

    fn test_core() -> crate::CoreState {
        let config = crate::GameConfigState {
            player: crate::PlayerConfigState {
                max_hp_raw: 60_000,
                starting_gold: 100,
                starting_hp_raw: 60_000,
                base_dice_chance: 3,
                max_stages: 5,
                base_hand_slots: 5,
            },
            towers: crate::TowerConfigState {
                entries: (0..=10)
                    .map(|kind| crate::TowerConfigEntryState {
                        kind,
                        damage_raw: 1_000,
                        range_raw: 1_000_000,
                        cooldown_ms: 1_000,
                    })
                    .collect(),
            },
            monsters: crate::MonsterConfigState {
                stats: vec![crate::MonsterConfigEntryState {
                    kind: 0,
                    base_hp_raw: 1_000,
                    velocity_mul_raw: crate::RATIO_SCALE,
                    damage_raw: 100,
                    reward: 1,
                }],
                stage_waves: vec![crate::StageWaveState {
                    stage: 1,
                    entries: vec![crate::StageWaveEntryState { kind: 0, count: 1 }],
                }],
            },
        };
        crate::CoreState::new_initial(config, 7)
    }

    fn with_upgrades(core: &mut crate::CoreState, upgrades: Vec<crate::UpgradeEntryState>) {
        core.edit_snapshot(|parts| parts.upgrades.upgrades = upgrades)
            .expect("test upgrade state must be valid");
    }

    #[test]
    fn acquisition_applies_kind_payload_and_recovery() {
        let mut core = test_core();
        core.edit_snapshot(|parts| parts.hp_raw = 10_000)
            .expect("test health must be valid");

        let recovery = core
            .acquire_upgrade(generated_upgrade(crate::UpgradeKind::Apple))
            .expect("apple kind is valid")
            .recovery;
        assert_eq!(recovery, UpgradeAcquireRecovery::Amount(6_000));
        core.apply_upgrade_recovery(recovery);
        assert_eq!(core.hp_raw(), 16_000);

        let output = core
            .acquire_upgrade(generated_upgrade(crate::UpgradeKind::Metronome))
            .expect("metronome kind is valid");
        assert_eq!(output.recovery, UpgradeAcquireRecovery::None);
        assert_eq!(
            core.upgrades()
                .upgrades
                .last()
                .expect("metronome must be acquired")
                .scalar_values,
            vec![core.progress().stage as u64]
        );
    }

    #[test]
    fn monster_death_trigger_updates_gold_metrics_and_health() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![
                crate::UpgradeEntryState {
                    id: 1,
                    kind: 3,
                    scalar_values: vec![7],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeEntryState {
                    id: 2,
                    kind: 31,
                    scalar_values: vec![2],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
            ],
        );
        core.edit_snapshot(|parts| {
            parts.progress.gold = 10;
            parts.hp_raw = 10_000;
        })
        .expect("test trigger state must be valid");

        core.trigger_monster_death_upgrades();

        assert_eq!(core.progress().gold, 17);
        assert_eq!(core.metrics().total_gold_earned, 7);
        assert_eq!(core.hp_raw(), 12_000);
    }

    #[test]
    fn gold_trigger_refreshes_crock_payload() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![crate::UpgradeEntryState {
                id: 1,
                kind: 12,
                scalar_values: vec![0],
                ratio_values_raw: vec![],
                bool_values: vec![],
                optional_ids: vec![],
            }],
        );
        core.edit_snapshot(|parts| parts.progress.gold = 250)
            .expect("test gold state must be valid");

        core.trigger_gold_earned_upgrades();

        assert_eq!(core.upgrades().upgrades[0].scalar_values, vec![2]);
    }

    #[test]
    fn card_reroll_trigger_updates_resolution_and_broken_pottery() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![
                crate::UpgradeEntryState {
                    id: 1,
                    kind: 22,
                    scalar_values: vec![0],
                    ratio_values_raw: vec![250_000],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeEntryState {
                    id: 2,
                    kind: 34,
                    scalar_values: vec![],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
            ],
        );
        core.edit_snapshot(|parts| {
            parts.progress.left_dice = 3;
            parts.progress.rerolled_count = 4;
        })
        .expect("test reroll state must be valid");

        core.trigger_card_reroll_upgrades();

        assert_eq!(core.upgrades().upgrades[0].scalar_values, vec![3]);
        assert_eq!(core.progress().left_dice, 4);
    }

    #[test]
    fn shop_purchase_trigger_awards_item_reroll() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![crate::UpgradeEntryState {
                id: 1,
                kind: 21,
                scalar_values: vec![],
                ratio_values_raw: vec![],
                bool_values: vec![],
                optional_ids: vec![],
            }],
        );
        core.edit_snapshot(|parts| parts.progress.left_dice = 2)
            .expect("test purchase state must be valid");

        core.trigger_shop_purchase_upgrades(true);

        assert_eq!(core.progress().left_dice, 3);
    }

    #[test]
    fn tower_placement_and_removal_triggers_apply_kind_rules() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![
                crate::UpgradeEntryState {
                    id: 1,
                    kind: 20,
                    scalar_values: vec![],
                    ratio_values_raw: vec![2_000_000],
                    bool_values: vec![],
                    optional_ids: vec![None],
                },
                crate::UpgradeEntryState {
                    id: 2,
                    kind: 23,
                    scalar_values: vec![],
                    ratio_values_raw: vec![],
                    bool_values: vec![true],
                    optional_ids: vec![],
                },
                crate::UpgradeEntryState {
                    id: 3,
                    kind: 29,
                    scalar_values: vec![],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeEntryState {
                    id: 4,
                    kind: 17,
                    scalar_values: vec![],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
            ],
        );
        let template = crate::TowerTemplateState {
            kind: 0,
            rerolled_count: 0,
            shoot_interval: 60,
            default_attack_range_radius_raw: 1,
            default_damage_raw: 1,
            suit: Some(0),
            rank: Some(12),
            skill_templates: vec![],
            default_status_effects: vec![],
            used_cards: vec![],
        };

        core.trigger_tower_placed_upgrades(9, true, &template);
        assert_eq!(core.upgrades().upgrades[0].optional_ids, vec![Some(9)]);
        assert!(!core.upgrades().upgrades[1].bool_values[0]);
        assert_eq!(core.progress().gold, 150);
        assert_eq!(core.hand().slots.len(), 6);

        core.edit_snapshot(|parts| parts.progress.left_dice = 0)
            .expect("test removal state must be valid");
        core.trigger_tower_removed_upgrades(3);
        assert_eq!(core.progress().left_dice, 3);
    }

    #[test]
    fn stage_triggers_apply_start_and_end_payloads() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![
                crate::UpgradeEntryState {
                    id: 1,
                    kind: 18,
                    scalar_values: vec![0],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeEntryState {
                    id: 2,
                    kind: 19,
                    scalar_values: vec![0],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeEntryState {
                    id: 3,
                    kind: 11,
                    scalar_values: vec![],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeEntryState {
                    id: 4,
                    kind: 30,
                    scalar_values: vec![5],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
            ],
        );
        core.edit_snapshot(|parts| parts.progress.left_dice = 0)
            .expect("test stage state must be valid");

        core.trigger_stage_start_upgrades(1);
        assert_eq!(core.progress().left_dice, 2);
        core.trigger_stage_start_upgrades(3);
        assert_eq!(
            core.stage_modifiers().enemy_speed_multipliers_raw,
            vec![750_000]
        );

        core.trigger_stage_end_upgrades(true, 0, 2);
        assert_eq!(core.stage_modifiers().free_card_services, 1);
        assert_eq!(core.progress().gold, 110);
    }

    #[test]
    fn generated_upgrade_payloads_cover_every_catalog_kind() {
        let mut seen_rarities = [false; 4];
        assert_eq!(
            definition::UPGRADE_DEFINITIONS.len(),
            crate::UpgradeKind::COUNT
        );
        assert_eq!(
            definition::UPGRADE_DEFINITIONS.len(),
            crate::UpgradeKind::ALL.len()
        );
        for (index, &kind) in crate::UpgradeKind::ALL.iter().enumerate() {
            let definition = &definition::UPGRADE_DEFINITIONS[index];
            assert_eq!(definition.kind, kind);
            assert_eq!(super::definition(kind).kind, kind);
            let upgrade = generated_upgrade(kind);
            assert_eq!(upgrade.kind, kind.raw());
            assert!(upgrade_rarity(kind).index() < crate::Rarity::ALL.len());
            seen_rarities[upgrade_rarity(kind).index()] = true;
        }
        assert_eq!(seen_rarities, [true, true, true, true]);
    }

    #[test]
    fn raw_upgrade_boundaries_round_trip_and_reject_unknown_kinds() {
        for &kind in crate::UpgradeKind::ALL {
            let generated = generated_upgrade_raw(kind.raw()).expect("catalog kind is valid");
            assert_eq!(generated.upgrade_kind_raw(), kind.raw());
            assert_eq!(
                generated.upgrade_kind().expect("catalog kind is valid"),
                kind
            );
            assert_eq!(
                generated_upgrade_raw(kind.raw()).expect("catalog kind is valid"),
                generated
            );
            assert_eq!(
                upgrade_rarity_raw(kind.raw()).expect("catalog kind is valid"),
                upgrade_rarity(kind)
            );
        }
        assert_eq!(
            generated_upgrade_raw(u8::MAX),
            Err(crate::CommandError::InvalidUpgradeKind { raw: u8::MAX })
        );
        assert_eq!(
            upgrade_rarity_raw(u8::MAX),
            Err(crate::CommandError::InvalidUpgradeKind { raw: u8::MAX })
        );
        let invalid = crate::UpgradeEntryState {
            id: 0,
            kind: u8::MAX,
            scalar_values: Vec::new(),
            ratio_values_raw: Vec::new(),
            bool_values: Vec::new(),
            optional_ids: Vec::new(),
        };
        let mut core = test_core();
        assert_eq!(
            core.acquire_upgrade(invalid),
            Err(crate::CommandError::InvalidUpgradeKind { raw: u8::MAX })
        );
    }

    #[test]
    fn generated_upgrade_serde_preserves_every_canonical_raw_kind() {
        for &kind in crate::UpgradeKind::ALL {
            let generated = generated_upgrade(kind);
            let encoded = serde_json::to_string(&generated).expect("upgrade serializes");
            let decoded: UpgradeEntryState =
                serde_json::from_str(&encoded).expect("upgrade deserializes");
            assert_eq!(decoded, generated);
            assert_eq!(decoded.upgrade_kind(), Ok(kind));
        }
    }

    #[test]
    fn acquisition_and_trigger_families_are_deterministic_across_core_states() {
        let mut first = test_core();
        let mut second = test_core();
        for &kind in crate::UpgradeKind::ALL {
            let first_output = first.acquire_upgrade(generated_upgrade(kind));
            let second_output = second.acquire_upgrade(generated_upgrade(kind));
            assert_eq!(first_output, second_output, "acquisition kind {kind:?}");
        }

        first
            .edit_snapshot(|parts| {
                parts.progress.gold = 250;
                parts.progress.left_dice = 4;
                parts.progress.rerolled_count = 4;
            })
            .expect("first trigger fixture");
        second
            .edit_snapshot(|parts| {
                parts.progress.gold = 250;
                parts.progress.left_dice = 4;
                parts.progress.rerolled_count = 4;
            })
            .expect("second trigger fixture");

        for state in [&mut first, &mut second] {
            state.trigger_monster_death_upgrades();
            state.trigger_gold_earned_upgrades();
            state.trigger_gold_spent_upgrades();
            state.trigger_card_reroll_upgrades();
            state.trigger_shop_purchase_upgrades(true);
            state.trigger_stage_start_upgrades(2);
            state.trigger_stage_end_upgrades(true, 1, 1);
        }

        assert_eq!(first, second);
        assert_eq!(
            crate::authoritative_hash(&first),
            crate::authoritative_hash(&second)
        );
    }
}
