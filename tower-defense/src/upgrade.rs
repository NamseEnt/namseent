use crate::{
    card::{Rank, Suit, REVERSED_RANKS, SUITS},
    game_state::GameState,
    rarity::Rarity,
};
use rand::{seq::SliceRandom, thread_rng, Rng};

pub const MAX_QUEST_SLOT_UPGRADE: usize = 5;
pub const MAX_QUEST_BOARD_SLOT_UPGRADE: usize = 3;
pub const MAX_SHOP_SLOT_UPGRADE: usize = 5;
pub const MAX_REROLL_UPGRADE: usize = 2;

#[derive(Debug, Clone)]
pub enum Upgrade {
    Tower {
        target: TowerUpgradeTarget,
        upgrade: TowerUpgrade,
    },
    ShopSlot {
        extra_slot: usize,
    },
    QuestSlot {
        extra_slot: usize,
    },
    QuestBoardSlot {
        extra_slot: usize,
    },
    Reroll {
        extra_reroll: usize,
    },
}
pub fn merge_or_append_upgrade(upgrades: &mut Vec<Upgrade>, upgrade: Upgrade) {
    match &upgrade {
        Upgrade::Tower {
            target,
            upgrade: tower_upgrade,
        } => {
            for existing_upgrade in upgrades.iter_mut() {
                let Upgrade::Tower {
                    target: existing_target,
                    upgrade: existing_upgrade,
                } = existing_upgrade
                else {
                    continue;
                };

                if target != existing_target {
                    continue;
                }

                match (tower_upgrade, existing_upgrade) {
                    (
                        TowerUpgrade::DamagePlus { damage },
                        TowerUpgrade::DamagePlus {
                            damage: existing_damage,
                        },
                    ) => {
                        *existing_damage += damage;
                        return;
                    }
                    (
                        TowerUpgrade::DamageMultiplier { multiplier },
                        TowerUpgrade::DamageMultiplier {
                            multiplier: existing_multiplier,
                        },
                    ) => {
                        *existing_multiplier *= multiplier;
                        return;
                    }
                    (
                        TowerUpgrade::SpeedPlus { speed },
                        TowerUpgrade::SpeedPlus {
                            speed: existing_speed,
                        },
                    ) => {
                        *existing_speed += speed;
                        return;
                    }
                    (
                        TowerUpgrade::SpeedMultiplier { multiplier },
                        TowerUpgrade::SpeedMultiplier {
                            multiplier: existing_multiplier,
                        },
                    ) => {
                        *existing_multiplier *= multiplier;
                        return;
                    }
                    (
                        TowerUpgrade::RangePlus { range },
                        TowerUpgrade::RangePlus {
                            range: existing_range,
                        },
                    ) => {
                        *existing_range += range;
                        return;
                    }
                    _ => {}
                }
            }
        }
        Upgrade::ShopSlot { extra_slot } => {
            for existing_upgrade in upgrades.iter_mut() {
                let Upgrade::ShopSlot {
                    extra_slot: existing_extra_slot,
                } = existing_upgrade
                else {
                    continue;
                };

                *existing_extra_slot += extra_slot;
                return;
            }
        }
        Upgrade::QuestSlot { extra_slot } => {
            for existing_upgrade in upgrades.iter_mut() {
                let Upgrade::QuestSlot {
                    extra_slot: existing_extra_slot,
                } = existing_upgrade
                else {
                    continue;
                };

                *existing_extra_slot += extra_slot;
                return;
            }
        }
        Upgrade::QuestBoardSlot { extra_slot } => {
            for existing_upgrade in upgrades.iter_mut() {
                let Upgrade::QuestBoardSlot {
                    extra_slot: existing_extra_slot,
                } = existing_upgrade
                else {
                    continue;
                };

                *existing_extra_slot += extra_slot;
                return;
            }
        }
        Upgrade::Reroll { extra_reroll } => {
            for existing_upgrade in upgrades.iter_mut() {
                let Upgrade::Reroll {
                    extra_reroll: existing_extra_reroll,
                } = existing_upgrade
                else {
                    continue;
                };

                *existing_extra_reroll += extra_reroll;
                return;
            }
        }
    }

    upgrades.push(upgrade);
}
impl Upgrade {
    pub fn name(&self) -> &'static str {
        match self {
            Upgrade::Tower { .. } => "타워 업그레이드",
            Upgrade::ShopSlot { .. } => "상점 슬롯 확장",
            Upgrade::QuestSlot { .. } => "퀘스트 슬롯 확장",
            Upgrade::QuestBoardSlot { .. } => "퀘스트 게시판 슬롯 확장",
            Upgrade::Reroll { .. } => "리롤 횟수 증가가",
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
            Upgrade::ShopSlot { .. } => "상점 슬롯을 확장합니다.".to_string(),
            Upgrade::QuestSlot { .. } => "퀘스트 슬롯을 확장합니다.".to_string(),
            Upgrade::QuestBoardSlot { .. } => "퀘스트 게시판 슬롯을 확장합니다.".to_string(),
            Upgrade::Reroll { .. } => "리롤 횟수를 증가시킵니다.".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TowerUpgradeTarget {
    Rank { rank: Rank },
    Suit { suit: Suit },
}
#[derive(Debug, Clone)]
pub enum TowerUpgrade {
    DamagePlus { damage: f32 },
    DamageMultiplier { multiplier: f32 },
    SpeedPlus { speed: f32 },
    SpeedMultiplier { multiplier: f32 },
    RangePlus { range: f32 },
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
        UpgradeCandidate::ShopSlot => Upgrade::ShopSlot { extra_slot: 1 },
        UpgradeCandidate::QuestSlot => Upgrade::QuestSlot { extra_slot: 1 },
        UpgradeCandidate::QuestBoardSlot => Upgrade::QuestBoardSlot { extra_slot: 1 },
        UpgradeCandidate::Reroll => Upgrade::Reroll { extra_reroll: 1 },
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
    upgrade_candidate_table.push((UpgradeCandidate::Tower, 1.0));

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
        let remaining_upgrade = MAX_QUEST_SLOT_UPGRADE - game_state.max_quest_slot;
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

    let reroll_upgrade = {
        let remaining_upgrade = MAX_REROLL_UPGRADE - game_state.reroll;
        let weight = match rarity {
            Rarity::Common => [0.0, 0.0, 0.0],
            Rarity::Rare => [0.0, 0.0, 0.1],
            Rarity::Epic => [0.0, 0.1, 0.2],
            Rarity::Legendary => [0.0, 0.4, 0.6],
        };
        (
            UpgradeCandidate::Reroll,
            *weight
                .get(remaining_upgrade)
                .expect("Too many reroll upgrades are available"),
        )
    };
    upgrade_candidate_table.push(reroll_upgrade);

    upgrade_candidate_table
}
enum UpgradeCandidate {
    Tower,
    ShopSlot,
    QuestSlot,
    QuestBoardSlot,
    Reroll,
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
