mod generation;

use crate::{
    card::{Rank, Suit},
    game_state::tower::Tower,
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
pub enum Upgrade {
    GoldEarnPlus,
    ShopSlot,
    QuestSlot,
    QuestBoardSlot,
    Reroll,
    Tower {
        target: TowerUpgradeTarget,
        upgrade: TowerUpgrade,
    },
}

impl UpgradeState {
    pub fn upgrade(&mut self, upgrade: Upgrade) {
        match upgrade {
            Upgrade::GoldEarnPlus => match self.gold_earn_plus {
                0 => self.gold_earn_plus = 1,
                1 => self.gold_earn_plus = 2,
                2 => self.gold_earn_plus = 4,
                4 => self.gold_earn_plus = 8,
                8 => self.gold_earn_plus = 16,
                _ => unreachable!("Invalid gold earn plus upgrade: {}", self.gold_earn_plus),
            },
            Upgrade::ShopSlot => match self.shop_slot {
                0 => self.shop_slot = 1,
                1 => self.shop_slot = 2,
                _ => unreachable!("Invalid shop slot upgrade: {}", self.shop_slot),
            },
            Upgrade::QuestSlot => match self.quest_slot {
                0 => self.quest_slot = 1,
                1 => self.quest_slot = 2,
                _ => unreachable!("Invalid quest slot upgrade: {}", self.quest_slot),
            },
            Upgrade::QuestBoardSlot => match self.quest_board_slot {
                0 => self.quest_board_slot = 1,
                1 => self.quest_board_slot = 2,
                _ => unreachable!(
                    "Invalid quest board slot upgrade: {}",
                    self.quest_board_slot
                ),
            },
            Upgrade::Reroll => match self.reroll_count_plus {
                0 => self.reroll_count_plus = 1,
                1 => self.reroll_count_plus = 2,
                _ => unreachable!("Invalid reroll upgrade: {}", self.reroll_count_plus),
            },
            Upgrade::Tower { target, upgrade } => {
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

impl Upgrade {
    pub fn name(&self) -> &'static str {
        match self {
            Upgrade::Tower { .. } => "타워 업그레이드",
            Upgrade::ShopSlot => "상점 슬롯 확장",
            Upgrade::QuestSlot => "퀘스트 슬롯 확장",
            Upgrade::QuestBoardSlot => "퀘스트 게시판 슬롯 확장",
            Upgrade::Reroll => "리롤 횟수 증가가",
            Upgrade::GoldEarnPlus => "골드 획득량 증가",
        }
    }
    pub fn description(&self) -> String {
        match self {
            Upgrade::Tower { target, upgrade } => {
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
            Upgrade::ShopSlot => "상점 슬롯을 확장합니다.".to_string(),
            Upgrade::QuestSlot => "퀘스트 슬롯을 확장합니다.".to_string(),
            Upgrade::QuestBoardSlot => "퀘스트 게시판 슬롯을 확장합니다.".to_string(),
            Upgrade::Reroll => "리롤 횟수가 증가합니다.".to_string(),
            Upgrade::GoldEarnPlus => "골드 획득량이 증가합니다.".to_string(),
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
