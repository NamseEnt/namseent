mod display;
mod generation;

use super::tower::TowerKind;
use crate::{
    card::{Rank, Suit},
    game_state::tower::Tower,
    rarity::Rarity,
};
pub use generation::*;
use std::collections::BTreeMap;

pub const MAX_QUEST_SLOT_UPGRADE: usize = 5;
pub const MAX_QUEST_BOARD_SLOT_UPGRADE: usize = 3;
pub const MAX_SHOP_SLOT_UPGRADE: usize = 5;
pub const MAX_REROLL_UPGRADE: usize = 2;
pub const MAX_INVENTORY_SLOT_UPGRADE: usize = 9;
pub const MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE: usize = 15;

#[derive(Debug, Clone, Default)]
pub struct UpgradeState {
    pub gold_earn_plus: usize,
    pub shop_slot: usize,
    pub quest_slot: usize,
    pub quest_board_slot: usize,
    pub reroll_count_plus: usize,
    pub tower_upgrade_states: BTreeMap<TowerUpgradeTarget, TowerUpgradeState>,
    pub tower_select_upgrade_states: BTreeMap<TowerSelectUpgradeTarget, TowerUpgradeState>,
    pub shop_item_price_minus: usize,
    pub max_shop_refresh: usize,
    pub max_quest_board_refresh: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Upgrade {
    pub kind: UpgradeKind,
    pub rarity: Rarity,
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
            UpgradeKind::RankAttackDamagePlus { rank, damage_plus } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Rank { rank },
                    TowerUpgrade::DamagePlus {
                        damage: damage_plus,
                    },
                );
            }
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
            UpgradeKind::RankAttackSpeedPlus { rank, speed_plus } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Rank { rank },
                    TowerUpgrade::SpeedPlus { speed: speed_plus },
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
            UpgradeKind::SuitAttackDamagePlus { suit, damage_plus } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Suit { suit },
                    TowerUpgrade::DamagePlus {
                        damage: damage_plus,
                    },
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
            UpgradeKind::SuitAttackSpeedPlus { suit, speed_plus } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::Suit { suit },
                    TowerUpgrade::SpeedPlus { speed: speed_plus },
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
            UpgradeKind::HandAttackDamagePlus {
                tower_kind,
                damage_plus,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::TowerKind { tower_kind },
                    TowerUpgrade::DamagePlus {
                        damage: damage_plus,
                    },
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
            UpgradeKind::HandAttackSpeedPlus {
                tower_kind,
                speed_plus,
            } => {
                self.apply_tower_upgrade(
                    TowerUpgradeTarget::TowerKind { tower_kind },
                    TowerUpgrade::SpeedPlus { speed: speed_plus },
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
            UpgradeKind::ShopSlotExpansion => match self.shop_slot {
                0 => self.shop_slot = 1,
                1 => self.shop_slot = 2,
                _ => unreachable!("Invalid shop slot upgrade: {}", self.shop_slot),
            },
            UpgradeKind::QuestSlotExpansion => match self.quest_slot {
                0 => self.quest_slot = 1,
                1 => self.quest_slot = 2,
                _ => unreachable!("Invalid quest slot upgrade: {}", self.quest_slot),
            },
            UpgradeKind::QuestBoardExpansion => match self.quest_board_slot {
                0 => self.quest_board_slot = 1,
                1 => self.quest_board_slot = 2,
                _ => unreachable!(
                    "Invalid quest board slot upgrade: {}",
                    self.quest_board_slot
                ),
            },
            UpgradeKind::RerollCountPlus => match self.reroll_count_plus {
                0 => self.reroll_count_plus = 1,
                1 => self.reroll_count_plus = 2,
                _ => unreachable!("Invalid reroll upgrade: {}", self.reroll_count_plus),
            },
            UpgradeKind::LowCardTowerDamagePlus { damage_plus } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::LowCard,
                    TowerUpgrade::DamagePlus {
                        damage: damage_plus,
                    },
                );
            }
            UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::LowCard,
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::LowCardTowerAttackSpeedPlus { speed_plus } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::LowCard,
                    TowerUpgrade::SpeedPlus { speed: speed_plus },
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
            UpgradeKind::ShopRefreshPlus => match self.max_shop_refresh {
                0 => self.max_shop_refresh = 1,
                1 => self.max_shop_refresh = 2,
                2 => self.max_shop_refresh = 3,
                _ => unreachable!("Invalid shop refresh upgrade: {}", self.max_shop_refresh),
            },
            UpgradeKind::QuestBoardRefreshPlus => match self.max_quest_board_refresh {
                0 => self.max_quest_board_refresh = 1,
                1 => self.max_quest_board_refresh = 2,
                2 => self.max_quest_board_refresh = 3,
                _ => unreachable!(
                    "Invalid quest board refresh upgrade: {}",
                    self.max_quest_board_refresh
                ),
            },
            UpgradeKind::NoRerollTowerAttackDamagePlus { damage_plus } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::NoReroll,
                    TowerUpgrade::DamagePlus {
                        damage: damage_plus,
                    },
                );
            }
            UpgradeKind::NoRerollTowerAttackDamageMultiply { damage_multiplier } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::NoReroll,
                    TowerUpgrade::DamageMultiplier {
                        multiplier: damage_multiplier,
                    },
                );
            }
            UpgradeKind::NoRerollTowerAttackSpeedPlus { speed_plus } => {
                self.apply_tower_select_upgrade(
                    TowerSelectUpgradeTarget::NoReroll,
                    TowerUpgrade::SpeedPlus { speed: speed_plus },
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
            UpgradeKind::EvenOddTowerAttackDamagePlus { .. } => todo!(),
            UpgradeKind::EvenOddTowerAttackDamageMultiply { .. } => todo!(),
            UpgradeKind::EvenOddTowerAttackSpeedPlus { .. } => todo!(),
            UpgradeKind::EvenOddTowerAttackSpeedMultiply { .. } => todo!(),
            UpgradeKind::EvenOddTowerAttackRangePlus { .. } => todo!(),
            UpgradeKind::FaceNumberCardTowerAttackDamagePlus { .. } => todo!(),
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { .. } => todo!(),
            UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { .. } => todo!(),
            UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply { .. } => todo!(),
            UpgradeKind::FaceNumberCardTowerAttackRangePlus { .. } => todo!(),
            UpgradeKind::ShortenStraightFlushTo4Cards => todo!(),
            UpgradeKind::SkipRankForStraight => todo!(),
            UpgradeKind::TreatSuitsAsSame => todo!(),
            UpgradeKind::RerollTowerAttackDamagePlus { .. } => todo!(),
            UpgradeKind::RerollTowerAttackDamageMultiply { .. } => todo!(),
            UpgradeKind::RerollTowerAttackSpeedPlus { .. } => todo!(),
            UpgradeKind::RerollTowerAttackSpeedMultiply { .. } => todo!(),
            UpgradeKind::RerollTowerAttackRangePlus { .. } => todo!(),
        }
    }
    pub fn tower_upgrades(&self, tower: &Tower) -> Vec<TowerUpgradeState> {
        [
            TowerUpgradeTarget::Rank { rank: tower.rank },
            TowerUpgradeTarget::Suit { suit: tower.suit },
        ]
        .iter()
        .map(|target| {
            self.tower_upgrade_states
                .get(target)
                .map(|x| *x)
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

#[derive(Debug, Clone, Copy)]
pub enum UpgradeKind {
    GoldEarnPlus,
    RankAttackDamagePlus {
        rank: Rank,
        damage_plus: f32,
    },
    RankAttackDamageMultiply {
        rank: Rank,
        damage_multiplier: f32,
    },
    RankAttackSpeedPlus {
        rank: Rank,
        speed_plus: f32,
    },
    RankAttackSpeedMultiply {
        rank: Rank,
        speed_multiplier: f32,
    },
    RankAttackRangePlus {
        rank: Rank,
        range_plus: f32,
    },
    SuitAttackDamagePlus {
        suit: Suit,
        damage_plus: f32,
    },
    SuitAttackDamageMultiply {
        suit: Suit,
        damage_multiplier: f32,
    },
    SuitAttackSpeedPlus {
        suit: Suit,
        speed_plus: f32,
    },
    SuitAttackSpeedMultiply {
        suit: Suit,
        speed_multiplier: f32,
    },
    SuitAttackRangePlus {
        suit: Suit,
        range_plus: f32,
    },
    HandAttackDamagePlus {
        tower_kind: TowerKind,
        damage_plus: f32,
    },
    HandAttackDamageMultiply {
        tower_kind: TowerKind,
        damage_multiplier: f32,
    },
    HandAttackSpeedPlus {
        tower_kind: TowerKind,
        speed_plus: f32,
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
    QuestSlotExpansion,
    QuestBoardExpansion,
    RerollCountPlus,
    LowCardTowerDamagePlus {
        damage_plus: f32,
    },
    LowCardTowerDamageMultiply {
        damage_multiplier: f32,
    },
    LowCardTowerAttackSpeedPlus {
        speed_plus: f32,
    },
    LowCardTowerAttackSpeedMultiply {
        speed_multiplier: f32,
    },
    LowCardTowerAttackRangePlus {
        range_plus: f32,
    },
    ShopItemPriceMinus,
    ShopRefreshPlus,
    QuestBoardRefreshPlus,
    NoRerollTowerAttackDamagePlus {
        damage_plus: f32,
    },
    NoRerollTowerAttackDamageMultiply {
        damage_multiplier: f32,
    },
    NoRerollTowerAttackSpeedPlus {
        speed_plus: f32,
    },
    NoRerollTowerAttackSpeedMultiply {
        speed_multiplier: f32,
    },
    NoRerollTowerAttackRangePlus {
        range_plus: f32,
    },
    EvenOddTowerAttackDamagePlus {
        damage_plus: f32,
    },
    EvenOddTowerAttackDamageMultiply {
        damage_multiplier: f32,
    },
    EvenOddTowerAttackSpeedPlus {
        speed_plus: f32,
    },
    EvenOddTowerAttackSpeedMultiply {
        speed_multiplier: f32,
    },
    EvenOddTowerAttackRangePlus {
        range_plus: f32,
    },
    FaceNumberCardTowerAttackDamagePlus {
        damage_plus: f32,
    },
    FaceNumberCardTowerAttackDamageMultiply {
        damage_multiplier: f32,
    },
    FaceNumberCardTowerAttackSpeedPlus {
        speed_plus: f32,
    },
    FaceNumberCardTowerAttackSpeedMultiply {
        speed_multiplier: f32,
    },
    FaceNumberCardTowerAttackRangePlus {
        range_plus: f32,
    },
    ShortenStraightFlushTo4Cards,
    SkipRankForStraight,
    TreatSuitsAsSame,
    RerollTowerAttackDamagePlus {
        damage_plus: f32,
    },
    RerollTowerAttackDamageMultiply {
        damage_multiplier: f32,
    },
    RerollTowerAttackSpeedPlus {
        speed_plus: f32,
    },
    RerollTowerAttackSpeedMultiply {
        speed_multiplier: f32,
    },
    RerollTowerAttackRangePlus {
        range_plus: f32,
    },
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum TowerUpgradeTarget {
    Rank { rank: Rank },
    Suit { suit: Suit },
    TowerKind { tower_kind: TowerKind },
}
#[derive(Debug, Clone, Copy)]
pub enum TowerUpgrade {
    DamagePlus { damage: f32 },
    DamageMultiplier { multiplier: f32 },
    SpeedPlus { speed: f32 },
    SpeedMultiplier { multiplier: f32 },
    RangePlus { range: f32 },
}
#[derive(Debug, Clone, Copy)]
pub struct TowerUpgradeState {
    pub damage_plus: f32,
    pub damage_multiplier: f32,
    pub speed_plus: f32,
    pub speed_multiplier: f32,
    pub range_plus: f32,
}
impl TowerUpgradeState {
    fn apply_upgrade(&mut self, upgrade: TowerUpgrade) {
        match upgrade {
            TowerUpgrade::DamagePlus { damage } => self.damage_plus += damage,
            TowerUpgrade::DamageMultiplier { multiplier } => self.damage_multiplier *= multiplier,
            TowerUpgrade::SpeedPlus { speed } => self.speed_plus += speed,
            TowerUpgrade::SpeedMultiplier { multiplier } => self.speed_multiplier *= multiplier,
            TowerUpgrade::RangePlus { range } => self.range_plus += range,
        }
    }
}
impl Default for TowerUpgradeState {
    fn default() -> Self {
        TowerUpgradeState {
            damage_plus: 0.0,
            damage_multiplier: 1.0,
            speed_plus: 0.0,
            speed_multiplier: 1.0,
            range_plus: 0.0,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum TowerSelectUpgradeTarget {
    LowCard,
    NoReroll,
}
