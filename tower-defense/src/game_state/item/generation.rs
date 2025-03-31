use super::{Item, ItemKind};
use crate::{
    card::{REVERSED_RANKS, SUITS},
    game_state::GameState,
    rarity::Rarity,
};
use namui::*;
use rand::{Rng, seq::SliceRandom, thread_rng};

pub fn generate_items(game_state: &GameState, amount: usize) -> Vec<Item> {
    (0..amount)
        .map(|_| game_state.generate_rarity())
        .map(generate_item)
        .collect()
}
pub fn generate_item(rarity: Rarity) -> Item {
    let candidates = generate_item_candidate_table(rarity);
    let candidate = &candidates
        .choose_weighted(&mut rand::thread_rng(), |x| x.1)
        .unwrap()
        .0;

    let kind = match candidate {
        ItemCandidate::Heal => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 5.0..9.0,
                Rarity::Rare => 10.0..14.0,
                Rarity::Epic => 15.0..19.0,
                Rarity::Legendary => 20.0..25.0,
            });
            ItemKind::Heal { amount }
        }
        ItemCandidate::AttackPowerPlusBuff => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 5.0..10.0,
                Rarity::Rare => 10.0..15.0,
                Rarity::Epic => 15.0..40.0,
                Rarity::Legendary => 50.0..100.0,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 3,
                Rarity::Epic => 5,
                Rarity::Legendary => 8,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::AttackPowerPlusBuff {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::AttackPowerMultiplyBuff => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.5,
                Rarity::Epic => 1.5..1.75,
                Rarity::Legendary => 1.75..2.0,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 3,
                Rarity::Epic => 5,
                Rarity::Legendary => 8,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::AttackPowerMultiplyBuff {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::AttackSpeedPlusBuff => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.1..0.25,
                Rarity::Rare => 0.25..0.5,
                Rarity::Epic => 0.5..0.75,
                Rarity::Legendary => 0.75..1.0,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 3,
                Rarity::Epic => 5,
                Rarity::Legendary => 8,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::AttackSpeedPlusBuff {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::AttackSpeedMultiplyBuff => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.5,
                Rarity::Epic => 1.5..1.75,
                Rarity::Legendary => 1.75..2.0,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 3,
                Rarity::Epic => 5,
                Rarity::Legendary => 8,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::AttackSpeedMultiplyBuff {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::AttackRangePlus => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.5,
                Rarity::Epic => 1.5..1.75,
                Rarity::Legendary => 1.75..2.0,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 3,
                Rarity::Epic => 5,
                Rarity::Legendary => 8,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::AttackRangePlus {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::MovementSpeedDebuff => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.8..0.9,
                Rarity::Rare => 0.7..0.8,
                Rarity::Epic => 0.6..0.7,
                Rarity::Legendary => 0.5..0.6,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 3,
                Rarity::Epic => 5,
                Rarity::Legendary => 8,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::MovementSpeedDebuff {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::RoundDamage => {
            let mut rng = thread_rng();
            let rank = *REVERSED_RANKS.choose(&mut rng).unwrap();
            let suit = *SUITS.choose(&mut rng).unwrap();
            let damage = thread_rng().gen_range(match rarity {
                Rarity::Common => 25.0..100.0,
                Rarity::Rare => 250.0..750.0,
                Rarity::Epic => 2000.0..4000.0,
                Rarity::Legendary => 5000.0..7500.0,
            });
            let radius = match rarity {
                Rarity::Common => 1.0,
                Rarity::Rare => 2.0,
                Rarity::Epic => 4.0,
                Rarity::Legendary => 5.0,
            };
            ItemKind::RoundDamage {
                rank,
                suit,
                damage,
                radius,
            }
        }
        ItemCandidate::RoundDamageOverTime => {
            let mut rng = thread_rng();
            let rank = *REVERSED_RANKS.choose(&mut rng).unwrap();
            let suit = *SUITS.choose(&mut rng).unwrap();
            let damage = thread_rng().gen_range(match rarity {
                Rarity::Common => 50.0..150.0,
                Rarity::Rare => 400.0..800.0,
                Rarity::Epic => 3000.0..6000.0,
                Rarity::Legendary => 8000.0..10000.0,
            });
            let radius = match rarity {
                Rarity::Common => 2.0,
                Rarity::Rare => 3.0,
                Rarity::Epic => 4.0,
                Rarity::Legendary => 5.0,
            };
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 3,
                Rarity::Rare => 4,
                Rarity::Epic => 6,
                Rarity::Legendary => 8,
            });
            ItemKind::RoundDamageOverTime {
                rank,
                suit,
                damage,
                radius,
                duration,
            }
        }
        ItemCandidate::Lottery => {
            let amount = match rarity {
                Rarity::Common => 250.0,
                Rarity::Rare => 500.0,
                Rarity::Epic => 1000.0,
                Rarity::Legendary => 2500.0,
            };
            let probability = match rarity {
                Rarity::Common => 0.01,
                Rarity::Rare => 0.02,
                Rarity::Epic => 0.03,
                Rarity::Legendary => 0.05,
            };
            ItemKind::Lottery {
                amount,
                probability,
            }
        }
        ItemCandidate::LinearDamage => {
            let mut rng = thread_rng();
            let rank = *REVERSED_RANKS.choose(&mut rng).unwrap();
            let suit = *SUITS.choose(&mut rng).unwrap();
            let damage = thread_rng().gen_range(match rarity {
                Rarity::Common => 25.0..100.0,
                Rarity::Rare => 250.0..750.0,
                Rarity::Epic => 2000.0..4000.0,
                Rarity::Legendary => 5000.0..7500.0,
            });
            let thickness = 2.0;
            ItemKind::LinearDamage {
                rank,
                suit,
                damage,
                thickness,
            }
        }
        ItemCandidate::LinearDamageOverTime => {
            let mut rng = thread_rng();
            let rank = *REVERSED_RANKS.choose(&mut rng).unwrap();
            let suit = *SUITS.choose(&mut rng).unwrap();
            let damage = thread_rng().gen_range(match rarity {
                Rarity::Common => 50.0..150.0,
                Rarity::Rare => 400.0..800.0,
                Rarity::Epic => 3000.0..6000.0,
                Rarity::Legendary => 8000.0..10000.0,
            });
            let thickness = 2.0;
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 3,
                Rarity::Rare => 4,
                Rarity::Epic => 6,
                Rarity::Legendary => 8,
            });
            ItemKind::LinearDamageOverTime {
                rank,
                suit,
                damage,
                thickness,
                duration,
            }
        }
        ItemCandidate::ExtraReroll => ItemKind::ExtraReroll,
        ItemCandidate::Shield => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 10.0..15.0,
                Rarity::Rare => 15.0..25.0,
                Rarity::Epic => 25.0..35.0,
                Rarity::Legendary => 35.0..50.0,
            });
            ItemKind::Shield { amount }
        }
        ItemCandidate::DamageReduction => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.85..0.9,
                Rarity::Rare => 0.8..0.85,
                Rarity::Epic => 0.7..0.8,
                Rarity::Legendary => 0.55..0.7,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 3,
                Rarity::Rare => 4,
                Rarity::Epic => 6,
                Rarity::Legendary => 8,
            });
            ItemKind::DamageReduction {
                damage_multiply: amount,
                duration,
            }
        }
    };

    Item { kind, rarity }
}
fn generate_item_candidate_table(rarity: Rarity) -> Vec<(ItemCandidate, f32)> {
    let candidate_weight = match rarity {
        Rarity::Common => [
            100.0, 100.0, 50.0, 80.0, 40.0, 20.0, 50.0, 30.0, 25.0, 10.0, 20.0, 15.0, 1.0, 5.0, 5.0,
        ],
        Rarity::Rare => [
            100.0, 90.0, 60.0, 70.0, 50.0, 20.0, 60.0, 35.0, 30.0, 30.0, 25.0, 20.0, 5.0, 10.0,
            10.0,
        ],
        Rarity::Epic => [
            100.0, 80.0, 70.0, 60.0, 60.0, 20.0, 70.0, 40.0, 35.0, 30.0, 30.0, 25.0, 20.0, 30.0,
            30.0,
        ],
        Rarity::Legendary => [
            100.0, 70.0, 80.0, 50.0, 70.0, 20.0, 80.0, 45.0, 40.0, 30.0, 35.0, 30.0, 30.0, 50.0,
            30.0,
        ],
    };
    let candidate_table = vec![
        (ItemCandidate::Heal, candidate_weight[0]),
        (ItemCandidate::AttackPowerPlusBuff, candidate_weight[1]),
        (ItemCandidate::AttackPowerMultiplyBuff, candidate_weight[2]),
        (ItemCandidate::AttackSpeedPlusBuff, candidate_weight[3]),
        (ItemCandidate::AttackSpeedMultiplyBuff, candidate_weight[4]),
        (ItemCandidate::AttackRangePlus, candidate_weight[5]),
        (ItemCandidate::MovementSpeedDebuff, candidate_weight[6]),
        (ItemCandidate::RoundDamage, candidate_weight[7]),
        (ItemCandidate::RoundDamageOverTime, candidate_weight[8]),
        (ItemCandidate::Lottery, candidate_weight[9]),
        (ItemCandidate::LinearDamage, candidate_weight[10]),
        (ItemCandidate::LinearDamageOverTime, candidate_weight[11]),
        (ItemCandidate::ExtraReroll, candidate_weight[12]),
        (ItemCandidate::Shield, candidate_weight[13]),
        (ItemCandidate::DamageReduction, candidate_weight[14]),
    ];
    candidate_table
}
enum ItemCandidate {
    Heal,
    AttackPowerPlusBuff,
    AttackPowerMultiplyBuff,
    AttackSpeedPlusBuff,
    AttackSpeedMultiplyBuff,
    AttackRangePlus,
    MovementSpeedDebuff,
    RoundDamage,
    RoundDamageOverTime,
    Lottery,
    LinearDamage,
    LinearDamageOverTime,
    ExtraReroll,
    Shield,
    DamageReduction,
}
