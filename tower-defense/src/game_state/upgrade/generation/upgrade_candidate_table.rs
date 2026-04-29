use super::*;
use crate::game_state::upgrade::{UpgradeKindDiscriminants, MAX_REMOVE_NUMBER_RANKS};
use crate::game_state::{GameState, tower::TowerKind};

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
        .map(|upgrade| {
            let disc: UpgradeKindDiscriminants = upgrade.name.parse().unwrap_or_else(|_| panic!("Unknown tower damage upgrade config name: {}", upgrade.name));
            CandidateRow {
                weight: upgrade.entry.weight,
                kind_gen: make_tower_damage_upgrade_kind_gen(
                    disc,
                    upgrade.entry.damage_multiplier,
                ),
            }
        })
        .collect()
}

fn make_tower_damage_upgrade_kind_gen(disc: UpgradeKindDiscriminants, damage_multiplier: Option<f32>) -> KindGen {
    let damage_multiplier = damage_multiplier.unwrap_or(1.0);
    match disc {
        UpgradeKindDiscriminants::Staff => Box::new(move || UpgradeKind::Staff(StaffUpgrade { damage_multiplier })),
        UpgradeKindDiscriminants::LongSword => {
            Box::new(move || UpgradeKind::LongSword(LongSwordUpgrade { damage_multiplier }))
        }
        UpgradeKindDiscriminants::Mace => Box::new(move || UpgradeKind::Mace(MaceUpgrade { damage_multiplier })),
        UpgradeKindDiscriminants::ClubSword => {
            Box::new(move || UpgradeKind::ClubSword(ClubSwordUpgrade { damage_multiplier }))
        }
        UpgradeKindDiscriminants::Tricycle => {
            Box::new(move || UpgradeKind::Tricycle(TricycleUpgrade { damage_multiplier }))
        }
        UpgradeKindDiscriminants::SingleChopstick => Box::new(move || {
            UpgradeKind::SingleChopstick(SingleChopstickUpgrade { damage_multiplier })
        }),
        UpgradeKindDiscriminants::PairChopsticks => Box::new(move || {
            UpgradeKind::PairChopsticks(PairChopsticksUpgrade { damage_multiplier })
        }),
        UpgradeKindDiscriminants::FountainPen => {
            Box::new(move || UpgradeKind::FountainPen(FountainPenUpgrade { damage_multiplier }))
        }
        UpgradeKindDiscriminants::Brush => Box::new(move || UpgradeKind::Brush(BrushUpgrade { damage_multiplier })),
        UpgradeKindDiscriminants::BrokenPottery => {
            Box::new(move || UpgradeKind::BrokenPottery(BrokenPotteryUpgrade { damage_multiplier }))
        }
        UpgradeKindDiscriminants::PerfectPottery => {
            Box::new(move || UpgradeKind::PerfectPottery(PerfectPotteryUpgrade { damage_multiplier }))
        }
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
        let disc: UpgradeKindDiscriminants = upgrade.name.parse().unwrap_or_else(|_| panic!("Unknown treasure upgrade config name: {}", upgrade.name));
        let weight = upgrade.entry.weight;
        let kind_gen = make_treasure_upgrade_kind_gen(
            disc,
            upgrade.entry.damage_multiplier,
            upgrade_state,
        );
        let current_and_max = match disc {
            UpgradeKindDiscriminants::Cat => Some((upgrade_state.gold_earn_plus(), MAX_GOLD_EARN_PLUS)),
            UpgradeKindDiscriminants::Backpack => Some((upgrade_state.shop_slot_expand(), MAX_SHOP_SLOT_EXPAND)),
            UpgradeKindDiscriminants::DiceBundle => Some((upgrade_state.dice_chance_plus(), MAX_DICE_CHANCE_PLUS)),
            UpgradeKindDiscriminants::EnergyDrink => Some((
                upgrade_state.shop_item_price_minus(),
                MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE,
            )),
            UpgradeKindDiscriminants::FourLeafClover => Some((
                upgrade_state.shorten_straight_flush_to_4_cards() as usize,
                1,
            )),
            UpgradeKindDiscriminants::Rabbit => Some((upgrade_state.skip_rank_for_straight() as usize, 1)),
            UpgradeKindDiscriminants::BlackWhite => Some((upgrade_state.treat_suits_as_same() as usize, 1)),
            UpgradeKindDiscriminants::Eraser => Some((
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
    disc: UpgradeKindDiscriminants,
    damage_multiplier: Option<f32>,
    upgrade_state: &UpgradeState,
) -> KindGen {
    let damage_multiplier = damage_multiplier.unwrap_or(1.0);
    let add = match disc {
        UpgradeKindDiscriminants::Cat => next_cat_add(upgrade_state.gold_earn_plus()),
        UpgradeKindDiscriminants::Backpack => 1,
        UpgradeKindDiscriminants::DiceBundle => 1,
        UpgradeKindDiscriminants::EnergyDrink => 5,
        UpgradeKindDiscriminants::Eraser => 1,
        _ => 0,
    };
    match disc {
        UpgradeKindDiscriminants::Cat => Box::new(move || UpgradeKind::Cat(CatUpgrade { add })),
        UpgradeKindDiscriminants::Backpack => Box::new(move || UpgradeKind::Backpack(BackpackUpgrade { add })),
        UpgradeKindDiscriminants::DiceBundle => Box::new(move || UpgradeKind::DiceBundle(DiceBundleUpgrade { add })),
        UpgradeKindDiscriminants::EnergyDrink => Box::new(move || UpgradeKind::EnergyDrink(EnergyDrinkUpgrade { add })),
        UpgradeKindDiscriminants::PerfectPottery => Box::new(move || {
            UpgradeKind::PerfectPottery(PerfectPotteryUpgrade { damage_multiplier })
        }),
        UpgradeKindDiscriminants::BrokenPottery => {
            Box::new(move || UpgradeKind::BrokenPottery(BrokenPotteryUpgrade { damage_multiplier }))
        }
        UpgradeKindDiscriminants::FourLeafClover => Box::new(|| UpgradeKind::FourLeafClover(FourLeafCloverUpgrade)),
        UpgradeKindDiscriminants::Rabbit => Box::new(|| UpgradeKind::Rabbit(RabbitUpgrade)),
        UpgradeKindDiscriminants::BlackWhite => Box::new(|| UpgradeKind::BlackWhite(BlackWhiteUpgrade)),
        UpgradeKindDiscriminants::Eraser => Box::new(move || UpgradeKind::Eraser(EraserUpgrade { add })),
        UpgradeKindDiscriminants::Trophy => Box::new(|| {
            UpgradeKind::Trophy(TrophyUpgrade {
                perfect_clear_stacks: 0,
            })
        }),
        UpgradeKindDiscriminants::Crock => Box::new(|| UpgradeKind::Crock(CrockUpgrade)),
        UpgradeKindDiscriminants::DemolitionHammer => Box::new(move || {
            UpgradeKind::DemolitionHammer(DemolitionHammerUpgrade {
                damage_multiplier,
                removed_tower_count: 0,
            })
        }),
        UpgradeKindDiscriminants::Metronome => Box::new(|| UpgradeKind::Metronome(MetronomeUpgrade { start_stage: None })),
        UpgradeKindDiscriminants::Tape => Box::new(|| UpgradeKind::Tape(TapeUpgrade { acquired_stage: 0 })),
        UpgradeKindDiscriminants::NameTag => Box::new(move || {
            UpgradeKind::NameTag(NameTagUpgrade {
                damage_multiplier,
                pending: true,
            })
        }),
        UpgradeKindDiscriminants::ShoppingBag => Box::new(move || {
            UpgradeKind::ShoppingBag(ShoppingBagUpgrade {
                damage_multiplier,
                stacks: 0,
            })
        }),
        UpgradeKindDiscriminants::Resolution => Box::new(move || {
            UpgradeKind::Resolution(ResolutionUpgrade {
                damage_multiplier_per_reroll: damage_multiplier,
                pending: true,
            })
        }),
        UpgradeKindDiscriminants::Mirror => Box::new(|| UpgradeKind::Mirror(MirrorUpgrade { pending: true })),
        UpgradeKindDiscriminants::IceCream => Box::new(move || {
            UpgradeKind::IceCream(IceCreamUpgrade {
                damage_multiplier,
                waves_remaining: 5,
            })
        }),
        UpgradeKindDiscriminants::Spanner => Box::new(|| UpgradeKind::Spanner(SpannerUpgrade)),
        UpgradeKindDiscriminants::Pea => Box::new(|| UpgradeKind::Pea(PeaUpgrade)),
        UpgradeKindDiscriminants::SlotMachine => Box::new(|| {
            UpgradeKind::SlotMachine(SlotMachineUpgrade {
                next_round_dice: 10,
            })
        }),
        UpgradeKindDiscriminants::PiggyBank => Box::new(|| UpgradeKind::PiggyBank(PiggyBankUpgrade)),
        UpgradeKindDiscriminants::Camera => Box::new(|| UpgradeKind::Camera(CameraUpgrade)),
        UpgradeKindDiscriminants::GiftBox => Box::new(|| UpgradeKind::GiftBox(GiftBoxUpgrade)),
        UpgradeKindDiscriminants::Fang => Box::new(|| UpgradeKind::Fang(FangUpgrade)),
        UpgradeKindDiscriminants::Popcorn => Box::new(move || {
            UpgradeKind::Popcorn(PopcornUpgrade {
                max_multiplier: damage_multiplier,
                duration: 5,
                waves_remaining: 5,
            })
        }),
        UpgradeKindDiscriminants::MembershipCard => Box::new(|| {
            UpgradeKind::MembershipCard(MembershipCardUpgrade {
                pending_free_shop: true,
            })
        }),
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
            upgrades: vec![Upgrade {
                kind: UpgradeKind::Cat(CatUpgrade {
                    add: MAX_GOLD_EARN_PLUS,
                }),
                value: crate::OneZero::default(),
            }],
            ..UpgradeState::default()
        };
        let kind_gen = make_treasure_upgrade_kind_gen(UpgradeKindDiscriminants::Cat, None, &upgrade_state);
        assert!(matches!(
            (kind_gen)(),
            UpgradeKind::Cat(CatUpgrade { add: 0 })
        ));
    }
}
