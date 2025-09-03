use super::*;
use crate::{
    card::{REVERSED_RANKS, SUITS},
    game_state::{GameState, tower::TowerKind},
    rarity::Rarity,
};
use rand::{Rng, seq::SliceRandom, thread_rng};

type KindGen = fn(rarity: Rarity) -> UpgradeKind;

pub struct CandidateRow {
    pub weight: f32,
    pub kind_gen: KindGen,
}
pub fn generate_upgrade_candidate_table(
    game_state: &GameState,
    rarity: Rarity,
) -> Vec<CandidateRow> {
    let upgrade_state = &game_state.upgrade_state;

    candidate_table![
        rarity,
        (
            |_rarity| UpgradeKind::GoldEarnPlus,
            Some((upgrade_state.gold_earn_plus, MAX_GOLD_EARN_PLUS)),
            (10, 50, 50, 100),
        ),
        (
            |rarity| UpgradeKind::RankAttackDamagePlus {
                rank: *REVERSED_RANKS.choose(&mut thread_rng()).unwrap(),
                damage_plus: rarity_gen(
                    rarity,
                    (10.0..100.0, 50.0..750.0, 500.0..1500.0, 1250.0..2500.0),
                ),
            },
            None,
            (38, 75, 75, 75),
        ),
        (
            |rarity| UpgradeKind::RankAttackDamageMultiply {
                rank: *REVERSED_RANKS.choose(&mut thread_rng()).unwrap(),
                damage_multiplier: rarity_gen(rarity, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..3.5)),
            },
            None,
            (19, 38, 38, 38),
        ),
        (
            |rarity| UpgradeKind::RankAttackSpeedPlus {
                rank: *REVERSED_RANKS.choose(&mut thread_rng()).unwrap(),
                speed_plus: rarity_gen(rarity, (0.2..0.4, 0.2..0.6, 0.4..1.0, 0.5..1.5)),
            },
            None,
            (30, 60, 60, 60),
        ),
        (
            |rarity| UpgradeKind::RankAttackSpeedMultiply {
                rank: *REVERSED_RANKS.choose(&mut thread_rng()).unwrap(),
                speed_multiplier: rarity_gen(rarity, (1.1..1.2, 1.2..1.5, 1.5..1.75, 1.75..2.0)),
            },
            None,
            (15, 30, 30, 30),
        ),
        (
            |rarity| UpgradeKind::RankAttackRangePlus {
                rank: *REVERSED_RANKS.choose(&mut thread_rng()).unwrap(),
                range_plus: rarity_gen(rarity, (1.5..2.5, 2.0..4.0, 3.0..5.0, 3.0..6.0)),
            },
            None,
            (8, 15, 15, 15),
        ),
        (
            |rarity| UpgradeKind::SuitAttackDamagePlus {
                suit: *SUITS.choose(&mut thread_rng()).unwrap(),
                damage_plus: rarity_gen(
                    rarity,
                    (10.0..50.0, 50.0..250.0, 250.0..1000.0, 1000.0..2500.0),
                ),
            },
            None,
            (13, 25, 25, 25),
        ),
        (
            |rarity| UpgradeKind::SuitAttackDamageMultiply {
                suit: *SUITS.choose(&mut thread_rng()).unwrap(),
                damage_multiplier: rarity_gen(rarity, (1.1..1.25, 1.15..1.5, 1.25..1.75, 1.5..3.5)),
            },
            None,
            (6, 13, 13, 13),
        ),
        (
            |rarity| UpgradeKind::SuitAttackSpeedPlus {
                suit: *SUITS.choose(&mut thread_rng()).unwrap(),
                speed_plus: rarity_gen(rarity, (0.1..0.25, 0.25..0.5, 0.5..0.75, 0.75..1.0)),
            },
            None,
            (10, 20, 20, 20),
        ),
        (
            |rarity| UpgradeKind::SuitAttackSpeedMultiply {
                suit: *SUITS.choose(&mut thread_rng()).unwrap(),
                speed_multiplier: rarity_gen(rarity, (1.1..1.2, 1.2..1.5, 1.5..1.75, 1.75..2.0)),
            },
            None,
            (5, 10, 10, 10),
        ),
        (
            |rarity| UpgradeKind::SuitAttackRangePlus {
                suit: *SUITS.choose(&mut thread_rng()).unwrap(),
                range_plus: rarity_gen(rarity, (1.5..2.5, 2.0..4.0, 3.0..5.0, 3.0..6.0)),
            },
            None,
            (3, 5, 5, 5),
        ),
        (
            |rarity| UpgradeKind::HandAttackDamagePlus {
                tower_kind: get_tower_kind_with_weight(&[
                    11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 6.0, 6.0, 3.0, 2.0,
                ]),
                damage_plus: rarity_gen(
                    rarity,
                    (10.0..100.0, 100.0..500.0, 500.0..2000.0, 2000.0..5000.0),
                ),
            },
            None,
            (50, 100, 100, 50),
        ),
        (
            |rarity| UpgradeKind::HandAttackDamageMultiply {
                tower_kind: get_tower_kind_with_weight(&[
                    11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 6.0, 6.0, 3.0, 2.0,
                ]),
                damage_multiplier: rarity_gen(rarity, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..4.0)),
            },
            None,
            (25, 50, 50, 25),
        ),
        (
            |rarity| UpgradeKind::HandAttackSpeedPlus {
                tower_kind: get_tower_kind_with_weight(&[
                    11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 6.0, 6.0, 3.0, 2.0,
                ]),
                speed_plus: rarity_gen(rarity, (0.2..0.4, 0.2..0.6, 0.4..1.0, 0.5..1.5)),
            },
            None,
            (40, 80, 80, 40),
        ),
        (
            |rarity| UpgradeKind::HandAttackSpeedMultiply {
                tower_kind: get_tower_kind_with_weight(&[
                    11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 6.0, 6.0, 3.0, 2.0,
                ]),
                speed_multiplier: rarity_gen(rarity, (1.2..1.4, 1.2..1.6, 1.4..2.0, 1.5..2.0)),
            },
            None,
            (20, 40, 40, 20),
        ),
        (
            |rarity| UpgradeKind::HandAttackRangePlus {
                tower_kind: get_tower_kind_with_weight(&[
                    11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 6.0, 6.0, 3.0, 2.0,
                ]),
                range_plus: rarity_gen(rarity, (1.5..2.5, 2.0..5.0, 4.0..8.0, 6.0..10.0)),
            },
            None,
            (10, 20, 20, 10),
        ),
        (
            |_rarity| UpgradeKind::ShopSlotExpansion,
            Some((upgrade_state.shop_slot_expand, MAX_SHOP_SLOT_EXPAND)),
            (10, 50, 50, 100),
        ),
        (
            |_rarity| UpgradeKind::QuestSlotExpansion,
            Some((upgrade_state.quest_slot_expand, MAX_QUEST_SLOT_EXPAND)),
            (10, 50, 50, 100),
        ),
        (
            |_rarity| UpgradeKind::QuestBoardExpansion,
            Some((
                upgrade_state.quest_board_slot_expand,
                MAX_QUEST_BOARD_SLOT_EXPAND,
            )),
            (10, 50, 50, 100),
        ),
        (
            |_rarity| UpgradeKind::RerollCountPlus,
            Some((upgrade_state.reroll_chance_plus, MAX_REROLL_CHANCE_PLUS)),
            (5, 10, 50, 100),
        ),
        (
            |rarity| UpgradeKind::LowCardTowerDamagePlus {
                damage_plus: rarity_gen(
                    rarity,
                    (10.0..100.0, 100.0..500.0, 500.0..2000.0, 2000.0..5000.0),
                ),
            },
            None,
            (50, 100, 100, 50),
        ),
        (
            |rarity| UpgradeKind::LowCardTowerDamageMultiply {
                damage_multiplier: rarity_gen(rarity, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..4.0)),
            },
            None,
            (25, 50, 50, 25),
        ),
        (
            |rarity| UpgradeKind::LowCardTowerAttackSpeedPlus {
                speed_plus: rarity_gen(rarity, (0.2..0.4, 0.2..0.6, 0.4..1.0, 0.5..1.5)),
            },
            None,
            (40, 80, 80, 40),
        ),
        (
            |rarity| UpgradeKind::LowCardTowerAttackSpeedMultiply {
                speed_multiplier: rarity_gen(rarity, (1.2..1.4, 1.2..1.6, 1.4..2.0, 1.5..2.0)),
            },
            None,
            (20, 40, 40, 20),
        ),
        (
            |rarity| UpgradeKind::LowCardTowerAttackRangePlus {
                range_plus: rarity_gen(rarity, (1.5..2.5, 2.0..5.0, 4.0..8.0, 6.0..10.0)),
            },
            None,
            (10, 20, 20, 10),
        ),
        (
            |_rarity| UpgradeKind::ShopItemPriceMinus,
            Some((
                upgrade_state.shop_item_price_minus,
                MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE,
            )),
            (10, 10, 10, 10),
        ),
        (
            |_rarity| UpgradeKind::ShopRefreshPlus,
            Some((
                upgrade_state.shop_refresh_chance_plus,
                MAX_SHOP_REFRESH_CHANCE_PLUS,
            )),
            (10, 50, 50, 10),
        ),
        (
            |_rarity| UpgradeKind::QuestBoardRefreshPlus,
            Some((
                upgrade_state.quest_board_refresh_chance_plus,
                MAX_QUEST_BOARD_REFRESH_CHANCE_PLUS,
            )),
            (10, 50, 50, 10),
        ),
        (
            |rarity| UpgradeKind::NoRerollTowerAttackDamagePlus {
                damage_plus: rarity_gen(
                    rarity,
                    (10.0..100.0, 100.0..500.0, 500.0..2000.0, 2000.0..5000.0),
                ),
            },
            None,
            (40, 50, 100, 100),
        ),
        (
            |rarity| UpgradeKind::NoRerollTowerAttackDamageMultiply {
                damage_multiplier: rarity_gen(rarity, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..4.0)),
            },
            None,
            (20, 25, 50, 50),
        ),
        (
            |rarity| UpgradeKind::NoRerollTowerAttackSpeedPlus {
                speed_plus: rarity_gen(rarity, (0.2..0.4, 0.2..0.6, 0.4..1.0, 0.5..1.5)),
            },
            None,
            (30, 40, 80, 80),
        ),
        (
            |rarity| UpgradeKind::NoRerollTowerAttackSpeedMultiply {
                speed_multiplier: rarity_gen(rarity, (1.2..1.4, 1.2..1.6, 1.4..2.0, 1.5..2.0)),
            },
            None,
            (15, 20, 40, 40),
        ),
        (
            |rarity| UpgradeKind::NoRerollTowerAttackRangePlus {
                range_plus: rarity_gen(rarity, (1.5..2.5, 2.0..5.0, 4.0..8.0, 6.0..10.0)),
            },
            None,
            (15, 25, 30, 30),
        ),
        (
            |rarity| UpgradeKind::EvenOddTowerAttackDamagePlus {
                even: thread_rng().gen_bool(0.5),
                damage_plus: rarity_gen(
                    rarity,
                    (5.0..25.0, 25.0..150.0, 100.0..500.0, 250.0..1500.0),
                ),
            },
            None,
            (30, 40, 50, 100),
        ),
        (
            |rarity| UpgradeKind::EvenOddTowerAttackDamageMultiply {
                even: thread_rng().gen_bool(0.5),
                damage_multiplier: rarity_gen(rarity, (1.1..1.2, 1.2..1.4, 1.4..1.5, 1.5..1.6)),
            },
            None,
            (15, 20, 25, 50),
        ),
        (
            |rarity| UpgradeKind::EvenOddTowerAttackSpeedPlus {
                even: thread_rng().gen_bool(0.5),
                speed_plus: rarity_gen(rarity, (0.1..0.15, 0.15..0.2, 0.2..0.25, 0.25..0.3)),
            },
            None,
            (20, 30, 40, 80),
        ),
        (
            |rarity| UpgradeKind::EvenOddTowerAttackSpeedMultiply {
                even: thread_rng().gen_bool(0.5),
                speed_multiplier: rarity_gen(rarity, (1.1..1.15, 1.15..1.2, 1.2..1.25, 1.25..1.3)),
            },
            None,
            (10, 15, 20, 40),
        ),
        (
            |rarity| UpgradeKind::EvenOddTowerAttackRangePlus {
                even: thread_rng().gen_bool(0.5),
                range_plus: rarity_gen(rarity, (0.5..1.5, 1.0..2.0, 1.5..2.5, 1.5..3.0)),
            },
            None,
            (5, 10, 15, 25),
        ),
        (
            |rarity| UpgradeKind::FaceNumberCardTowerAttackDamagePlus {
                face: thread_rng().gen_bool(0.5),
                damage_plus: rarity_gen(
                    rarity,
                    (5.0..25.0, 25.0..150.0, 100.0..500.0, 250.0..1500.0),
                ),
            },
            None,
            (30, 40, 50, 100),
        ),
        (
            |rarity| UpgradeKind::FaceNumberCardTowerAttackDamageMultiply {
                face: thread_rng().gen_bool(0.5),
                damage_multiplier: rarity_gen(rarity, (1.1..1.2, 1.2..1.4, 1.4..1.5, 1.5..1.6)),
            },
            None,
            (15, 20, 25, 50),
        ),
        (
            |rarity| UpgradeKind::FaceNumberCardTowerAttackSpeedPlus {
                face: thread_rng().gen_bool(0.5),
                speed_plus: rarity_gen(rarity, (0.1..0.15, 0.15..0.2, 0.2..0.25, 0.25..0.3)),
            },
            None,
            (20, 30, 40, 80),
        ),
        (
            |rarity| UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply {
                face: thread_rng().gen_bool(0.5),
                speed_multiplier: rarity_gen(rarity, (1.1..1.15, 1.15..1.2, 1.2..1.25, 1.25..1.3)),
            },
            None,
            (10, 15, 20, 40),
        ),
        (
            |rarity| UpgradeKind::FaceNumberCardTowerAttackRangePlus {
                face: thread_rng().gen_bool(0.5),
                range_plus: rarity_gen(rarity, (0.5..1.5, 1.0..2.0, 1.5..2.5, 1.5..3.0)),
            },
            None,
            (5, 10, 15, 20),
        ),
        (
            |_rarity| UpgradeKind::ShortenStraightFlushTo4Cards,
            Some((upgrade_state.shorten_straight_flush_to_4_cards as usize, 1)),
            (5, 10, 20, 25),
        ),
        (
            |_rarity| UpgradeKind::SkipRankForStraight,
            Some((upgrade_state.skip_rank_for_straight as usize, 1)),
            (5, 10, 20, 25),
        ),
        (
            |_rarity| UpgradeKind::TreatSuitsAsSame,
            Some((upgrade_state.treat_suits_as_same as usize, 1)),
            (5, 10, 20, 25),
        ),
        (
            |rarity| UpgradeKind::RerollTowerAttackDamagePlus {
                damage_plus: rarity_gen(
                    rarity,
                    (5.0..15.0, 10.0..100.0, 75.0..250.0, 200.0..1000.0)
                ),
            },
            None,
            (30, 40, 50, 100),
        ),
        (
            |rarity| UpgradeKind::RerollTowerAttackDamageMultiply {
                damage_multiplier: rarity_gen(
                    rarity,
                    (1.1..1.15, 1.15..1.25, 1.25..1.35, 1.35..1.5)
                ),
            },
            None,
            (15, 20, 25, 50),
        ),
        (
            |rarity| UpgradeKind::RerollTowerAttackSpeedPlus {
                speed_plus: rarity_gen(rarity, (0.1..0.15, 0.15..0.2, 0.2..0.25, 0.25..0.3)),
            },
            None,
            (20, 30, 40, 80),
        ),
        (
            |rarity| UpgradeKind::RerollTowerAttackSpeedMultiply {
                speed_multiplier: rarity_gen(rarity, (1.1..1.15, 1.15..1.2, 1.2..1.25, 1.25..1.3)),
            },
            None,
            (10, 15, 20, 40),
        ),
        (
            |rarity| UpgradeKind::RerollTowerAttackRangePlus {
                range_plus: rarity_gen(rarity, (0.5..1.5, 1.0..2.0, 1.5..2.5, 1.5..3.0)),
            },
            None,
            (5, 10, 15, 20),
        )
    ]
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

