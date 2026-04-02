use super::*;
use crate::game_state::{GameState, tower::TowerKind};
use rand::{Rng, seq::SliceRandom, thread_rng};

type KindGen = fn() -> UpgradeKind;

pub struct CandidateRow {
    pub weight: f32,
    pub kind_gen: KindGen,
}

pub fn generate_tower_damage_upgrade_candidate_table(_game_state: &GameState) -> Vec<CandidateRow> {
    vec![
        CandidateRow {
            weight: 13.0,
            kind_gen: || UpgradeKind::CainSword {
                damage_multiplier: thread_rng().gen_range(1.15..1.5),
            },
        },
        CandidateRow {
            weight: 13.0,
            kind_gen: || UpgradeKind::LongSword {
                damage_multiplier: thread_rng().gen_range(1.15..1.5),
            },
        },
        CandidateRow {
            weight: 13.0,
            kind_gen: || UpgradeKind::Mace {
                damage_multiplier: thread_rng().gen_range(1.15..1.5),
            },
        },
        CandidateRow {
            weight: 13.0,
            kind_gen: || UpgradeKind::ClubSword {
                damage_multiplier: thread_rng().gen_range(1.15..1.5),
            },
        },
        CandidateRow {
            weight: 50.0,
            kind_gen: || UpgradeKind::Spoon {
                damage_multiplier: thread_rng().gen_range(1.3..1.75),
            },
        },
        CandidateRow {
            weight: 20.0,
            kind_gen: || UpgradeKind::SingleChopstick {
                damage_multiplier: thread_rng().gen_range(1.2..1.4),
            },
        },
        CandidateRow {
            weight: 20.0,
            kind_gen: || UpgradeKind::PairChopsticks {
                damage_multiplier: thread_rng().gen_range(1.2..1.4),
            },
        },
        CandidateRow {
            weight: 20.0,
            kind_gen: || UpgradeKind::FountainPen {
                damage_multiplier: thread_rng().gen_range(1.2..1.4),
            },
        },
        CandidateRow {
            weight: 20.0,
            kind_gen: || UpgradeKind::Brush {
                damage_multiplier: thread_rng().gen_range(1.2..1.4),
            },
        },
        CandidateRow {
            weight: 20.0,
            kind_gen: || UpgradeKind::BrokenPottery {
                damage_multiplier: thread_rng().gen_range(1.15..1.25),
            },
        },
    ]
}

pub fn generate_treasure_upgrade_candidate_table(game_state: &GameState) -> Vec<CandidateRow> {
    let upgrade_state = &game_state.upgrade_state;

    let mut rows = Vec::with_capacity(16);
    let mut push_row = |kind_gen: KindGen, current_and_max: Option<(usize, usize)>, weight: f32| {
        let actual_weight = if let Some((current, max)) = current_and_max {
            if current >= max { 0.0 } else { weight }
        } else {
            weight
        };
        rows.push(CandidateRow {
            weight: actual_weight,
            kind_gen,
        });
    };

    push_row(
        || UpgradeKind::Magnet,
        Some((upgrade_state.gold_earn_plus, MAX_GOLD_EARN_PLUS)),
        50.0,
    );
    push_row(
        || UpgradeKind::Backpack,
        Some((upgrade_state.shop_slot_expand, MAX_SHOP_SLOT_EXPAND)),
        50.0,
    );
    push_row(
        || UpgradeKind::DiceBundle,
        Some((upgrade_state.dice_chance_plus, MAX_DICE_CHANCE_PLUS)),
        10.0,
    );
    push_row(
        || UpgradeKind::EnergyDrink,
        Some((
            upgrade_state.shop_item_price_minus,
            MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE,
        )),
        10.0,
    );
    push_row(
        || UpgradeKind::PerfectPottery {
            damage_multiplier: thread_rng().gen_range(1.3..1.75),
        },
        None,
        25.0,
    );
    push_row(
        || UpgradeKind::BrokenPottery {
            damage_multiplier: thread_rng().gen_range(1.15..1.25),
        },
        None,
        20.0,
    );
    push_row(
        || UpgradeKind::FourLeafClover,
        Some((upgrade_state.shorten_straight_flush_to_4_cards as usize, 1)),
        10.0,
    );
    push_row(
        || UpgradeKind::Rabbit,
        Some((upgrade_state.skip_rank_for_straight as usize, 1)),
        10.0,
    );
    push_row(
        || UpgradeKind::BlackWhite,
        Some((upgrade_state.treat_suits_as_same as usize, 1)),
        10.0,
    );

    rows
}

#[allow(dead_code)]
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
