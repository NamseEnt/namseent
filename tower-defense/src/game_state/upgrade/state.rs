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
}

impl UpgradeState {
    pub fn upgrade(&mut self, upgrade: Upgrade) {
        self.revision = self.revision.wrapping_add(1);
        self.upgrades.push(upgrade);
    }

    pub fn stage_start_effects(&mut self, stage: usize) -> StageStartEffects {
        let mut effects = StageStartEffects::new();
        for upgrade in &mut self.upgrades {
            upgrade.apply_on_stage_start(stage, &mut effects);
        }
        effects
    }

    pub fn apply_pending_placement_bonuses(
        &mut self,
        tower_template: &mut TowerTemplate,
        left_dice: usize,
    ) {
        for upgrade in &mut self.upgrades {
            upgrade.apply_pending_placement_bonuses(tower_template, left_dice);
        }
    }

    pub fn consume_pending_mirror_count(&mut self) -> usize {
        self.upgrades
            .iter_mut()
            .map(|upgrade| upgrade.consume_pending_mirror_count())
            .sum()
    }

    pub fn clear_shield_on_stage_start(&self) -> bool {
        self.upgrades
            .iter()
            .all(|upgrade| upgrade.clear_shield_on_stage_start())
    }

    pub fn on_monster_death(&mut self) -> bool {
        self.upgrades
            .iter_mut()
            .any(|upgrade| upgrade.on_monster_death())
    }

    pub fn on_stage_end(&mut self, gold: usize, item_count: usize) -> usize {
        self.upgrades
            .iter_mut()
            .map(|upgrade| upgrade.on_stage_end(gold, item_count))
            .sum()
    }

    pub fn pea_max_hp_plus(&self) -> usize {
        self.upgrades
            .iter()
            .map(|upgrade| upgrade.max_hp_plus() as usize)
            .sum()
    }

    pub fn gold_earn_plus(&self) -> usize {
        self.upgrades
            .iter()
            .map(|upgrade| upgrade.gold_earn_plus())
            .sum()
    }

    pub fn shop_slot_expand(&self) -> usize {
        self.upgrades
            .iter()
            .map(|upgrade| upgrade.shop_slot_expand())
            .sum()
    }

    pub fn dice_chance_plus(&self) -> usize {
        self.upgrades
            .iter()
            .map(|upgrade| upgrade.dice_chance_plus())
            .sum()
    }

    pub fn shop_item_price_minus(&self) -> usize {
        self.upgrades
            .iter()
            .map(|upgrade| upgrade.shop_item_price_minus())
            .sum()
    }

    pub fn shorten_straight_flush_to_4_cards(&self) -> bool {
        self.upgrades
            .iter()
            .any(|upgrade| upgrade.shorten_straight_flush_to_4_cards())
    }

    pub fn skip_rank_for_straight(&self) -> bool {
        self.upgrades
            .iter()
            .any(|upgrade| upgrade.skip_rank_for_straight())
    }

    pub fn treat_suits_as_same(&self) -> bool {
        self.upgrades
            .iter()
            .any(|upgrade| upgrade.treat_suits_as_same())
    }

    pub fn removed_number_rank_count(&self) -> usize {
        self.upgrades
            .iter()
            .map(|upgrade| upgrade.removed_number_rank_count())
            .sum()
    }

    pub fn record_perfect_clear(&mut self) {
        for upgrade in &mut self.upgrades {
            upgrade.record_perfect_clear();
        }
    }

    pub fn record_tower_removed(&mut self) {
        for upgrade in &mut self.upgrades {
            upgrade.record_tower_removed();
        }
    }

    pub fn on_tower_placed(&mut self, tower: &Tower) -> TowerPlacementResult {
        self.upgrades
            .iter_mut()
            .map(|upgrade| upgrade.on_tower_placed(tower))
            .fold(TowerPlacementResult::default(), |mut acc, result| {
                acc += result;
                acc
            })
    }

    pub fn global_tower_damage_multiplier(&self, game_state: &GameState) -> f32 {
        1.0 + self
            .upgrades
            .iter()
            .filter_map(|upgrade| upgrade.get_global_damage_multiplier(game_state))
            .sum::<f32>()
    }

