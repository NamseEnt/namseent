use super::{
    MAX_GOLD_EARN_PLUS, MAX_QUEST_BOARD_REFRESH_CHANCE_PLUS, MAX_QUEST_BOARD_SLOT_EXPAND,
    MAX_QUEST_SLOT_EXPAND, MAX_REROLL_CHANCE_PLUS, MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE,
    MAX_SHOP_REFRESH_CHANCE_PLUS, MAX_SHOP_SLOT_EXPAND, Upgrade, UpgradeKind,
};
use crate::{
    card::{REVERSED_RANKS, SUITS},
    game_state::{GameState, level_rarity_weight::RarityGenerationOption, tower::TowerKind},
    rarity::Rarity,
};
use rand::{Rng, seq::SliceRandom, thread_rng};

//TODO: Call this function on clear boss stage
pub fn generate_upgrades_for_boss_reward(game_state: &GameState, amount: usize) -> Vec<Upgrade> {
    let rarities =
        (0..amount).map(|_| game_state.generate_rarity(RarityGenerationOption { no_common: true }));

    rarities
        .map(|rarity| generate_upgrade(game_state, rarity))
        .collect()
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
                Rarity::Common => 10.0..100.0,
                Rarity::Rare => 50.0..750.0,
                Rarity::Epic => 500.0..1500.0,
                Rarity::Legendary => 1250.0..2500.0,
            });
            UpgradeKind::RankAttackDamagePlus { rank, damage_plus }
        }
        UpgradeCandidate::RankAttackDamageMultiply => {
            let rank = *REVERSED_RANKS.choose(&mut thread_rng()).unwrap();
            let damage_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.2..1.5,
                Rarity::Rare => 1.3..1.75,
                Rarity::Epic => 1.5..2.5,
                Rarity::Legendary => 2.0..3.5,
            });
            UpgradeKind::RankAttackDamageMultiply {
                rank,
                damage_multiplier,
            }
        }
        UpgradeCandidate::RankAttackSpeedPlus => {
            let rank = *REVERSED_RANKS.choose(&mut thread_rng()).unwrap();
            let speed_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.2..0.4,
                Rarity::Rare => 0.2..0.6,
                Rarity::Epic => 0.4..1.0,
                Rarity::Legendary => 0.5..1.5,
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
                Rarity::Common => 1.5..2.5,
                Rarity::Rare => 2.0..4.0,
                Rarity::Epic => 3.0..5.0,
                Rarity::Legendary => 3.0..6.0,
            });
            UpgradeKind::RankAttackRangePlus { rank, range_plus }
        }
        UpgradeCandidate::SuitAttackDamagePlus => {
            let suit = *SUITS.choose(&mut thread_rng()).unwrap();
            let damage_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 10.0..50.0,
                Rarity::Rare => 50.0..250.0,
                Rarity::Epic => 250.0..1000.0,
                Rarity::Legendary => 1000.0..2500.0,
            });
            UpgradeKind::SuitAttackDamagePlus { suit, damage_plus }
        }
        UpgradeCandidate::SuitAttackDamageMultiply => {
            let suit = *SUITS.choose(&mut thread_rng()).unwrap();
            let damage_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.25,
                Rarity::Rare => 1.15..1.5,
                Rarity::Epic => 1.25..1.75,
                Rarity::Legendary => 1.5..3.5,
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
                Rarity::Common => 1.5..2.5,
                Rarity::Rare => 2.0..4.0,
                Rarity::Epic => 3.0..5.0,
                Rarity::Legendary => 3.0..6.0,
            });
            UpgradeKind::SuitAttackRangePlus { suit, range_plus }
        }
        UpgradeCandidate::HandAttackDamagePlus => {
            let tower_kind =
                get_tower_kind_with_weight(&[11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 6.0, 6.0, 3.0, 2.0]);
            let damage_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 10.0..100.0,
                Rarity::Rare => 100.0..500.0,
                Rarity::Epic => 500.0..2000.0,
                Rarity::Legendary => 2000.0..5000.0,
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
                Rarity::Common => 1.2..1.5,
                Rarity::Rare => 1.3..1.75,
                Rarity::Epic => 1.5..2.5,
                Rarity::Legendary => 2.0..4.0,
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
                Rarity::Rare => 0.2..0.6,
                Rarity::Epic => 0.4..1.0,
                Rarity::Legendary => 0.5..1.5,
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
                Rarity::Common => 1.2..1.4,
                Rarity::Rare => 1.2..1.6,
                Rarity::Epic => 1.4..2.0,
                Rarity::Legendary => 1.5..2.0,
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
                Rarity::Common => 1.5..2.5,
                Rarity::Rare => 2.0..5.0,
                Rarity::Epic => 4.0..8.0,
                Rarity::Legendary => 6.0..10.0,
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
                Rarity::Common => 10.0..100.0,
                Rarity::Rare => 100.0..500.0,
                Rarity::Epic => 500.0..2000.0,
                Rarity::Legendary => 2000.0..5000.0,
            });
            UpgradeKind::LowCardTowerDamagePlus { damage_plus }
        }
        UpgradeCandidate::LowCardTowerDamageMultiply => {
            let damage_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.2..1.5,
                Rarity::Rare => 1.3..1.75,
                Rarity::Epic => 1.5..2.5,
                Rarity::Legendary => 2.0..4.0,
            });
            UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier }
        }
        UpgradeCandidate::LowCardTowerAttackSpeedPlus => {
            let speed_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.2..0.4,
                Rarity::Rare => 0.2..0.6,
                Rarity::Epic => 0.4..1.0,
                Rarity::Legendary => 0.5..1.5,
            });
            UpgradeKind::LowCardTowerAttackSpeedPlus { speed_plus }
        }
        UpgradeCandidate::LowCardTowerAttackSpeedMultiply => {
            let speed_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.2..1.4,
                Rarity::Rare => 1.2..1.6,
                Rarity::Epic => 1.4..2.0,
                Rarity::Legendary => 1.5..2.0,
            });
            UpgradeKind::LowCardTowerAttackSpeedMultiply { speed_multiplier }
        }
        UpgradeCandidate::LowCardTowerAttackRangePlus => {
            let range_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.5..2.5,
                Rarity::Rare => 2.0..5.0,
                Rarity::Epic => 4.0..8.0,
                Rarity::Legendary => 6.0..10.0,
            });
            UpgradeKind::LowCardTowerAttackRangePlus { range_plus }
        }
        UpgradeCandidate::ShopItemPriceMinus => UpgradeKind::ShopItemPriceMinus,
        UpgradeCandidate::ShopRefreshPlus => UpgradeKind::ShopRefreshPlus,
        UpgradeCandidate::QuestBoardRefreshPlus => UpgradeKind::QuestBoardRefreshPlus,
        UpgradeCandidate::NoRerollTowerAttackDamagePlus => {
            let damage_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 10.0..100.0,
                Rarity::Rare => 100.0..500.0,
                Rarity::Epic => 500.0..2000.0,
                Rarity::Legendary => 2000.0..5000.0,
            });
            UpgradeKind::NoRerollTowerAttackDamagePlus { damage_plus }
        }
        UpgradeCandidate::NoRerollTowerAttackDamageMultiply => {
            let damage_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.2..1.5,
                Rarity::Rare => 1.3..1.75,
                Rarity::Epic => 1.5..2.5,
                Rarity::Legendary => 2.0..4.0,
            });
            UpgradeKind::NoRerollTowerAttackDamageMultiply { damage_multiplier }
        }
        UpgradeCandidate::NoRerollTowerAttackSpeedPlus => {
            let speed_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.2..0.4,
                Rarity::Rare => 0.2..0.6,
                Rarity::Epic => 0.4..1.0,
                Rarity::Legendary => 0.5..1.5,
            });
            UpgradeKind::NoRerollTowerAttackSpeedPlus { speed_plus }
        }
        UpgradeCandidate::NoRerollTowerAttackSpeedMultiply => {
            let speed_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.2..1.4,
                Rarity::Rare => 1.2..1.6,
                Rarity::Epic => 1.4..2.0,
                Rarity::Legendary => 1.5..2.0,
            });
            UpgradeKind::NoRerollTowerAttackSpeedMultiply { speed_multiplier }
        }
        UpgradeCandidate::NoRerollTowerAttackRangePlus => {
            let range_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.5..2.5,
                Rarity::Rare => 2.0..5.0,
                Rarity::Epic => 4.0..8.0,
                Rarity::Legendary => 6.0..10.0,
            });
            UpgradeKind::NoRerollTowerAttackRangePlus { range_plus }
        }
        UpgradeCandidate::EvenOddTowerAttackDamagePlus => {
            let even = thread_rng().gen_bool(0.5);
            let damage_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 5.0..25.0,
                Rarity::Rare => 25.0..150.0,
                Rarity::Epic => 100.0..500.0,
                Rarity::Legendary => 250.0..1500.0,
            });
            UpgradeKind::EvenOddTowerAttackDamagePlus { even, damage_plus }
        }
        UpgradeCandidate::EvenOddTowerAttackDamageMultiply => {
            let even = thread_rng().gen_bool(0.5);
            let damage_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.4,
                Rarity::Epic => 1.4..1.5,
                Rarity::Legendary => 1.5..1.6,
            });
            UpgradeKind::EvenOddTowerAttackDamageMultiply {
                even,
                damage_multiplier,
            }
        }
        UpgradeCandidate::EvenOddTowerAttackSpeedPlus => {
            let even = thread_rng().gen_bool(0.5);
            let speed_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.1..0.15,
                Rarity::Rare => 0.15..0.2,
                Rarity::Epic => 0.2..0.25,
                Rarity::Legendary => 0.25..0.3,
            });
            UpgradeKind::EvenOddTowerAttackSpeedPlus { even, speed_plus }
        }
        UpgradeCandidate::EvenOddTowerAttackSpeedMultiply => {
            let even = thread_rng().gen_bool(0.5);
            let speed_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.15,
                Rarity::Rare => 1.15..1.2,
                Rarity::Epic => 1.2..1.25,
                Rarity::Legendary => 1.25..1.3,
            });
            UpgradeKind::EvenOddTowerAttackSpeedMultiply {
                even,
                speed_multiplier,
            }
        }
        UpgradeCandidate::EvenOddTowerAttackRangePlus => {
            let even = thread_rng().gen_bool(0.5);
            let range_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.5..1.5,
                Rarity::Rare => 1.0..2.0,
                Rarity::Epic => 1.5..2.5,
                Rarity::Legendary => 1.5..3.0,
            });
            UpgradeKind::EvenOddTowerAttackRangePlus { even, range_plus }
        }
        UpgradeCandidate::FaceNumberCardTowerAttackDamagePlus => {
            let face = thread_rng().gen_bool(0.5);
            let damage_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 5.0..25.0,
                Rarity::Rare => 25.0..150.0,
                Rarity::Epic => 100.0..500.0,
                Rarity::Legendary => 250.0..1500.0,
            });
            UpgradeKind::FaceNumberCardTowerAttackDamagePlus { face, damage_plus }
        }
        UpgradeCandidate::FaceNumberCardTowerAttackDamageMultiply => {
            let face = thread_rng().gen_bool(0.5);
            let damage_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.4,
                Rarity::Epic => 1.4..1.5,
                Rarity::Legendary => 1.5..1.6,
            });
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply {
                face,
                damage_multiplier,
            }
        }
        UpgradeCandidate::FaceNumberCardTowerAttackSpeedPlus => {
            let face = thread_rng().gen_bool(0.5);
            let speed_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.1..0.15,
                Rarity::Rare => 0.15..0.2,
                Rarity::Epic => 0.2..0.25,
                Rarity::Legendary => 0.25..0.3,
            });
            UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { face, speed_plus }
        }
        UpgradeCandidate::FaceNumberCardTowerAttackSpeedMultiply => {
            let face = thread_rng().gen_bool(0.5);
            let speed_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.15,
                Rarity::Rare => 1.15..1.2,
                Rarity::Epic => 1.2..1.25,
                Rarity::Legendary => 1.25..1.3,
            });
            UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply {
                face,
                speed_multiplier,
            }
        }
        UpgradeCandidate::FaceNumberCardTowerAttackRangePlus => {
            let face = thread_rng().gen_bool(0.5);
            let range_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.5..1.5,
                Rarity::Rare => 1.0..2.0,
                Rarity::Epic => 1.5..2.5,
                Rarity::Legendary => 1.5..3.0,
            });
            UpgradeKind::FaceNumberCardTowerAttackRangePlus { face, range_plus }
        }
        UpgradeCandidate::ShortenStraightFlushTo4Cards => UpgradeKind::ShortenStraightFlushTo4Cards,
        UpgradeCandidate::SkipRankForStraight => UpgradeKind::SkipRankForStraight,
        UpgradeCandidate::TreatSuitsAsSame => UpgradeKind::TreatSuitsAsSame,
        UpgradeCandidate::RerollTowerAttackDamagePlus => {
            let damage_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 5.0..15.0,
                Rarity::Rare => 10.0..100.0,
                Rarity::Epic => 75.0..250.0,
                Rarity::Legendary => 200.0..1000.0,
            });
            UpgradeKind::RerollTowerAttackDamagePlus { damage_plus }
        }
        UpgradeCandidate::RerollTowerAttackDamageMultiply => {
            let damage_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.15,
                Rarity::Rare => 1.15..1.25,
                Rarity::Epic => 1.25..1.35,
                Rarity::Legendary => 1.35..1.5,
            });
            UpgradeKind::RerollTowerAttackDamageMultiply { damage_multiplier }
        }
        UpgradeCandidate::RerollTowerAttackSpeedPlus => {
            let speed_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.1..0.15,
                Rarity::Rare => 0.15..0.2,
                Rarity::Epic => 0.2..0.25,
                Rarity::Legendary => 0.25..0.3,
            });
            UpgradeKind::RerollTowerAttackSpeedPlus { speed_plus }
        }
        UpgradeCandidate::RerollTowerAttackSpeedMultiply => {
            let speed_multiplier = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.15,
                Rarity::Rare => 1.15..1.2,
                Rarity::Epic => 1.2..1.25,
                Rarity::Legendary => 1.25..1.3,
            });
            UpgradeKind::RerollTowerAttackSpeedMultiply { speed_multiplier }
        }
        UpgradeCandidate::RerollTowerAttackRangePlus => {
            let range_plus = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.5..1.5,
                Rarity::Rare => 1.0..2.0,
                Rarity::Epic => 1.5..2.5,
                Rarity::Legendary => 1.5..3.0,
            });
            UpgradeKind::RerollTowerAttackRangePlus { range_plus }
        }
    };

    Upgrade { kind, rarity }
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
        MAX_GOLD_EARN_PLUS,
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
        game_state.upgrade_state.shop_slot_expand,
        MAX_SHOP_SLOT_EXPAND,
        10,
        50,
        50,
        100,
    );

    // QuestSlotExpansion
    candidate_table_push(
        UpgradeCandidate::QuestSlotExpansion,
        game_state.upgrade_state.quest_slot_expand,
        MAX_QUEST_SLOT_EXPAND,
        10,
        50,
        50,
        100,
    );

    // QuestBoardExpansion
    candidate_table_push(
        UpgradeCandidate::QuestBoardExpansion,
        game_state.upgrade_state.quest_board_slot_expand,
        MAX_QUEST_BOARD_SLOT_EXPAND,
        10,
        50,
        50,
        100,
    );

    // RerollCountPlus
    candidate_table_push(
        UpgradeCandidate::RerollCountPlus,
        game_state.upgrade_state.reroll_chance_plus,
        MAX_REROLL_CHANCE_PLUS,
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
        10,
        10,
        10,
    );

    // ShopRefreshPlus
    candidate_table_push(
        UpgradeCandidate::ShopRefreshPlus,
        game_state.upgrade_state.shop_refresh_chance_plus,
        MAX_SHOP_REFRESH_CHANCE_PLUS,
        10,
        50,
        50,
        10,
    );

    // QuestBoardRefreshPlus
    candidate_table_push(
        UpgradeCandidate::QuestBoardRefreshPlus,
        game_state.upgrade_state.quest_board_refresh_chance_plus,
        MAX_QUEST_BOARD_REFRESH_CHANCE_PLUS,
        10,
        50,
        50,
        10,
    );

    // NoRerollTowerAttackDamagePlus
    candidate_table_push(
        UpgradeCandidate::NoRerollTowerAttackDamagePlus,
        usize::MIN,
        usize::MAX,
        40,
        50,
        100,
        100,
    );

    // NoRerollTowerAttackDamageMultiply
    candidate_table_push(
        UpgradeCandidate::NoRerollTowerAttackDamageMultiply,
        usize::MIN,
        usize::MAX,
        20,
        25,
        50,
        50,
    );

    // NoRerollTowerAttackSpeedPlus
    candidate_table_push(
        UpgradeCandidate::NoRerollTowerAttackSpeedPlus,
        usize::MIN,
        usize::MAX,
        30,
        40,
        80,
        80,
    );

    // NoRerollTowerAttackSpeedMultiply
    candidate_table_push(
        UpgradeCandidate::NoRerollTowerAttackSpeedMultiply,
        usize::MIN,
        usize::MAX,
        15,
        20,
        40,
        40,
    );

    // NoRerollTowerAttackRangePlus
    candidate_table_push(
        UpgradeCandidate::NoRerollTowerAttackRangePlus,
        usize::MIN,
        usize::MAX,
        15,
        25,
        30,
        30,
    );

    // EvenOddTowerAttackDamagePlus
    candidate_table_push(
        UpgradeCandidate::EvenOddTowerAttackDamagePlus,
        usize::MIN,
        usize::MAX,
        30,
        40,
        50,
        100,
    );

    // EvenOddTowerAttackDamageMultiply
    candidate_table_push(
        UpgradeCandidate::EvenOddTowerAttackDamageMultiply,
        usize::MIN,
        usize::MAX,
        15,
        20,
        25,
        50,
    );

    // EvenOddTowerAttackSpeedPlus
    candidate_table_push(
        UpgradeCandidate::EvenOddTowerAttackSpeedPlus,
        usize::MIN,
        usize::MAX,
        20,
        30,
        40,
        80,
    );

    // EvenOddTowerAttackSpeedMultiply
    candidate_table_push(
        UpgradeCandidate::EvenOddTowerAttackSpeedMultiply,
        usize::MIN,
        usize::MAX,
        10,
        15,
        20,
        40,
    );

    // EvenOddTowerAttackRangePlus
    candidate_table_push(
        UpgradeCandidate::EvenOddTowerAttackRangePlus,
        usize::MIN,
        usize::MAX,
        5,
        10,
        15,
        25,
    );

    // FaceNumberCardTowerAttackDamagePlus
    candidate_table_push(
        UpgradeCandidate::FaceNumberCardTowerAttackDamagePlus,
        usize::MIN,
        usize::MAX,
        30,
        40,
        50,
        100,
    );

    // FaceNumberCardTowerAttackDamageMultiply
    candidate_table_push(
        UpgradeCandidate::FaceNumberCardTowerAttackDamageMultiply,
        usize::MIN,
        usize::MAX,
        15,
        20,
        25,
        50,
    );

    // FaceNumberCardTowerAttackSpeedPlus
    candidate_table_push(
        UpgradeCandidate::FaceNumberCardTowerAttackSpeedPlus,
        usize::MIN,
        usize::MAX,
        20,
        30,
        40,
        80,
    );

    // FaceNumberCardTowerAttackSpeedMultiply
    candidate_table_push(
        UpgradeCandidate::FaceNumberCardTowerAttackSpeedMultiply,
        usize::MIN,
        usize::MAX,
        10,
        15,
        20,
        40,
    );

    // FaceNumberCardTowerAttackRangePlus
    candidate_table_push(
        UpgradeCandidate::FaceNumberCardTowerAttackRangePlus,
        usize::MIN,
        usize::MAX,
        5,
        10,
        15,
        20,
    );

    // ShortenStraightFlushTo4Cards
    candidate_table_push(
        UpgradeCandidate::ShortenStraightFlushTo4Cards,
        game_state.upgrade_state.shorten_straight_flush_to_4_cards as usize,
        1,
        5,
        10,
        20,
        25,
    );

    // SkipRankForStraight
    candidate_table_push(
        UpgradeCandidate::SkipRankForStraight,
        game_state.upgrade_state.skip_rank_for_straight as usize,
        1,
        5,
        10,
        20,
        25,
    );

    // TreatSuitsAsSame
    candidate_table_push(
        UpgradeCandidate::TreatSuitsAsSame,
        game_state.upgrade_state.treat_suits_as_same as usize,
        1,
        5,
        10,
        20,
        25,
    );

    // RerollTowerAttackDamagePlus
    candidate_table_push(
        UpgradeCandidate::RerollTowerAttackDamagePlus,
        usize::MIN,
        usize::MAX,
        30,
        40,
        50,
        100,
    );

    // RerollTowerAttackDamageMultiply
    candidate_table_push(
        UpgradeCandidate::RerollTowerAttackDamageMultiply,
        usize::MIN,
        usize::MAX,
        15,
        20,
        25,
        50,
    );

    // RerollTowerAttackSpeedPlus
    candidate_table_push(
        UpgradeCandidate::RerollTowerAttackSpeedPlus,
        usize::MIN,
        usize::MAX,
        20,
        30,
        40,
        80,
    );

    // RerollTowerAttackSpeedMultiply
    candidate_table_push(
        UpgradeCandidate::RerollTowerAttackSpeedMultiply,
        usize::MIN,
        usize::MAX,
        10,
        15,
        20,
        40,
    );

    // RerollTowerAttackRangePlus
    candidate_table_push(
        UpgradeCandidate::RerollTowerAttackRangePlus,
        usize::MIN,
        usize::MAX,
        5,
        10,
        15,
        20,
    );

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

    *TOWER_KINDS
        .iter()
        .zip(weights)
        .collect::<Vec<_>>()
        .choose_weighted(&mut thread_rng(), |x| x.1)
        .unwrap()
        .0
}
