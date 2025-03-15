mod generation;

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

#[derive(Debug, Clone, Default)]
pub struct UpgradeState {
    pub gold_earn_plus: usize,
    pub shop_slot: usize,
    pub quest_slot: usize,
    pub quest_board_slot: usize,
    pub reroll_count_plus: usize,
    pub tower_upgrade_states: BTreeMap<TowerUpgradeTarget, TowerUpgradeState>,
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
            UpgradeKind::ShopSlot => match self.shop_slot {
                0 => self.shop_slot = 1,
                1 => self.shop_slot = 2,
                _ => unreachable!("Invalid shop slot upgrade: {}", self.shop_slot),
            },
            UpgradeKind::QuestSlot => match self.quest_slot {
                0 => self.quest_slot = 1,
                1 => self.quest_slot = 2,
                _ => unreachable!("Invalid quest slot upgrade: {}", self.quest_slot),
            },
            UpgradeKind::QuestBoardSlot => match self.quest_board_slot {
                0 => self.quest_board_slot = 1,
                1 => self.quest_board_slot = 2,
                _ => unreachable!(
                    "Invalid quest board slot upgrade: {}",
                    self.quest_board_slot
                ),
            },
            UpgradeKind::Reroll => match self.reroll_count_plus {
                0 => self.reroll_count_plus = 1,
                1 => self.reroll_count_plus = 2,
                _ => unreachable!("Invalid reroll upgrade: {}", self.reroll_count_plus),
            },
            UpgradeKind::Tower { target, upgrade } => {
                let tower_upgrade_state = self.tower_upgrade_states.entry(target).or_default();
                tower_upgrade_state.apply_upgrade(upgrade);
            }
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
}

#[derive(Debug, Clone, Copy)]
pub enum UpgradeKind {
    Tower {
        target: TowerUpgradeTarget,
        upgrade: TowerUpgrade,
    },
    ShopSlot,
    QuestSlot,
    QuestBoardSlot,
    Reroll,
    GoldEarnPlus,
}
impl UpgradeKind {
    pub fn name(&self) -> &'static str {
        match self {
            UpgradeKind::Tower { .. } => "타워 업그레이드",
            UpgradeKind::ShopSlot => "상점 슬롯 확장",
            UpgradeKind::QuestSlot => "퀘스트 슬롯 확장",
            UpgradeKind::QuestBoardSlot => "퀘스트 게시판 슬롯 확장",
            UpgradeKind::Reroll => "리롤 횟수 증가가",
            UpgradeKind::GoldEarnPlus => "골드 획득량 증가",
        }
    }
    pub fn description(&self) -> String {
        match self {
            UpgradeKind::Tower { target, upgrade } => {
                let mut description = String::new();
                match target {
                    TowerUpgradeTarget::Rank { rank } => {
                        description.push_str(&format!("{}", rank));
                    }
                    TowerUpgradeTarget::Suit { suit } => {
                        description.push_str(&format!("{}", suit));
                    }
                }
                description.push_str("타워의 ");
                match upgrade {
                    TowerUpgrade::DamagePlus { damage } => {
                        description.push_str(&format!("공격력을 {}만큼 증가시킵니다.", damage));
                    }
                    TowerUpgrade::DamageMultiplier { multiplier } => {
                        description
                            .push_str(&format!("공격력을 {}배 만큼 증가시킵니다.", multiplier));
                    }
                    TowerUpgrade::SpeedPlus { speed } => {
                        description.push_str(&format!("공격 속도를 {}만큼 증가시킵니다.", speed));
                    }
                    TowerUpgrade::SpeedMultiplier { multiplier } => {
                        description
                            .push_str(&format!("공격 속도를 {}배 만큼 증가시킵니다.", multiplier));
                    }
                    TowerUpgrade::RangePlus { range } => {
                        description.push_str(&format!("공격 범위를 {}만큼 증가시킵니다.", range));
                    }
                }
                description
            }
            UpgradeKind::ShopSlot => "상점 슬롯을 확장합니다.".to_string(),
            UpgradeKind::QuestSlot => "퀘스트 슬롯을 확장합니다.".to_string(),
            UpgradeKind::QuestBoardSlot => "퀘스트 게시판 슬롯을 확장합니다.".to_string(),
            UpgradeKind::Reroll => "리롤 횟수가 증가합니다.".to_string(),
            UpgradeKind::GoldEarnPlus => "골드 획득량이 증가합니다.".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum TowerUpgradeTarget {
    Rank { rank: Rank },
    Suit { suit: Suit },
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
impl TowerUpgrade {
    pub fn kind(&self) -> TowerUpgradeKind {
        match self {
            TowerUpgrade::DamagePlus { .. } => TowerUpgradeKind::DamagePlus,
            TowerUpgrade::DamageMultiplier { .. } => TowerUpgradeKind::DamageMultiplier,
            TowerUpgrade::SpeedPlus { .. } => TowerUpgradeKind::SpeedPlus,
            TowerUpgrade::SpeedMultiplier { .. } => TowerUpgradeKind::SpeedMultiplier,
            TowerUpgrade::RangePlus { .. } => TowerUpgradeKind::RangePlus,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum TowerUpgradeKind {
    DamagePlus,
    DamageMultiplier,
    SpeedPlus,
    SpeedMultiplier,
    RangePlus,
}