    pub fn tower_upgrade_state(&self, target: TowerUpgradeTarget) -> TowerUpgradeState {
        let mut state = TowerUpgradeState::default();
        for upgrade in &self.upgrades {
            let multiplier = match (upgrade, &target) {
                (Upgrade::Staff(upgrade), TowerUpgradeTarget::Suit { suit })
                    if *suit == crate::card::Suit::Diamonds =>
                {
                    Some(upgrade.damage_multiplier)
                }
                (Upgrade::LongSword(upgrade), TowerUpgradeTarget::Suit { suit })
                    if *suit == crate::card::Suit::Spades =>
                {
                    Some(upgrade.damage_multiplier)
                }
                (Upgrade::Mace(upgrade), TowerUpgradeTarget::Suit { suit })
                    if *suit == crate::card::Suit::Hearts =>
                {
                    Some(upgrade.damage_multiplier)
                }
                (Upgrade::ClubSword(upgrade), TowerUpgradeTarget::Suit { suit })
                    if *suit == crate::card::Suit::Clubs =>
                {
                    Some(upgrade.damage_multiplier)
                }
                (Upgrade::SingleChopstick(upgrade), TowerUpgradeTarget::EvenOdd { even })
                    if !*even =>
                {
                    Some(upgrade.damage_multiplier)
                }
                (Upgrade::PairChopsticks(upgrade), TowerUpgradeTarget::EvenOdd { even })
                    if *even =>
                {
                    Some(upgrade.damage_multiplier)
                }
                (Upgrade::FountainPen(upgrade), TowerUpgradeTarget::FaceNumber { face })
                    if !*face =>
                {
                    Some(upgrade.damage_multiplier)
                }
                (Upgrade::Brush(upgrade), TowerUpgradeTarget::FaceNumber { face }) if *face => {
                    Some(upgrade.damage_multiplier)
                }
                _ => None,
            };
            if let Some(multiplier) = multiplier {
                state.apply_upgrade(TowerUpgrade::DamageMultiplier { multiplier });
            }
        }
        state
    }

    pub fn tower_select_upgrade_state(
        &self,
        target: TowerSelectUpgradeTarget,
    ) -> TowerUpgradeState {
        let mut state = TowerUpgradeState::default();
        for upgrade in &self.upgrades {
            let multiplier = match (upgrade, target) {
                (Upgrade::Tricycle(upgrade), TowerSelectUpgradeTarget::LowCard) => {
                    Some(upgrade.damage_multiplier)
                }
                (Upgrade::PerfectPottery(upgrade), TowerSelectUpgradeTarget::NoReroll) => {
                    Some(upgrade.damage_multiplier)
                }
                (Upgrade::BrokenPottery(upgrade), TowerSelectUpgradeTarget::Reroll) => {
                    Some(upgrade.damage_multiplier)
                }
                _ => None,
            };
            if let Some(multiplier) = multiplier {
                state.apply_upgrade(TowerUpgrade::DamageMultiplier { multiplier });
            }
        }
        state
    }

    pub fn tower_upgrades(&self, tower: &Tower) -> Vec<TowerUpgradeState> {
        vec![
            self.tower_upgrade_state(TowerUpgradeTarget::Suit { suit: tower.suit() }),
            self.tower_upgrade_state(TowerUpgradeTarget::EvenOdd {
                even: tower.rank().is_even(),
            }),
            self.tower_upgrade_state(TowerUpgradeTarget::FaceNumber {
                face: tower.rank().is_face(),
            }),
        ]
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord, State)]
pub enum TowerUpgradeTarget {
    Suit { suit: Suit },
    EvenOdd { even: bool },
    FaceNumber { face: bool },
}
#[derive(Debug, Clone, Copy, State)]
pub enum TowerUpgrade {
    DamageMultiplier { multiplier: f32 },
}
#[derive(Debug, Clone, Copy, State)]
pub struct TowerUpgradeState {
    pub damage_multiplier: f32,
}
impl TowerUpgradeState {
    fn apply_upgrade(&mut self, upgrade: TowerUpgrade) {
        match upgrade {
            TowerUpgrade::DamageMultiplier { multiplier } => self.damage_multiplier *= multiplier,
        }
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
