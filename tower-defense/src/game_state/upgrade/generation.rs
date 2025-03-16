use super::{
    MAX_QUEST_BOARD_SLOT_UPGRADE, MAX_QUEST_SLOT_UPGRADE, MAX_REROLL_UPGRADE,
    MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE, MAX_SHOP_SLOT_UPGRADE, Upgrade, UpgradeKind,
};
use crate::{
    card::{REVERSED_RANKS, SUITS},
    game_state::{GameState, tower::TowerKind},
    rarity::Rarity,
};
use rand::{Rng, seq::SliceRandom, thread_rng};
use std::usize;

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

    let kind = match upgrade_candidate {
        UpgradeCandidate::GoldEarnPlus => UpgradeKind::GoldEarnPlus,
        UpgradeCandidate::RankAttackDamagePlus => {
            let rank = *REVERSED_RANKS.choose(&mut thread_rng()).unwrap();
            let damage_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.0..5.0,
                Rarity::Rare => 5.0..10.0,
                Rarity::Epic => 15.0..40.0,
                Rarity::Legendary => 50.0..100.0,
            });
            UpgradeKind::RankAttackDamagePlus { rank, damage_plus }
        }
        UpgradeCandidate::RankAttackDamageMultiply => {
            let rank = *REVERSED_RANKS.choose(&mut thread_rng()).unwrap();
            let damage_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.5,
                Rarity::Epic => 1.5..1.75,
                Rarity::Legendary => 1.75..2.0,
            });
            UpgradeKind::RankAttackDamageMultiply {
                rank,
                damage_multiplier,
            }
        }
        UpgradeCandidate::RankAttackSpeedPlus => {
            let rank = *REVERSED_RANKS.choose(&mut thread_rng()).unwrap();
            let speed_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.1..0.25,
                Rarity::Rare => 0.25..0.5,
                Rarity::Epic => 0.5..0.75,
                Rarity::Legendary => 0.75..1.0,
            });
            UpgradeKind::RankAttackSpeedPlus { rank, speed_plus }
        }
        UpgradeCandidate::RankAttackSpeedMultiply => {
            let rank = *REVERSED_RANKS.choose(&mut thread_rng()).unwrap();
            let speed_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.5,
                Rarity::Epic => 1.5..1.75,
                Rarity::Legendary => 1.75..2.0,
            });
            UpgradeKind::RankAttackSpeedMultiply {
                rank,
                speed_multiplier,
            }
        }
        UpgradeCandidate::RankAttackRangePlus => {
            let rank = *REVERSED_RANKS.choose(&mut thread_rng()).unwrap();
            let range_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.5,
                Rarity::Epic => 1.5..1.75,
                Rarity::Legendary => 1.75..2.0,
            });
            UpgradeKind::RankAttackRangePlus { rank, range_plus }
        }
        UpgradeCandidate::SuitAttackDamagePlus => {
            let suit = *SUITS.choose(&mut thread_rng()).unwrap();
            let damage_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.0..5.0,
                Rarity::Rare => 5.0..10.0,
                Rarity::Epic => 15.0..40.0,
                Rarity::Legendary => 50.0..100.0,
            });
            UpgradeKind::SuitAttackDamagePlus { suit, damage_plus }
        }
        UpgradeCandidate::SuitAttackDamageMultiply => {
            let suit = *SUITS.choose(&mut thread_rng()).unwrap();
            let damage_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.5,
                Rarity::Epic => 1.5..1.75,
                Rarity::Legendary => 1.75..2.0,
            });
            UpgradeKind::SuitAttackDamageMultiply {
                suit,
                damage_multiplier,
            }
        }
        UpgradeCandidate::SuitAttackSpeedPlus => {
            let suit = *SUITS.choose(&mut thread_rng()).unwrap();
            let speed_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.1..0.25,
                Rarity::Rare => 0.25..0.5,
                Rarity::Epic => 0.5..0.75,
                Rarity::Legendary => 0.75..1.0,
            });
            UpgradeKind::SuitAttackSpeedPlus { suit, speed_plus }
        }
        UpgradeCandidate::SuitAttackSpeedMultiply => {
            let suit = *SUITS.choose(&mut thread_rng()).unwrap();
            let speed_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.5,
                Rarity::Epic => 1.5..1.75,
                Rarity::Legendary => 1.75..2.0,
            });
            UpgradeKind::SuitAttackSpeedMultiply {
                suit,
                speed_multiplier,
            }
        }
        UpgradeCandidate::SuitAttackRangePlus => {
            let suit = *SUITS.choose(&mut thread_rng()).unwrap();
            let range_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.5,
                Rarity::Epic => 1.5..1.75,
                Rarity::Legendary => 1.75..2.0,
            });
            UpgradeKind::SuitAttackRangePlus { suit, range_plus }
        }
        UpgradeCandidate::HandAttackDamagePlus => {
            let tower_kind =
                get_tower_kind_with_weight(&[11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 6.0, 6.0, 3.0, 2.0]);
            let damage_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 5.0..10.0,
                Rarity::Rare => 10.0..25.0,
                Rarity::Epic => 25.0..50.0,
                Rarity::Legendary => 50.0..125.0,
            });
            UpgradeKind::HandAttackDamagePlus {
                tower_kind,
                damage_plus,
            }
        }
        UpgradeCandidate::HandAttackDamageMultiply => {
            let tower_kind =
                get_tower_kind_with_weight(&[11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 6.0, 6.0, 3.0, 2.0]);
            let damage_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.2..1.4,
                Rarity::Rare => 1.4..1.6,
                Rarity::Epic => 1.6..1.85,
                Rarity::Legendary => 1.85..2.5,
            });
            UpgradeKind::HandAttackDamageMultiply {
                tower_kind,
                damage_multiplier,
            }
        }
        UpgradeCandidate::HandAttackSpeedPlus => {
            let tower_kind =
                get_tower_kind_with_weight(&[11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 6.0, 6.0, 3.0, 2.0]);
            let speed_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.2..0.4,
                Rarity::Rare => 0.4..0.6,
                Rarity::Epic => 0.6..0.85,
                Rarity::Legendary => 0.85..1.25,
            });
            UpgradeKind::HandAttackSpeedPlus {
                tower_kind,
                speed_plus,
            }
        }
        UpgradeCandidate::HandAttackSpeedMultiply => {
            let tower_kind =
                get_tower_kind_with_weight(&[11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 6.0, 6.0, 3.0, 2.0]);
            let speed_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.2..1.25,
                Rarity::Rare => 1.25..1.5,
                Rarity::Epic => 1.5..1.8,
                Rarity::Legendary => 1.8..2.1,
            });
            UpgradeKind::HandAttackSpeedMultiply {
                tower_kind,
                speed_multiplier,
            }
        }
        UpgradeCandidate::HandAttackRangePlus => {
            let tower_kind =
                get_tower_kind_with_weight(&[11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 6.0, 6.0, 3.0, 2.0]);
            let range_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.25,
                Rarity::Rare => 1.25..1.6,
                Rarity::Epic => 1.6..1.85,
                Rarity::Legendary => 1.85..2.15,
            });
            UpgradeKind::HandAttackRangePlus {
                tower_kind,
                range_plus,
            }
        }
        UpgradeCandidate::ShopSlotExpansion => UpgradeKind::ShopSlotExpansion,
        UpgradeCandidate::QuestSlotExpansion => UpgradeKind::QuestSlotExpansion,
        UpgradeCandidate::QuestBoardExpansion => UpgradeKind::QuestBoardExpansion,
        UpgradeCandidate::RerollCountPlus => UpgradeKind::RerollCountPlus,
        UpgradeCandidate::LowCardTowerDamagePlus => {
            let damage_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 5.0..15.0,
                Rarity::Rare => 15.0..25.0,
                Rarity::Epic => 25.0..75.0,
                Rarity::Legendary => 75.0..125.0,
            });
            UpgradeKind::LowCardTowerDamagePlus { damage_plus }
        }
        UpgradeCandidate::LowCardTowerDamageMultiply => {
            let damage_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.25..1.5,
                Rarity::Rare => 1.5..1.75,
                Rarity::Epic => 1.75..2.0,
                Rarity::Legendary => 2.0..2.5,
            });
            UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier }
        }
        UpgradeCandidate::LowCardTowerAttackSpeedPlus => {
            let speed_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.15..0.3,
                Rarity::Rare => 0.3..0.55,
                Rarity::Epic => 0.55..0.8,
                Rarity::Legendary => 0.8..1.1,
            });
            UpgradeKind::LowCardTowerAttackSpeedPlus { speed_plus }
        }
        UpgradeCandidate::LowCardTowerAttackSpeedMultiply => {
            let speed_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.25,
                Rarity::Rare => 1.25..1.55,
                Rarity::Epic => 1.55..1.8,
                Rarity::Legendary => 1.8..2.1,
            });
            UpgradeKind::LowCardTowerAttackSpeedMultiply { speed_multiplier }
        }
        UpgradeCandidate::LowCardTowerAttackRangePlus => {
            let range_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.15..1.3,
                Rarity::Rare => 1.3..1.55,
                Rarity::Epic => 1.55..1.9,
                Rarity::Legendary => 1.9..2.2,
            });
            UpgradeKind::LowCardTowerAttackRangePlus { range_plus }
        }
        UpgradeCandidate::ShopItemPriceMinus => UpgradeKind::ShopItemPriceMinus,
        UpgradeCandidate::ShopRefreshPlus => UpgradeKind::ShopRefreshPlus,
        UpgradeCandidate::QuestBoardRefreshPlus => todo!(),
        UpgradeCandidate::NoRerollTowerAttackDamagePlus => todo!(),
        UpgradeCandidate::NoRerollTowerAttackDamageMultiply => todo!(),
        UpgradeCandidate::NoRerollTowerAttackSpeedPlus => todo!(),
        UpgradeCandidate::NoRerollTowerAttackSpeedMultiply => todo!(),
        UpgradeCandidate::NoRerollTowerAttackRangePlus => todo!(),
        UpgradeCandidate::EvenOddTowerAttackDamagePlus => todo!(),
        UpgradeCandidate::EvenOddTowerAttackDamageMultiply => todo!(),
        UpgradeCandidate::EvenOddTowerAttackSpeedPlus => todo!(),
        UpgradeCandidate::EvenOddTowerAttackSpeedMultiply => todo!(),
        UpgradeCandidate::EvenOddTowerAttackRangePlus => todo!(),
        UpgradeCandidate::FaceNumberCardTowerAttackDamagePlus => todo!(),
        UpgradeCandidate::FaceNumberCardTowerAttackDamageMultiply => todo!(),
        UpgradeCandidate::FaceNumberCardTowerAttackSpeedPlus => todo!(),
        UpgradeCandidate::FaceNumberCardTowerAttackSpeedMultiply => todo!(),
        UpgradeCandidate::FaceNumberCardTowerAttackRangePlus => todo!(),
        UpgradeCandidate::ShortenStraightFlushTo4Cards => todo!(),
        UpgradeCandidate::SkipRankForStraight => todo!(),
        UpgradeCandidate::TreatSuitsAsSame => todo!(),
        UpgradeCandidate::RerollTowerAttackDamagePlus => todo!(),
        UpgradeCandidate::RerollTowerAttackDamageMultiply => todo!(),
        UpgradeCandidate::RerollTowerAttackSpeedPlus => todo!(),
        UpgradeCandidate::RerollTowerAttackSpeedMultiply => todo!(),
        UpgradeCandidate::RerollTowerAttackRangePlus => todo!(),
    };

    Upgrade { kind, rarity }
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
                    Rarity::Common => common_weight as f32,
                    Rarity::Rare => rare_weight as f32,
                    Rarity::Epic => epic_weight as f32,
                    Rarity::Legendary => legendary_weight as f32,
                }
            }
        }));
    };

    // GoldEarnPlus
    candidate_table_push(
        UpgradeCandidate::GoldEarnPlus,
        game_state.upgrade_state.gold_earn_plus,
        5,
        10,
        50,
        50,
        100,
    );

    // RankAttackDamagePlus
    candidate_table_push(
        UpgradeCandidate::RankAttackDamagePlus,
        usize::MIN,
        usize::MAX,
        38,
        75,
        75,
        75,
    );

    // RankAttackDamageMultiply
    candidate_table_push(
        UpgradeCandidate::RankAttackDamageMultiply,
        usize::MIN,
        usize::MAX,
        19,
        38,
        38,
        38,
    );

    // RankAttackSpeedPlus
    candidate_table_push(
        UpgradeCandidate::RankAttackSpeedPlus,
        usize::MIN,
        usize::MAX,
        30,
        60,
        60,
        60,
    );

    // RankAttackSpeedMultiply
    candidate_table_push(
        UpgradeCandidate::RankAttackSpeedMultiply,
        usize::MIN,
        usize::MAX,
        15,
        30,
        30,
        30,
    );

    // RankAttackRangePlus
    candidate_table_push(
        UpgradeCandidate::RankAttackRangePlus,
        usize::MIN,
        usize::MAX,
        8,
        15,
        15,
        15,
    );

    // SuitAttackDamagePlus
    candidate_table_push(
        UpgradeCandidate::SuitAttackDamagePlus,
        usize::MIN,
        usize::MAX,
        13,
        25,
        25,
        25,
    );

    // SuitAttackDamageMultiply
    candidate_table_push(
        UpgradeCandidate::SuitAttackDamageMultiply,
        usize::MIN,
        usize::MAX,
        6,
        13,
        13,
        13,
    );

    // SuitAttackSpeedPlus
    candidate_table_push(
        UpgradeCandidate::SuitAttackSpeedPlus,
        usize::MIN,
        usize::MAX,
        10,
        20,
        20,
        20,
    );

    // SuitAttackSpeedMultiply
    candidate_table_push(
        UpgradeCandidate::SuitAttackSpeedMultiply,
        usize::MIN,
        usize::MAX,
        5,
        10,
        10,
        10,
    );

    // SuitAttackRangePlus
    candidate_table_push(
        UpgradeCandidate::SuitAttackRangePlus,
        usize::MIN,
        usize::MAX,
        3,
        5,
        5,
        5,
    );

    // HandAttackDamagePlus
    candidate_table_push(
        UpgradeCandidate::HandAttackDamagePlus,
        usize::MIN,
        usize::MAX,
        50,
        100,
        100,
        50,
    );

    // HandAttackDamageMultiply
    candidate_table_push(
        UpgradeCandidate::HandAttackDamageMultiply,
        usize::MIN,
        usize::MAX,
        25,
        50,
        50,
        25,
    );

    // HandAttackSpeedPlus
    candidate_table_push(
        UpgradeCandidate::HandAttackSpeedPlus,
        usize::MIN,
        usize::MAX,
        40,
        80,
        80,
        40,
    );

    // HandAttackSpeedMultiply
    candidate_table_push(
        UpgradeCandidate::HandAttackSpeedMultiply,
        usize::MIN,
        usize::MAX,
        20,
        40,
        40,
        20,
    );

    // HandAttackRangePlus
    candidate_table_push(
        UpgradeCandidate::HandAttackRangePlus,
        usize::MIN,
        usize::MAX,
        10,
        20,
        20,
        10,
    );

    // ShopSlotExpansion
    candidate_table_push(
        UpgradeCandidate::ShopSlotExpansion,
        game_state.upgrade_state.shop_slot,
        MAX_SHOP_SLOT_UPGRADE,
        10,
        50,
        50,
        100,
    );

    // QuestSlotExpansion
    candidate_table_push(
        UpgradeCandidate::QuestSlotExpansion,
        game_state.upgrade_state.quest_slot,
        MAX_QUEST_SLOT_UPGRADE,
        10,
        50,
        50,
        100,
    );

    // QuestBoardExpansion
    candidate_table_push(
        UpgradeCandidate::QuestBoardExpansion,
        game_state.upgrade_state.quest_board_slot,
        MAX_QUEST_BOARD_SLOT_UPGRADE,
        10,
        50,
        50,
        100,
    );

    // RerollCountPlus
    candidate_table_push(
        UpgradeCandidate::RerollCountPlus,
        game_state.upgrade_state.reroll_count_plus,
        MAX_REROLL_UPGRADE,
        5,
        10,
        50,
        100,
    );

    // LowCardTowerDamagePlus
    candidate_table_push(
        UpgradeCandidate::LowCardTowerDamagePlus,
        usize::MIN,
        usize::MAX,
        50,
        100,
        100,
        50,
    );

    // LowCardTowerDamageMultiply
    candidate_table_push(
        UpgradeCandidate::LowCardTowerDamageMultiply,
        usize::MIN,
        usize::MAX,
        25,
        50,
        50,
        25,
    );

    // LowCardTowerAttackSpeedPlus
    candidate_table_push(
        UpgradeCandidate::LowCardTowerAttackSpeedPlus,
        usize::MIN,
        usize::MAX,
        40,
        80,
        80,
        40,
    );

    // LowCardTowerAttackSpeedMultiply
    candidate_table_push(
        UpgradeCandidate::LowCardTowerAttackSpeedMultiply,
        usize::MIN,
        usize::MAX,
        20,
        40,
        40,
        20,
    );

    // LowCardTowerAttackRangePlus
    candidate_table_push(
        UpgradeCandidate::LowCardTowerAttackRangePlus,
        usize::MIN,
        usize::MAX,
        10,
        20,
        20,
        10,
    );

    // ShopItemPriceMinus
    candidate_table_push(
        UpgradeCandidate::ShopItemPriceMinus,
        game_state.upgrade_state.shop_item_price_minus,
        MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE,
        10,
        50,
        50,
        100,
    );

    // ShopRefreshPlus
    candidate_table_push(
        UpgradeCandidate::ShopRefreshPlus,
        game_state.upgrade_state.max_shop_refresh,
        3,
        10,
        50,
        50,
        100,
    );

    // QuestBoardRefreshPlus

    // NoRerollTowerAttackDamagePlus

    // NoRerollTowerAttackDamageMultiply

    // NoRerollTowerAttackSpeedPlus

    // NoRerollTowerAttackSpeedMultiply

    // NoRerollTowerAttackRangePlus

    // EvenOddTowerAttackDamagePlus

    // EvenOddTowerAttackDamageMultiply

    // EvenOddTowerAttackSpeedPlus

    // EvenOddTowerAttackSpeedMultiply

    // EvenOddTowerAttackRangePlus

    // FaceNumberCardTowerAttackDamagePlus

    // FaceNumberCardTowerAttackDamageMultiply

    // FaceNumberCardTowerAttackSpeedPlus

    // FaceNumberCardTowerAttackSpeedMultiply

    // FaceNumberCardTowerAttackRangePlus

    // ShortenStraightFlushTo4Cards

    // SkipRankForStraight

    // TreatSuitsAsSame

    // RerollTowerAttackDamagePlus

    // RerollTowerAttackDamageMultiply

    // RerollTowerAttackSpeedPlus

    // RerollTowerAttackSpeedMultiply

    // RerollTowerAttackRangePlus

    upgrade_candidate_table
}

