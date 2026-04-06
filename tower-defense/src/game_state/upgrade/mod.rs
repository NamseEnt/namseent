mod data_conversion;
mod display;
mod generation;
mod thumbnail;

use crate::{card::Suit, game_state::tower::Tower, *};
pub use data_conversion::{UpgradeInfo, UpgradeInfoDescription, get_upgrade_infos};
pub use generation::*;
use std::collections::BTreeMap;

pub const MAX_GOLD_EARN_PLUS: usize = 16;
pub const MAX_SHOP_SLOT_EXPAND: usize = 2;
pub const MAX_DICE_CHANCE_PLUS: usize = 4;
pub const MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE: usize = 15;
pub const MAX_REMOVE_NUMBER_RANKS: usize = 5;
pub const DEFAULT_MAX_TREASURE_TOKENS: u8 = 2;

#[derive(Debug, Clone, State)]
pub struct UpgradeState {
    pub gold_earn_plus: usize,
    pub shop_slot_expand: usize,
    pub dice_chance_plus: usize,
    pub max_treasure_tokens: u8,
    pub tower_upgrade_states: BTreeMap<TowerUpgradeTarget, TowerUpgradeState>,
    pub tower_select_upgrade_states: BTreeMap<TowerSelectUpgradeTarget, TowerUpgradeState>,
    pub shop_item_price_minus: usize,
    pub removed_number_rank_count: usize,
    pub shorten_straight_flush_to_4_cards: bool,
    pub skip_rank_for_straight: bool,
    pub treat_suits_as_same: bool,
}

impl Default for UpgradeState {
    fn default() -> Self {
        Self {
            gold_earn_plus: 0,
            shop_slot_expand: 0,
            dice_chance_plus: 0,
            max_treasure_tokens: DEFAULT_MAX_TREASURE_TOKENS,
            tower_upgrade_states: BTreeMap::new(),
            tower_select_upgrade_states: BTreeMap::new(),
            shop_item_price_minus: 0,
            removed_number_rank_count: 0,
            shorten_straight_flush_to_4_cards: false,
            skip_rank_for_straight: false,
            treat_suits_as_same: false,
        }
    }
}

#[derive(Debug, Clone, Copy, State)]
pub struct Upgrade {
    pub kind: UpgradeKind,
    pub value: crate::OneZero,
}

