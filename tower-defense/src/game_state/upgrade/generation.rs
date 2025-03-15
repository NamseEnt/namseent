use super::{
    MAX_QUEST_BOARD_SLOT_UPGRADE, MAX_QUEST_SLOT_UPGRADE, MAX_REROLL_UPGRADE,
    MAX_SHOP_SLOT_UPGRADE, TowerUpgrade, TowerUpgradeTarget, Upgrade,
};
use crate::{
    card::{REVERSED_RANKS, SUITS},
    game_state::GameState,
    rarity::Rarity,
};
use rand::{Rng, seq::SliceRandom, thread_rng};

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
