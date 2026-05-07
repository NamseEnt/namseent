use super::*;
use crate::*;
use crate::{
    card::Suit,
    game_state::{
        GameState,
        tower::{Tower, TowerTemplate},
    },
};

#[derive(Debug, Clone, State, Default)]
pub struct UpgradeState {
    pub upgrades: Vec<Upgrade>,
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

pub(crate) enum UpgradeTriggerEvent<'a> {
    UpgradeAcquired {
        upgrade: Upgrade,
    },
    StageStart {
        stage: usize,
    },
    TowerPlaced {
        tower: &'a Tower,
    },
    TowerPlacement {
        tower_template: &'a mut TowerTemplate,
        left_dice: usize,
    },
    TowerRemoved,
    ItemBought,
    StageEnd {
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    },
}

pub(crate) enum UpgradeTriggerResult {
    Flags(UpgradeUpdateFlags),
    StageStart(StageStartEffects, UpgradeUpdateFlags),
    TowerPlaced(TowerPlacementResult, UpgradeUpdateFlags),
    TowerPlacement(usize),
    StageEnd(usize, UpgradeUpdateFlags),
}

impl UpgradeState {
    pub fn cache(&self) -> &UpgradeCache {
        &self.cache
    }

    pub fn with_upgrades(upgrades: Vec<Upgrade>) -> Self {
        let mut state = UpgradeState {
            upgrades,
            ..Default::default()
        };
        state.cache = UpgradeCache::from_state(&state);
        state
    }

    pub fn upgrade(&mut self, upgrade: Upgrade) {
        self.revision = self.revision.wrapping_add(1);
        self.upgrades.push(upgrade);
        self.cache = UpgradeCache::from_state(self);
    }

