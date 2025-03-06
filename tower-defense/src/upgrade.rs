use std::collections::BTreeMap;

use crate::{
    card::{REVERSED_RANKS, Rank, SUITS, Suit},
    game_state::{GameState, tower::Tower},
    rarity::Rarity,
};
use rand::{Rng, seq::SliceRandom, thread_rng};

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

//TODO: Call this function on clear boss stage
pub fn generate_upgrades_for_boss_reward(game_state: &GameState, amount: usize) -> Vec<Upgrade> {
    let rarity_table = generate_rarity_table_for_boss_reward(game_state.stage);
    let rarities = {
        let mut rarities = Vec::with_capacity(amount);
        for _ in 0..amount {
            let rarity = &rarity_table
                .choose_weighted(&mut rand::thread_rng(), |x| x.1)
                .unwrap()
                .0;
            rarities.push(*rarity);
        }
        rarities
    };

    let mut upgrades = Vec::with_capacity(rarities.len());
    for rarity in rarities {
        let upgrade = generate_upgrade(game_state, rarity);
        upgrades.push(upgrade);
    }
    upgrades
}
pub fn generate_upgrade(game_state: &GameState, rarity: Rarity) -> Upgrade {
    let upgrade_candidates = generate_upgrade_candidate_table(game_state, rarity);
    let upgrade_candidate = &upgrade_candidates
        .choose_weighted(&mut rand::thread_rng(), |x| x.1)
        .unwrap()
        .0;

    match upgrade_candidate {
        UpgradeCandidate::Tower => {
            let target = {
                let target_is_rank = [true, false]
                    .choose_weighted(&mut thread_rng(), |x| match x {
                        true => 1,
                        false => 3,
                    })
                    .unwrap();
                match target_is_rank {
                    true => {
                        let rank = *REVERSED_RANKS.choose(&mut thread_rng()).unwrap();
                        TowerUpgradeTarget::Rank { rank }
                    }
                    false => {
                        let suit = *SUITS.choose(&mut thread_rng()).unwrap();
                        TowerUpgradeTarget::Suit { suit }
                    }
                }
            };
            let upgrade = {
                // DamagePlus, DamageMultiplier, SpeedPlus, SpeedMultiplier, RangePlus
                let weight = match rarity {
                    Rarity::Common => [1.0, 0.1, 1.0, 0.1, 0.5],
                    Rarity::Rare => [1.0, 0.3, 1.0, 0.3, 0.5],
                    Rarity::Epic => [1.0, 0.5, 1.0, 0.5, 0.5],
                    Rarity::Legendary => [0.5, 1.0, 0.5, 1.0, 0.5],
                };
                let upgrade_candidates = TARGET_UPGRADE_CANDIDATES
                    .iter()
                    .zip(weight)
                    .collect::<Vec<_>>();
                let upgrade_candidate = upgrade_candidates
                    .choose_weighted(&mut thread_rng(), |x| x.1)
                    .unwrap();
                match upgrade_candidate.0 {
                    TargetUpgradeCandidate::DamagePlus => {
                        let damage = thread_rng().gen_range(match rarity {
                            Rarity::Common => 1.0..5.0,
                            Rarity::Rare => 5.0..10.0,
                            Rarity::Epic => 15.0..40.0,
                            Rarity::Legendary => 50.0..100.0,
                        });
                        TowerUpgrade::DamagePlus { damage }
                    }
                    TargetUpgradeCandidate::DamageMultiplier => {
                        let multiplier = thread_rng().gen_range(match rarity {
                            Rarity::Common => 1.1..1.2,
                            Rarity::Rare => 1.2..1.5,
                            Rarity::Epic => 1.5..1.75,
                            Rarity::Legendary => 1.75..2.0,
                        });
                        TowerUpgrade::DamageMultiplier { multiplier }
                    }
                    TargetUpgradeCandidate::SpeedPlus => {
                        let speed = thread_rng().gen_range(match rarity {
                            Rarity::Common => 0.1..0.25,
                            Rarity::Rare => 0.25..0.5,
                            Rarity::Epic => 0.5..0.75,
                            Rarity::Legendary => 0.75..1.0,
                        });
                        TowerUpgrade::SpeedPlus { speed }
                    }
                    TargetUpgradeCandidate::SpeedMultiplier => {
                        let multiplier = thread_rng().gen_range(match rarity {
                            Rarity::Common => 1.1..1.2,
                            Rarity::Rare => 1.2..1.5,
                            Rarity::Epic => 1.5..1.75,
                            Rarity::Legendary => 1.75..2.0,
                        });
                        TowerUpgrade::SpeedMultiplier { multiplier }
                    }
                    TargetUpgradeCandidate::RangePlus => {
                        let range = thread_rng().gen_range(match rarity {
                            Rarity::Common => 0.5..1.0,
                            Rarity::Rare => 1.0..2.0,
                            Rarity::Epic => 2.0..3.0,
                            Rarity::Legendary => 3.0..5.0,
                        });
                        TowerUpgrade::RangePlus { range }
                    }
                }
            };
            Upgrade::Tower { target, upgrade }
        }
        UpgradeCandidate::ShopSlot => Upgrade::ShopSlot,
        UpgradeCandidate::QuestSlot => Upgrade::QuestSlot,
        UpgradeCandidate::QuestBoardSlot => Upgrade::QuestBoardSlot,
        UpgradeCandidate::RerollCountPlus => Upgrade::Reroll,
        UpgradeCandidate::GoldEarnPlus => Upgrade::GoldEarnPlus,
    }
}
fn generate_rarity_table_for_boss_reward(stage: usize) -> Vec<(Rarity, f32)> {
    let rarity_weight = match stage {
        15 => [0.85, 0.15, 0.00],
        25 => [0.78, 0.2, 0.02],
        30 => [0.7, 0.25, 0.1],
        35 => [0.48, 0.4, 0.22],
        40 => [0.35, 0.25, 0.3],
        45 => [0.15, 0.2, 0.4],
        46 => [0.15, 0.2, 0.4],
        47 => [0.15, 0.2, 0.4],
        48 => [0.15, 0.2, 0.4],
        49 => [0.15, 0.2, 0.4],
        _ => panic!("Invalid stage: {}", stage),
    };
    let rarity_table = vec![
        (Rarity::Rare, rarity_weight[0]),
        (Rarity::Epic, rarity_weight[1]),
        (Rarity::Legendary, rarity_weight[2]),
    ];
    rarity_table
}
fn generate_upgrade_candidate_table(
    game_state: &GameState,
    rarity: Rarity,
) -> Vec<(UpgradeCandidate, f32)> {
    let mut upgrade_candidate_table = Vec::with_capacity(5);

    let shop_slot_upgrade = {
        let remaining_upgrade = MAX_SHOP_SLOT_UPGRADE - game_state.max_shop_slot;
        let weight = match rarity {
            Rarity::Common => [0.0, 0.0, 0.1],
            Rarity::Rare => [0.0, 0.0, 0.2],
            Rarity::Epic => [0.0, 0.1, 0.3],
            Rarity::Legendary => [0.0, 0.2, 0.4],
        };
        (
            UpgradeCandidate::ShopSlot,
            *weight
                .get(remaining_upgrade)
                .expect("Too many shop slot upgrades are available"),
        )
    };
    upgrade_candidate_table.push(shop_slot_upgrade);

    let quest_slot_upgrade = {
        let remaining_upgrade = MAX_QUEST_SLOT_UPGRADE - game_state.max_quests;
        let weight = match rarity {
            Rarity::Common => [0.0, 0.0, 0.1],
            Rarity::Rare => [0.0, 0.0, 0.2],
            Rarity::Epic => [0.0, 0.1, 0.3],
            Rarity::Legendary => [0.0, 0.2, 0.4],
        };
        (
            UpgradeCandidate::QuestSlot,
            *weight
                .get(remaining_upgrade)
                .expect("Too many quest slot upgrades are available"),
        )
    };
    upgrade_candidate_table.push(quest_slot_upgrade);

    let quest_board_slot_upgrade = {
        let remaining_upgrade = MAX_QUEST_BOARD_SLOT_UPGRADE - game_state.max_quest_board_slot;
        let weight = match rarity {
            Rarity::Common => [0.0, 0.0, 0.1],
            Rarity::Rare => [0.0, 0.0, 0.2],
            Rarity::Epic => [0.0, 0.1, 0.3],
            Rarity::Legendary => [0.0, 0.2, 0.4],
        };
        (
            UpgradeCandidate::QuestBoardSlot,
            *weight
                .get(remaining_upgrade)
                .expect("Too many quest board slot upgrades are available"),
        )
    };
    upgrade_candidate_table.push(quest_board_slot_upgrade);

    let mut candidate_table_push = |candidate: UpgradeCandidate,
                                    current: usize,
                                    max: usize,
                                    common_weight: usize,
                                    rare_weight: usize,
                                    epic_weight: usize,
                                    legendary_weight: usize| {
        upgrade_candidate_table.push((candidate, {
            if current >= max {
                0.0
            } else {
                match rarity {
                    Rarity::Common => common_weight as f32 / 100.0,
                    Rarity::Rare => rare_weight as f32 / 100.0,
                    Rarity::Epic => epic_weight as f32 / 100.0,
                    Rarity::Legendary => legendary_weight as f32 / 100.0,
                }
            }
        }));
    };

    candidate_table_push(
        UpgradeCandidate::Tower,
        usize::MIN,
        usize::MAX,
        50,
        100,
        100,
        100,
    );
    candidate_table_push(
        UpgradeCandidate::RerollCountPlus,
        game_state.upgrade_state.reroll_count_plus,
        MAX_REROLL_UPGRADE,
        5,
        10,
        50,
        100,
    );
    candidate_table_push(
        UpgradeCandidate::GoldEarnPlus,
        game_state.upgrade_state.gold_earn_plus,
        5,
        10,
        50,
        50,
        100,
    );

    upgrade_candidate_table
}

enum UpgradeCandidate {
    Tower,
    ShopSlot,
    QuestSlot,
    QuestBoardSlot,
    RerollCountPlus,
    GoldEarnPlus,
}

#[derive(Debug, Clone, Copy)]
enum TargetUpgradeCandidate {
    DamagePlus,
    DamageMultiplier,
    SpeedPlus,
    SpeedMultiplier,
    RangePlus,
}
const TARGET_UPGRADE_CANDIDATES: [TargetUpgradeCandidate; 5] = [
    TargetUpgradeCandidate::DamagePlus,
    TargetUpgradeCandidate::DamageMultiplier,
    TargetUpgradeCandidate::SpeedPlus,
    TargetUpgradeCandidate::SpeedMultiplier,
    TargetUpgradeCandidate::RangePlus,
];
