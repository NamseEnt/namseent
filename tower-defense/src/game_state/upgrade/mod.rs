mod display;
mod generation;
mod thumbnail;

use super::tower::TowerKind;
use crate::{
    card::{Rank, Suit},
    game_state::tower::Tower,
    rarity::Rarity,
    *,
};
pub use generation::*;
use std::collections::BTreeMap;

pub const MAX_GOLD_EARN_PLUS: usize = 16;
pub const MAX_SHOP_SLOT_EXPAND: usize = 2;
pub const MAX_SHOP_REFRESH_CHANCE_PLUS: usize = 2;
pub const MAX_REROLL_CHANCE_PLUS: usize = 2;
pub const MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE: usize = 15;

#[derive(Debug, Clone, Default, State)]
pub struct UpgradeState {
    pub gold_earn_plus: usize,
    pub shop_slot_expand: usize,
    pub shop_refresh_chance_plus: usize,
    pub reroll_chance_plus: usize,
    pub tower_upgrade_states: BTreeMap<TowerUpgradeTarget, TowerUpgradeState>,
    pub tower_select_upgrade_states: BTreeMap<TowerSelectUpgradeTarget, TowerUpgradeState>,
    pub shop_item_price_minus: usize,
    pub shorten_straight_flush_to_4_cards: bool,
    pub skip_rank_for_straight: bool,
    pub treat_suits_as_same: bool,
}

#[derive(Debug, Clone, Copy, State)]
pub struct Upgrade {
    pub kind: UpgradeKind,
    pub rarity: Rarity,
    pub value: crate::OneZero,
}