    pub(crate) fn handle_upgrade_trigger<'a>(
        &mut self,
        game_state: &GameState,
        event: UpgradeTriggerEvent<'a>,
    ) -> UpgradeTriggerResult {
        match event {
            UpgradeTriggerEvent::UpgradeAcquired { .. } => {
                unreachable!("UpgradeAcquired is handled by GameState directly",)
            }
            UpgradeTriggerEvent::StageStart { stage } => {
                let (effects, flags) = self.stage_start_effects(stage);
                UpgradeTriggerResult::StageStart(effects, flags)
            }
            UpgradeTriggerEvent::TowerPlaced { tower } => {
                let (placement_result, flags) = self.on_tower_placed(tower);
                UpgradeTriggerResult::TowerPlaced(placement_result, flags)
            }
            UpgradeTriggerEvent::TowerPlacement {
                tower_template,
                left_dice,
            } => {
                let gold = self.on_tower_placement(tower_template, left_dice);
                UpgradeTriggerResult::TowerPlacement(gold)
            }
            UpgradeTriggerEvent::TowerRemoved => {
                UpgradeTriggerResult::Flags(self.on_tower_removed())
            }
            UpgradeTriggerEvent::ItemBought => UpgradeTriggerResult::Flags(self.on_item_bought()),
            UpgradeTriggerEvent::StageEnd {
                perfect_clear,
                gold,
                item_count,
            } => {
                let (bonus_gold, flags) =
                    self.on_stage_end(game_state, perfect_clear, gold, item_count);
                UpgradeTriggerResult::StageEnd(bonus_gold, flags)
            }
        }
    }

    pub(crate) fn stage_start_effects(
        &mut self,
        stage: usize,
    ) -> (StageStartEffects, UpgradeUpdateFlags) {
        let mut effects = StageStartEffects::new();
        let mut flags = UpgradeUpdateFlags::NONE;
        for upgrade in &mut self.upgrades {
            flags |= upgrade.on_stage_start(stage, &mut effects);
        }
        (effects, flags)
    }

    pub(crate) fn on_item_bought(&mut self) -> UpgradeUpdateFlags {
        self.upgrades
            .iter_mut()
            .fold(UpgradeUpdateFlags::NONE, |flags, upgrade| {
                flags | upgrade.on_item_bought()
            })
    }

    pub fn on_tower_placement(
        &mut self,
        tower_template: &mut TowerTemplate,
        left_dice: usize,
    ) -> usize {
        self.upgrades
            .iter_mut()
            .map(|upgrade| upgrade.on_tower_placement(tower_template, left_dice))
            .sum()
    }

    pub fn clear_shield_on_stage_start(&self) -> bool {
        self.cache().clear_shield_on_stage_start
    }

    pub fn on_monster_death(&mut self) -> bool {
        self.upgrades
            .iter_mut()
            .any(|upgrade| upgrade.on_monster_death())
    }

    pub(crate) fn on_stage_end(
        &mut self,
        game_state: &GameState,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        let mut flags = UpgradeUpdateFlags::NONE;
        let result: usize = self
            .upgrades
            .iter_mut()
            .map(|upgrade| {
                let (upgrade_result, upgrade_flags) =
                    upgrade.on_stage_end_with_state(game_state, perfect_clear, gold, item_count);
                flags |= upgrade_flags;
                upgrade_result
            })
            .sum();
        (result, flags)
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

    pub(crate) fn on_tower_removed(&mut self) -> UpgradeUpdateFlags {
        let mut flags = UpgradeUpdateFlags::NONE;
        for upgrade in &mut self.upgrades {
            flags |= upgrade.on_tower_removed();
        }
        flags
    }

    pub(crate) fn on_tower_placed(
        &mut self,
        tower: &Tower,
    ) -> (TowerPlacementResult, UpgradeUpdateFlags) {
        let mut flags = UpgradeUpdateFlags::NONE;
        let result = self
            .upgrades
            .iter_mut()
            .map(|upgrade| {
                let (upgrade_result, upgrade_flags) = upgrade.on_tower_placed(tower);
                flags |= upgrade_flags;
                upgrade_result
            })
            .fold(TowerPlacementResult::default(), |mut acc, result| {
                acc += result;
                acc
            });
        (result, flags)
    }

    pub(crate) fn on_tower_placed_mut(
        &mut self,
        game_state: &mut GameState,
        tower: &Tower,
    ) -> UpgradeUpdateFlags {
        self.upgrades
            .iter_mut()
            .fold(UpgradeUpdateFlags::NONE, |flags, upgrade| {
                flags | upgrade.on_tower_placed_mut(game_state, tower)
            })
    }

    pub fn tower_upgrade_damage_bonuses(
        &self,
        game_state: &GameState,
    ) -> Vec<TowerUpgradeDamageBonus> {
        self.upgrades
            .iter()
            .filter_map(|upgrade| upgrade.tower_upgrade_damage_bonus(game_state))
            .map(|(target, bonus_pct)| TowerUpgradeDamageBonus { target, bonus_pct })
            .collect()
    }

    pub fn tower_upgrade_state(
        &self,
        target: TowerUpgradeTarget,
        game_state: &GameState,
    ) -> TowerUpgradeState {
        let bonus_sum: f32 = self
            .upgrades
            .iter()
            .filter_map(|upgrade| upgrade.tower_upgrade_damage_bonus(game_state))
            .filter(|(bonus_target, _)| *bonus_target == target)
            .map(|(_, bonus_pct)| bonus_pct)
            .sum();

        TowerUpgradeState {
            damage_multiplier: 1.0 + bonus_sum,
        }
    }

    pub fn tower_select_upgrade_state(
        &self,
        target: TowerSelectUpgradeTarget,
    ) -> TowerUpgradeState {
        let mut state = TowerUpgradeState::default();
        for upgrade in &self.upgrades {
            let multiplier = match (upgrade, target) {
                (Upgrade::Tricycle(upgrade), TowerSelectUpgradeTarget::LowCard) => {
                    Some(1.0 + upgrade.damage_bonus_pct)
                }
                (Upgrade::PerfectPottery(upgrade), TowerSelectUpgradeTarget::NoReroll) => {
                    Some(1.0 + upgrade.damage_bonus_pct)
                }
                (Upgrade::BrokenPottery(upgrade), TowerSelectUpgradeTarget::Reroll) => {
                    Some(1.0 + upgrade.damage_bonus_pct)
                }
                _ => None,
            };
            if let Some(multiplier) = multiplier {
                state.apply_upgrade(TowerUpgrade::DamageMultiplier { multiplier });
            }
        }
        state
    }

    pub fn tower_upgrades(&self, tower: &Tower, game_state: &GameState) -> Vec<TowerUpgradeState> {
        vec![
            self.tower_upgrade_state(TowerUpgradeTarget::Suit { suit: tower.suit() }, game_state),
            self.tower_upgrade_state(
                TowerUpgradeTarget::EvenOdd {
                    even: tower.rank().is_even(),
                },
                game_state,
            ),
            self.tower_upgrade_state(
                TowerUpgradeTarget::FaceNumber {
                    face: tower.rank().is_face(),
                },
                game_state,
            ),
            self.tower_upgrade_state(TowerUpgradeTarget::Global, game_state),
        ]
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord, State)]
pub enum TowerUpgradeTarget {
    Global,
    Suit { suit: Suit },
    EvenOdd { even: bool },
    FaceNumber { face: bool },
    TowerId { tower_id: usize },
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TowerUpgradeDamageBonus {
    pub target: TowerUpgradeTarget,
    pub bonus_pct: f32,
}

impl TowerUpgradeDamageBonus {
    pub fn applies_to_tower(&self, tower: &Tower) -> bool {
        self.target.applies_to_tower(tower)
    }
}

impl TowerUpgradeTarget {
    pub fn applies_to_tower(&self, tower: &Tower) -> bool {
        match self {
            TowerUpgradeTarget::Global => true,
            TowerUpgradeTarget::Suit { suit } => *suit == tower.suit(),
            TowerUpgradeTarget::EvenOdd { even } => *even == tower.rank().is_even(),
            TowerUpgradeTarget::FaceNumber { face } => *face == tower.rank().is_face(),
            TowerUpgradeTarget::TowerId { tower_id } => *tower_id == tower.id(),
        }
    }
}

#[derive(Debug, Clone, Copy, State)]
pub enum TowerUpgrade {
    DamageMultiplier { multiplier: f32 },
}
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TowerUpgradeState {
    pub damage_multiplier: f32,
}
impl TowerUpgradeState {
    fn apply_upgrade(&mut self, upgrade: TowerUpgrade) {
        match upgrade {
            TowerUpgrade::DamageMultiplier { multiplier } => self.damage_multiplier *= multiplier,
        }
    }

    pub fn merge(&mut self, other: TowerUpgradeState) {
        self.damage_multiplier *= other.damage_multiplier;
    }
}
impl Default for TowerUpgradeState {
    fn default() -> Self {
        TowerUpgradeState {
            damage_multiplier: 1.0,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord, State)]
pub enum TowerSelectUpgradeTarget {
    LowCard,
    NoReroll,
    Reroll,
}