fn rarity_gen(
    rarity: Rarity,
    ranges: (
        std::ops::Range<f32>,
        std::ops::Range<f32>,
        std::ops::Range<f32>,
        std::ops::Range<f32>,
    ),
) -> f32 {
    thread_rng().gen_range(match rarity {
        Rarity::Common => ranges.0,
        Rarity::Rare => ranges.1,
        Rarity::Epic => ranges.2,
        Rarity::Legendary => ranges.3,
    })
}

macro_rules! candidate_table {
    ($rarity:expr, $(($kind_gen:expr, $current_and_max:expr, $weights:expr,)),*) => {
        {
            let mut upgrade_candidate_table = Vec::with_capacity(64);
            let mut candidate_table_push =
            |kind_gen: KindGen,
             current_and_max: Option<(usize, usize)>,
             weights: (usize, usize, usize, usize)| {
                let weight = {
                    if let Some((current, max)) = current_and_max
                        && current >= max
                    {
                        0.0
                    } else {
                        match $rarity {
                            Rarity::Common => weights.0 as f32,
                            Rarity::Rare => weights.1 as f32,
                            Rarity::Epic => weights.2 as f32,
                            Rarity::Legendary => weights.3 as f32,
                        }
                    }
                };
                upgrade_candidate_table.push(CandidateRow { weight, kind_gen })
            };
            $(
                candidate_table_push($kind_gen, $current_and_max, $weights);
            )*
            upgrade_candidate_table
        }
    };
}
use candidate_table;