impl UpgradeState {
    pub fn upgrade(&mut self, upgrade: Upgrade) {
        match upgrade.kind {
            UpgradeKind::GoldEarnPlus => match self.gold_earn_plus {
                0 => self.gold_earn_plus = 1,
                1 => self.gold_earn_plus = 2,
                2 => self.gold_earn_plus = 4,
                4 => self.gold_earn_plus = 8,
                8 => self.gold_earn_plus = 16,
                _ => unreachable!("Invalid gold earn plus upgrade: {}", self.gold_earn_plus),
            },
            UpgradeKind::RankAttackDamageMultiply {
                rank,
                damage_multiplier,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Rank { rank },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::RankAttackSpeedMultiply {
                rank,
                speed_multiplier,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Rank { rank },
                    TowerUpgrade::SpeedMultiplier {
                        multiplier: speed_multiplier,
                    },
                );
            }
            UpgradeKind::RankAttackRangePlus { rank, range_plus } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Rank { rank },
                    TowerUpgrade::RangePlus { range: range_plus },
                );
            }
            UpgradeKind::SuitAttackDamageMultiply {
                suit,
                damage_multiplier,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Suit { suit },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::SuitAttackSpeedMultiply {
                suit,
                speed_multiplier,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Suit { suit },
                    TowerUpgrade::SpeedMultiplier {
                        multiplier: speed_multiplier,
                    },
                );
            }
            UpgradeKind::SuitAttackRangePlus { suit, range_plus } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Suit { suit },
                    TowerUpgrade::RangePlus { range: range_plus },
                );
            }
            UpgradeKind::HandAttackDamageMultiply {
                tower_kind,
                damage_multiplier,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::TowerKind { tower_kind },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::HandAttackSpeedMultiply {
                tower_kind,
                speed_multiplier,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::TowerKind { tower_kind },
                    TowerUpgrade::SpeedMultiplier {
                        multiplier: speed_multiplier,
                    },
                );
            }
            UpgradeKind::HandAttackRangePlus {
                tower_kind,
                range_plus,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::TowerKind { tower_kind },
                    TowerUpgrade::RangePlus { range: range_plus },
                );
            }
            UpgradeKind::ShopSlotExpansion => match self.shop_slot_expand {
                0 => self.shop_slot_expand = 1,
                1 => self.shop_slot_expand = 2,
                _ => unreachable!("Invalid shop slot upgrade: {}", self.shop_slot_expand),
            },
            UpgradeKind::RerollCountPlus => match self.reroll_chance_plus {
                0 => self.reroll_chance_plus = 1,
                1 => self.reroll_chance_plus = 2,
                _ => unreachable!("Invalid reroll upgrade: {}", self.reroll_chance_plus),
            },
            UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::LowCard,
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::LowCardTowerAttackSpeedMultiply { speed_multiplier } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::LowCard,
                    TowerUpgrade::SpeedMultiplier {
                        multiplier: speed_multiplier,
                    },
                );
            }
            UpgradeKind::LowCardTowerAttackRangePlus { range_plus } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::LowCard,
                    TowerUpgrade::RangePlus { range: range_plus },
                );
            }
            UpgradeKind::ShopItemPriceMinus => match self.shop_item_price_minus {
                0 => self.shop_item_price_minus = 5,
                5 => self.shop_item_price_minus = 10,
                10 => self.shop_item_price_minus = 15,
                _ => unreachable!(
                    "Invalid shop item price minus upgrade: {}",
                    self.shop_item_price_minus
                ),
            },
            UpgradeKind::ShopRefreshPlus => match self.shop_refresh_chance_plus {
                0 => self.shop_refresh_chance_plus = 1,
                1 => self.shop_refresh_chance_plus = 2,
                2 => self.shop_refresh_chance_plus = 3,
                _ => unreachable!(
                    "Invalid shop refresh upgrade: {}",
                    self.shop_refresh_chance_plus
                ),
            },
            UpgradeKind::NoRerollTowerAttackDamageMultiply { damage_multiplier } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::NoReroll,
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::NoRerollTowerAttackSpeedMultiply { speed_multiplier } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::NoReroll,
                    TowerUpgrade::SpeedMultiplier {
                        multiplier: speed_multiplier,
                    },
                );
            }
            UpgradeKind::NoRerollTowerAttackRangePlus { range_plus } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::NoReroll,
                    TowerUpgrade::RangePlus { range: range_plus },
                );
            }
            UpgradeKind::EvenOddTowerAttackDamageMultiply {
                even,
                damage_multiplier,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::EvenOdd { even },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::EvenOddTowerAttackSpeedMultiply {
                even,
                speed_multiplier,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::EvenOdd { even },
                    TowerUpgrade::SpeedMultiplier {
                        multiplier: speed_multiplier,
                    },
                );
            }
            UpgradeKind::EvenOddTowerAttackRangePlus { even, range_plus } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::EvenOdd { even },
                    TowerUpgrade::RangePlus { range: range_plus },
                );
            }
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply {
                face,
                damage_multiplier,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::FaceNumber { face },
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply {
                face,
                speed_multiplier,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::FaceNumber { face },
                    TowerUpgrade::SpeedMultiplier {
                        multiplier: speed_multiplier,
                    },
                );
            }
            UpgradeKind::FaceNumberCardTowerAttackRangePlus { face, range_plus } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::FaceNumber { face },
                    TowerUpgrade::RangePlus { range: range_plus },
                );
            }
            UpgradeKind::ShortenStraightFlushTo4Cards => {
                self.shorten_straight_flush_to_4_cards = true;
            }
            UpgradeKind::SkipRankForStraight => {
                self.skip_rank_for_straight = true;
            }
            UpgradeKind::TreatSuitsAsSame => {
                self.treat_suits_as_same = true;
            }
            UpgradeKind::RerollTowerAttackDamageMultiply { damage_multiplier } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::Reroll,
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::RerollTowerAttackSpeedMultiply { speed_multiplier } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::Reroll,
                    TowerUpgrade::SpeedMultiplier {
                        multiplier: speed_multiplier,
                    },
                );
            }
            UpgradeKind::RerollTowerAttackRangePlus { range_plus } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::Reroll,
                    TowerUpgrade::RangePlus { range: range_plus },
                );
            }
        }
    }
    pub fn tower_upgrades(&self, tower: &Tower) -> Vec<TowerUpgradeState> {
        [
            TowerUpgradeTarget::Rank { rank: tower.rank },
            TowerUpgradeTarget::Suit { suit: tower.suit },
            TowerUpgradeTarget::TowerKind {
                tower_kind: tower.kind,
            },
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

#[derive(Debug, Clone, Copy, State)]
pub enum UpgradeKind {
    GoldEarnPlus,
    RankAttackDamageMultiply {
        rank: Rank,
        damage_multiplier: f32,
    },
    RankAttackSpeedMultiply {
        rank: Rank,
        speed_multiplier: f32,
    },
    RankAttackRangePlus {
        rank: Rank,
        range_plus: f32,
    },
    SuitAttackDamageMultiply {
        suit: Suit,
        damage_multiplier: f32,
    },
    SuitAttackSpeedMultiply {
        suit: Suit,
        speed_multiplier: f32,
    },
    SuitAttackRangePlus {
        suit: Suit,
        range_plus: f32,
    },
    HandAttackDamageMultiply {
        tower_kind: TowerKind,
        damage_multiplier: f32,
    },
    HandAttackSpeedMultiply {
        tower_kind: TowerKind,
        speed_multiplier: f32,
    },
    HandAttackRangePlus {
        tower_kind: TowerKind,
        range_plus: f32,
    },
    ShopSlotExpansion,
    RerollCountPlus,
    LowCardTowerDamageMultiply {
        damage_multiplier: f32,
    },
    LowCardTowerAttackSpeedMultiply {
        speed_multiplier: f32,
    },
    LowCardTowerAttackRangePlus {
        range_plus: f32,
    },
    ShopItemPriceMinus,
    ShopRefreshPlus,
    NoRerollTowerAttackDamageMultiply {
        damage_multiplier: f32,
    },
    NoRerollTowerAttackSpeedMultiply {
        speed_multiplier: f32,
    },
    NoRerollTowerAttackRangePlus {
        range_plus: f32,
    },
    EvenOddTowerAttackDamageMultiply {
        even: bool,
        damage_multiplier: f32,
    },
    EvenOddTowerAttackSpeedMultiply {
        even: bool,
        speed_multiplier: f32,
    },
    EvenOddTowerAttackRangePlus {
        even: bool,
        range_plus: f32,
    },
    FaceNumberCardTowerAttackDamageMultiply {
        face: bool,
        damage_multiplier: f32,
    },
    FaceNumberCardTowerAttackSpeedMultiply {
        face: bool,
        speed_multiplier: f32,
    },
    FaceNumberCardTowerAttackRangePlus {
        face: bool,
        range_plus: f32,
    },
    ShortenStraightFlushTo4Cards,
    SkipRankForStraight,
    TreatSuitsAsSame,
    RerollTowerAttackDamageMultiply {
        damage_multiplier: f32,
    },
    RerollTowerAttackSpeedMultiply {
        speed_multiplier: f32,
    },
    RerollTowerAttackRangePlus {
        range_plus: f32,
    },
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord, State)]
pub enum TowerUpgradeTarget {
    Rank { rank: Rank },
    Suit { suit: Suit },
    TowerKind { tower_kind: TowerKind },
    EvenOdd { even: bool },
    FaceNumber { face: bool },
}
#[derive(Debug, Clone, Copy, State)]
pub enum TowerUpgrade {
    DamageMultiplier { multiplier: f32 },
    SpeedMultiplier { multiplier: f32 },
    RangePlus { range: f32 },
}
#[derive(Debug, Clone, Copy, State)]
pub struct TowerUpgradeState {
    pub damage_multiplier: f32,
    pub speed_multiplier: f32,
    pub range_plus: f32,
}
impl TowerUpgradeState {
    fn apply_upgrade(&mut self, upgrade: TowerUpgrade) {
        match upgrade {
            TowerUpgrade::DamageMultiplier { multiplier } => self.damage_multiplier *= multiplier,
            TowerUpgrade::SpeedMultiplier { multiplier } => self.speed_multiplier *= multiplier,
            TowerUpgrade::RangePlus { range } => self.range_plus += range,
        }
    }
}
impl Default for TowerUpgradeState {
    fn default() -> Self {
        TowerUpgradeState {
            damage_multiplier: 1.0,
            speed_multiplier: 1.0,
            range_plus: 0.0,
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