impl UpgradeState {
    pub fn upgrade(&mut self, upgrade: Upgrade) {
        match upgrade.kind {
            UpgradeKind::Magnet => match self.gold_earn_plus {
                0 => self.gold_earn_plus = 1,
                1 => self.gold_earn_plus = 2,
                2 => self.gold_earn_plus = 4,
                4 => self.gold_earn_plus = 8,
                8 => self.gold_earn_plus = 16,
                _ => unreachable!("Invalid magnet upgrade: {}", self.gold_earn_plus),
            },
            UpgradeKind::CainSword { damage_multiplier } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Suit {
                        suit: Suit::Diamonds,
                    },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::LongSword { damage_multiplier } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Suit { suit: Suit::Spades },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::Mace { damage_multiplier } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Suit { suit: Suit::Hearts },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::ClubSword { damage_multiplier } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Suit { suit: Suit::Clubs },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::Backpack => match self.shop_slot_expand {
                0 => self.shop_slot_expand = 1,
                1 => self.shop_slot_expand = 2,
                _ => unreachable!("Invalid backpack upgrade: {}", self.shop_slot_expand),
            },
            UpgradeKind::DiceBundle => match self.dice_chance_plus {
                0 => self.dice_chance_plus = 1,
                1 => self.dice_chance_plus = 2,
                2 => self.dice_chance_plus = 3,
                3 => self.dice_chance_plus = 4,
                _ => unreachable!("Invalid dice bundle upgrade: {}", self.dice_chance_plus),
            },
            UpgradeKind::Spoon { damage_multiplier } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::LowCard,
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::EnergyDrink => match self.shop_item_price_minus {
                0 => self.shop_item_price_minus = 5,
                5 => self.shop_item_price_minus = 10,
                10 => self.shop_item_price_minus = 15,
                _ => unreachable!(
                    "Invalid energy drink upgrade: {}",
                    self.shop_item_price_minus
                ),
            },
            UpgradeKind::PerfectPottery { damage_multiplier } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::NoReroll,
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::SingleChopstick { damage_multiplier } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::EvenOdd { even: false },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::PairChopsticks { damage_multiplier } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::EvenOdd { even: true },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::FountainPen { damage_multiplier } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::FaceNumber { face: false },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::Brush { damage_multiplier } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::FaceNumber { face: true },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::FourLeafClover => {
                self.shorten_straight_flush_to_4_cards = true;
            }
            UpgradeKind::Rabbit => {
                self.skip_rank_for_straight = true;
            }
            UpgradeKind::BlackWhite => {
                self.treat_suits_as_same = true;
            }
            UpgradeKind::Eraser => {
                self.removed_number_rank_count =
                    (self.removed_number_rank_count + 1).min(MAX_REMOVE_NUMBER_RANKS);
            }
            UpgradeKind::BrokenPottery { damage_multiplier } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::Reroll,
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
        }
    }
    pub fn tower_upgrades(&self, tower: &Tower) -> Vec<TowerUpgradeState> {
        [
            TowerUpgradeTarget::Suit { suit: tower.suit },
            TowerUpgradeTarget::EvenOdd {
                even: tower.rank.is_even(),
            },
            TowerUpgradeTarget::FaceNumber {
                face: tower.rank.is_face(),
            },
        ]
        .iter()
        .map(|target| {
            self.tower_upgrade_states
                .get(target)
                .copied()
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
    }
    fn apply_tower_upgrade(&mut self, target: TowerUpgradeTarget, upgrade: TowerUpgrade) {
        self.tower_upgrade_states
            .entry(target)
            .or_default()
            .apply_upgrade(upgrade);
    }
    fn apply_tower_select_upgrade(
        &mut self,
        target: TowerSelectUpgradeTarget,
        upgrade: TowerUpgrade,
    ) {
        self.tower_select_upgrade_states
            .entry(target)
            .or_default()
            .apply_upgrade(upgrade);
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub enum UpgradeKind {
    Magnet,
    CainSword { damage_multiplier: f32 },
    LongSword { damage_multiplier: f32 },
    Mace { damage_multiplier: f32 },
    ClubSword { damage_multiplier: f32 },
    Backpack,
    DiceBundle,
    Spoon { damage_multiplier: f32 },
    EnergyDrink,
    PerfectPottery { damage_multiplier: f32 },
    SingleChopstick { damage_multiplier: f32 },
    PairChopsticks { damage_multiplier: f32 },
    FountainPen { damage_multiplier: f32 },
    Brush { damage_multiplier: f32 },
    FourLeafClover,
    Rabbit,
    BlackWhite,
    Eraser,
    BrokenPottery { damage_multiplier: f32 },
}

impl UpgradeKind {
    pub fn is_tower_damage_upgrade(&self) -> bool {
        matches!(
            self,
            UpgradeKind::CainSword { .. }
                | UpgradeKind::LongSword { .. }
                | UpgradeKind::Mace { .. }
                | UpgradeKind::ClubSword { .. }
                | UpgradeKind::Spoon { .. }
                | UpgradeKind::PerfectPottery { .. }
                | UpgradeKind::SingleChopstick { .. }
                | UpgradeKind::PairChopsticks { .. }
                | UpgradeKind::FountainPen { .. }
                | UpgradeKind::Brush { .. }
                | UpgradeKind::BrokenPottery { .. }
        )
    }

    pub fn is_treasure_upgrade(&self) -> bool {
        !self.is_tower_damage_upgrade()
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

/// Equal to or less than the number of cards in the hand.
pub const LOW_CARD_COUNT: usize = 3;