#[derive(Debug, Clone, Copy)]
enum UpgradeCandidate {
    GoldEarnPlus,
    RankAttackDamagePlus,
    RankAttackDamageMultiply,
    RankAttackSpeedPlus,
    RankAttackSpeedMultiply,
    RankAttackRangePlus,
    SuitAttackDamagePlus,
    SuitAttackDamageMultiply,
    SuitAttackSpeedPlus,
    SuitAttackSpeedMultiply,
    SuitAttackRangePlus,
    HandAttackDamagePlus,
    HandAttackDamageMultiply,
    HandAttackSpeedPlus,
    HandAttackSpeedMultiply,
    HandAttackRangePlus,
    ShopSlotExpansion,
    QuestSlotExpansion,
    QuestBoardExpansion,
    RerollCountPlus,
    LowCardTowerDamagePlus,
    LowCardTowerDamageMultiply,
    LowCardTowerAttackSpeedPlus,
    LowCardTowerAttackSpeedMultiply,
    LowCardTowerAttackRangePlus,
    ShopItemPriceMinus,
    ShopRefreshPlus,
    QuestBoardRefreshPlus,
    NoRerollTowerAttackDamagePlus,
    NoRerollTowerAttackDamageMultiply,
    NoRerollTowerAttackSpeedPlus,
    NoRerollTowerAttackSpeedMultiply,
    NoRerollTowerAttackRangePlus,
    EvenOddTowerAttackDamagePlus,
    EvenOddTowerAttackDamageMultiply,
    EvenOddTowerAttackSpeedPlus,
    EvenOddTowerAttackSpeedMultiply,
    EvenOddTowerAttackRangePlus,
    FaceNumberCardTowerAttackDamagePlus,
    FaceNumberCardTowerAttackDamageMultiply,
    FaceNumberCardTowerAttackSpeedPlus,
    FaceNumberCardTowerAttackSpeedMultiply,
    FaceNumberCardTowerAttackRangePlus,
    ShortenStraightFlushTo4Cards,
    SkipRankForStraight,
    TreatSuitsAsSame,
    RerollTowerAttackDamagePlus,
    RerollTowerAttackDamageMultiply,
    RerollTowerAttackSpeedPlus,
    RerollTowerAttackSpeedMultiply,
    RerollTowerAttackRangePlus,
}

fn get_tower_kind_with_weight(weights: &[f32; 10]) -> TowerKind {
    const TOWER_KINDS: [TowerKind; 10] = [
        TowerKind::High,
        TowerKind::OnePair,
        TowerKind::TwoPair,
        TowerKind::ThreeOfAKind,
        TowerKind::Straight,
        TowerKind::Flush,
        TowerKind::FullHouse,
        TowerKind::FourOfAKind,
        TowerKind::StraightFlush,
        TowerKind::RoyalFlush,
    ];

    TOWER_KINDS
        .iter()
        .zip(weights)
        .collect::<Vec<_>>()
        .choose_weighted(&mut thread_rng(), |x| x.1)
        .unwrap()
        .0
        .clone()
}
