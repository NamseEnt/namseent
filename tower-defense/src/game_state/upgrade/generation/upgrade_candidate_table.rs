use super::*;
use crate::game_state::upgrade::MAX_REMOVE_NUMBER_RANKS;
use crate::game_state::{GameState, tower::TowerKind};
use rand::{Rng, seq::SliceRandom, thread_rng};

type KindGen = Box<dyn Fn() -> UpgradeKind + Send + Sync>;

pub struct CandidateRow {
    pub weight: f32,
    pub kind_gen: KindGen,
}

pub fn generate_tower_damage_upgrade_candidate_table(game_state: &GameState) -> Vec<CandidateRow> {
    game_state
        .config
        .upgrades
        .tower_damage_upgrades
        .iter()
        .map(|(name, entry)| CandidateRow {
            weight: entry.weight,
            kind_gen: make_tower_damage_upgrade_kind_gen(name, entry.damage_multiplier_range),
        })
        .collect()
}

fn make_tower_damage_upgrade_kind_gen(
    name: &str,
    range: Option<(f32, f32)>,
) -> KindGen {
    let range = range.unwrap_or((1.0, 1.0));
    match name {
        "CainSword" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::CainSword {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        "LongSword" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::LongSword {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        "Mace" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::Mace {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        "ClubSword" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::ClubSword {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        "Spoon" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::Spoon {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        "SingleChopstick" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::SingleChopstick {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        "PairChopsticks" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::PairChopsticks {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        "FountainPen" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::FountainPen {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        "Brush" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::Brush {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        "BrokenPottery" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::BrokenPottery {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        other => panic!("Unknown tower damage upgrade kind: {other}"),
    }
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

    for (name, entry) in &game_state.config.upgrades.treasure_upgrades {
        let weight = entry.weight;
        let kind_gen = make_treasure_upgrade_kind_gen(name, entry.damage_multiplier_range);
        let current_and_max = match name.as_str() {
            "Magnet" => Some((upgrade_state.gold_earn_plus, MAX_GOLD_EARN_PLUS)),
            "Backpack" => Some((upgrade_state.shop_slot_expand, MAX_SHOP_SLOT_EXPAND)),
            "DiceBundle" => Some((upgrade_state.dice_chance_plus, MAX_DICE_CHANCE_PLUS)),
            "EnergyDrink" => Some((
                upgrade_state.shop_item_price_minus,
                MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE,
            )),
            "FourLeafClover" => Some((upgrade_state.shorten_straight_flush_to_4_cards as usize, 1)),
            "Rabbit" => Some((upgrade_state.skip_rank_for_straight as usize, 1)),
            "BlackWhite" => Some((upgrade_state.treat_suits_as_same as usize, 1)),
            "Eraser" => Some((upgrade_state.removed_number_rank_count, MAX_REMOVE_NUMBER_RANKS)),
            _ => None,
        };
        push_row(kind_gen, current_and_max, weight);
    }

    rows
}

fn make_treasure_upgrade_kind_gen(
    name: &str,
    range: Option<(f32, f32)>,
) -> KindGen {
    let range = range.unwrap_or((1.0, 1.0));
    match name {
        "Magnet" => Box::new(|| UpgradeKind::Magnet),
        "Backpack" => Box::new(|| UpgradeKind::Backpack),
        "DiceBundle" => Box::new(|| UpgradeKind::DiceBundle),
        "EnergyDrink" => Box::new(|| UpgradeKind::EnergyDrink),
        "PerfectPottery" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::PerfectPottery {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        "BrokenPottery" => {
            let (min, max) = range;
            Box::new(move || UpgradeKind::BrokenPottery {
                damage_multiplier: thread_rng().gen_range(min..max),
            })
        }
        "FourLeafClover" => Box::new(|| UpgradeKind::FourLeafClover),
        "Rabbit" => Box::new(|| UpgradeKind::Rabbit),
        "BlackWhite" => Box::new(|| UpgradeKind::BlackWhite),
        "Eraser" => Box::new(|| UpgradeKind::Eraser),
        other => panic!("Unknown treasure upgrade kind: {other}"),
    }
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
