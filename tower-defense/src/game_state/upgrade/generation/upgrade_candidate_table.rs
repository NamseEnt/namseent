use super::*;
use crate::game_state::upgrade::{MAX_REMOVE_NUMBER_RANKS, UpgradeDiscriminants};
use crate::game_state::{GameState, tower::TowerKind};

type KindGen = Box<dyn Fn() -> Upgrade + Send + Sync>;

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
        .map(|upgrade| {
            let disc: UpgradeDiscriminants = upgrade.name.parse().unwrap_or_else(|_| {
                panic!("Unknown tower damage upgrade config name: {}", upgrade.name)
            });
            CandidateRow {
                weight: upgrade.entry.weight,
                kind_gen: make_tower_damage_upgrade_kind_gen(disc, upgrade.entry.damage_multiplier),
            }
        })
        .collect()
}

fn make_tower_damage_upgrade_kind_gen(
    disc: UpgradeDiscriminants,
    damage_multiplier: Option<f32>,
) -> KindGen {
    let damage_multiplier = damage_multiplier.unwrap_or(1.0);
    match disc {
        UpgradeDiscriminants::Staff => {
            Box::new(move || crate::game_state::upgrade::StaffUpgrade::into_upgrade(damage_multiplier))
        }
        UpgradeDiscriminants::LongSword => {
            Box::new(move || crate::game_state::upgrade::LongSwordUpgrade::into_upgrade(damage_multiplier))
        }
        UpgradeDiscriminants::Mace => {
            Box::new(move || crate::game_state::upgrade::MaceUpgrade::into_upgrade(damage_multiplier))
        }
        UpgradeDiscriminants::ClubSword => {
            Box::new(move || crate::game_state::upgrade::ClubSwordUpgrade::into_upgrade(damage_multiplier))
        }
        UpgradeDiscriminants::Tricycle => {
            Box::new(move || crate::game_state::upgrade::TricycleUpgrade::into_upgrade(damage_multiplier))
        }
        UpgradeDiscriminants::SingleChopstick => Box::new(move || {
            crate::game_state::upgrade::SingleChopstickUpgrade::into_upgrade(damage_multiplier)
        }),
        UpgradeDiscriminants::PairChopsticks => Box::new(move || {
            crate::game_state::upgrade::PairChopsticksUpgrade::into_upgrade(damage_multiplier)
        }),
        UpgradeDiscriminants::FountainPen => {
            Box::new(move || crate::game_state::upgrade::FountainPenUpgrade::into_upgrade(damage_multiplier))
        }
        UpgradeDiscriminants::Brush => {
            Box::new(move || crate::game_state::upgrade::BrushUpgrade::into_upgrade(damage_multiplier))
        }
        UpgradeDiscriminants::BrokenPottery => Box::new(move || {
            crate::game_state::upgrade::BrokenPotteryUpgrade::into_upgrade(damage_multiplier)
        }),
        UpgradeDiscriminants::PerfectPottery => Box::new(move || {
            crate::game_state::upgrade::PerfectPotteryUpgrade::into_upgrade(damage_multiplier)
        }),
        other => panic!("Invalid tower damage upgrade kind: {:?}", other),
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

    for upgrade in &game_state.config.upgrades.treasure_upgrades {
        let disc: UpgradeDiscriminants = upgrade
            .name
            .parse()
            .unwrap_or_else(|_| panic!("Unknown treasure upgrade config name: {}", upgrade.name));
        let weight = upgrade.entry.weight;
        let kind_gen =
            make_treasure_upgrade_kind_gen(disc, upgrade.entry.damage_multiplier, upgrade_state);
        let current_and_max = match disc {
            UpgradeDiscriminants::Cat => Some((upgrade_state.gold_earn_plus(), MAX_GOLD_EARN_PLUS)),
            UpgradeDiscriminants::Backpack => {
                Some((upgrade_state.shop_slot_expand(), MAX_SHOP_SLOT_EXPAND))
            }
            UpgradeDiscriminants::DiceBundle => {
                Some((upgrade_state.dice_chance_plus(), MAX_DICE_CHANCE_PLUS))
            }
            UpgradeDiscriminants::EnergyDrink => Some((
                upgrade_state.shop_item_price_minus(),
                MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE,
            )),
            UpgradeDiscriminants::FourLeafClover => Some((
                upgrade_state.shorten_straight_flush_to_4_cards() as usize,
                1,
            )),
            UpgradeDiscriminants::Rabbit => {
                Some((upgrade_state.skip_rank_for_straight() as usize, 1))
            }
            UpgradeDiscriminants::BlackWhite => {
                Some((upgrade_state.treat_suits_as_same() as usize, 1))
            }
            UpgradeDiscriminants::Eraser => Some((
                upgrade_state.removed_number_rank_count(),
                MAX_REMOVE_NUMBER_RANKS,
            )),
            _ => None,
        };
        push_row(kind_gen, current_and_max, weight);
    }

    rows
}

fn make_treasure_upgrade_kind_gen(
    disc: UpgradeDiscriminants,
    damage_multiplier: Option<f32>,
    upgrade_state: &UpgradeState,
) -> KindGen {
    let damage_multiplier = damage_multiplier.unwrap_or(1.0);
    let add = match disc {
        UpgradeDiscriminants::Cat => next_cat_add(upgrade_state.gold_earn_plus()),
        UpgradeDiscriminants::Backpack => 1,
        UpgradeDiscriminants::DiceBundle => 1,
        UpgradeDiscriminants::EnergyDrink => 5,
        UpgradeDiscriminants::Eraser => 1,
        _ => 0,
    };
    match disc {
        UpgradeDiscriminants::Cat => {
            Box::new(move || crate::game_state::upgrade::CatUpgrade::into_upgrade(add))
        }
        UpgradeDiscriminants::Backpack => {
            Box::new(move || crate::game_state::upgrade::BackpackUpgrade::into_upgrade(add))
        }
        UpgradeDiscriminants::DiceBundle => {
            Box::new(move || crate::game_state::upgrade::DiceBundleUpgrade::into_upgrade(add))
        }
        UpgradeDiscriminants::EnergyDrink => {
            Box::new(move || crate::game_state::upgrade::EnergyDrinkUpgrade::into_upgrade(5))
        }
        UpgradeDiscriminants::PerfectPottery => Box::new(move || {
            crate::game_state::upgrade::PerfectPotteryUpgrade::into_upgrade(damage_multiplier)
        }),
        UpgradeDiscriminants::BrokenPottery => Box::new(move || {
            crate::game_state::upgrade::BrokenPotteryUpgrade::into_upgrade(damage_multiplier)
        }),
        UpgradeDiscriminants::FourLeafClover => {
            Box::new(crate::game_state::upgrade::FourLeafCloverUpgrade::into_upgrade)
        }
        UpgradeDiscriminants::Rabbit => Box::new(crate::game_state::upgrade::RabbitUpgrade::into_upgrade),
        UpgradeDiscriminants::BlackWhite => {
            Box::new(crate::game_state::upgrade::BlackWhiteUpgrade::into_upgrade)
        }
        UpgradeDiscriminants::Eraser => {
            Box::new(move || crate::game_state::upgrade::EraserUpgrade::into_upgrade(add))
        }
        UpgradeDiscriminants::Trophy => Box::new(crate::game_state::upgrade::TrophyUpgrade::into_upgrade),
        UpgradeDiscriminants::Crock => Box::new(crate::game_state::upgrade::CrockUpgrade::into_upgrade),
        UpgradeDiscriminants::DemolitionHammer => Box::new(move || {
            crate::game_state::upgrade::DemolitionHammerUpgrade::into_upgrade(damage_multiplier)
        }),
        UpgradeDiscriminants::Metronome => {
            Box::new(crate::game_state::upgrade::MetronomeUpgrade::into_upgrade)
        }
        UpgradeDiscriminants::Tape => Box::new(|| crate::game_state::upgrade::TapeUpgrade::into_upgrade(0)),
        UpgradeDiscriminants::NameTag => {
            Box::new(move || crate::game_state::upgrade::NameTagUpgrade::into_upgrade(damage_multiplier))
        }
        UpgradeDiscriminants::ShoppingBag => {
            Box::new(move || crate::game_state::upgrade::ShoppingBagUpgrade::into_upgrade(damage_multiplier))
        }
        UpgradeDiscriminants::Resolution => {
            Box::new(move || crate::game_state::upgrade::ResolutionUpgrade::into_upgrade(damage_multiplier))
        }
        UpgradeDiscriminants::Mirror => Box::new(crate::game_state::upgrade::MirrorUpgrade::into_upgrade),
        UpgradeDiscriminants::IceCream => {
            Box::new(move || crate::game_state::upgrade::IceCreamUpgrade::into_upgrade(damage_multiplier, 5))
        }
        UpgradeDiscriminants::Spanner => Box::new(crate::game_state::upgrade::SpannerUpgrade::into_upgrade),
        UpgradeDiscriminants::Pea => Box::new(crate::game_state::upgrade::PeaUpgrade::into_upgrade),
        UpgradeDiscriminants::SlotMachine => {
            Box::new(|| crate::game_state::upgrade::SlotMachineUpgrade::into_upgrade(10))
        }
        UpgradeDiscriminants::PiggyBank => {
            Box::new(crate::game_state::upgrade::PiggyBankUpgrade::into_upgrade)
        }
        UpgradeDiscriminants::Camera => Box::new(crate::game_state::upgrade::CameraUpgrade::into_upgrade),
        UpgradeDiscriminants::GiftBox => Box::new(crate::game_state::upgrade::GiftBoxUpgrade::into_upgrade),
        UpgradeDiscriminants::Fang => Box::new(crate::game_state::upgrade::FangUpgrade::into_upgrade),
        UpgradeDiscriminants::Popcorn => Box::new(move || {
            crate::game_state::upgrade::PopcornUpgrade::into_upgrade(damage_multiplier, 5, 5)
        }),
        UpgradeDiscriminants::MembershipCard => {
            Box::new(crate::game_state::upgrade::MembershipCardUpgrade::into_upgrade)
        }
        other => panic!("Invalid treasure upgrade kind: {:?}", other),
    }
}

fn next_cat_add(gold_earn_plus: usize) -> usize {
    match gold_earn_plus {
        0 | 1 => 1,
        2 => 2,
        4 => 4,
        8 => 8,
        _ => 0,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_treasure_upgrade_kind_gen_maxed_cat_does_not_panic() {
        let upgrade_state = UpgradeState {
            upgrades: vec![crate::game_state::upgrade::CatUpgrade::into_upgrade(
                MAX_GOLD_EARN_PLUS,
            )],
            ..UpgradeState::default()
        };
        let kind_gen =
            make_treasure_upgrade_kind_gen(UpgradeDiscriminants::Cat, None, &upgrade_state);
        assert!(matches!((kind_gen)(), Upgrade::Cat(..)));
    }
}
