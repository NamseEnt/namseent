use super::GameState;
use crate::{
    card::{REVERSED_RANKS, Rank, SUITS, Suit},
    rarity::Rarity,
};
use namui::*;
use rand::{Rng, seq::SliceRandom, thread_rng};

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub rarity: Rarity,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Heal {
        amount: f32,
    },
    TowerDamagePlus {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerDamageMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerSpeedPlus {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerSpeedMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerRangePlus {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    WeakenMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    SlowdownMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    Attack {
        rank: Rank,
        suit: Suit,
        damage: f32,
        radius: f32,
    },
}

impl ItemKind {
    pub fn name(&self) -> &'static str {
        match self {
            ItemKind::Heal { .. } => "회복",
            ItemKind::TowerDamagePlus { .. } => "타워 공격력 증가",
            ItemKind::TowerDamageMultiply { .. } => "타워 공격력 증가",
            ItemKind::TowerSpeedPlus { .. } => "타워 공격 속도 증가",
            ItemKind::TowerSpeedMultiply { .. } => "타워 공격 속도 증가",
            ItemKind::TowerRangePlus { .. } => "타워 사거리 증가",
            ItemKind::WeakenMultiply { .. } => "적 공격력 약화",
            ItemKind::SlowdownMultiply { .. } => "적 슬로우",
            ItemKind::Attack { .. } => "범위공격",
        }
    }
    pub fn description(&self) -> String {
        match self {
            ItemKind::Heal { amount } => format!("체력을 {amount} 회복합니다"),
            ItemKind::TowerDamagePlus {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격력을 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::TowerDamageMultiply {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격력을 {amount}배 만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::TowerSpeedPlus {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격 속도를 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::TowerSpeedMultiply {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격 속도를 {amount}배 만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::TowerRangePlus {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 사거리를 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::WeakenMultiply {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 적들의 공격력을 {amount}배 만큼 약화시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::SlowdownMultiply {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 적들의 이동 속도를 {amount}배 만큼 느리게 합니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::Attack {
                rank,
                suit,
                damage,
                radius,
            } => format!("{radius} 범위 내 적들에게 {damage}만큼의 {suit}{rank} 피해를 입힙니다."),
        }
    }
}

pub fn use_item(item_index: usize) -> &'static ItemKind {
    todo!()
}

pub fn generate_items(game_state: &GameState, amount: usize) -> Vec<Item> {
    let rarity_table = generate_rarity_table(game_state.stage);
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

    rarities
        .into_iter()
        .map(|rarity| Item {
            kind: generate_item(rarity),
            rarity,
        })
        .collect()
}
pub fn generate_item(rarity: Rarity) -> ItemKind {
    let candidates = generate_item_candidate_table(rarity);
    let candidate = &candidates
        .choose_weighted(&mut rand::thread_rng(), |x| x.1)
        .unwrap()
        .0;

    match candidate {
        ItemCandidate::Heal => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 5.0..9.0,
                Rarity::Rare => 10.0..14.0,
                Rarity::Epic => 15.0..19.0,
                Rarity::Legendary => 20.0..25.0,
            });
            ItemKind::Heal { amount }
        }
        ItemCandidate::TowerDamagePlus => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.0..5.0,
                Rarity::Rare => 5.0..10.0,
                Rarity::Epic => 15.0..40.0,
                Rarity::Legendary => 50.0..100.0,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 4,
                Rarity::Epic => 6,
                Rarity::Legendary => 10,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::TowerDamagePlus {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::TowerDamageMultiply => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.5,
                Rarity::Epic => 1.5..1.75,
                Rarity::Legendary => 1.75..2.0,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 4,
                Rarity::Epic => 6,
                Rarity::Legendary => 10,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::TowerDamageMultiply {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::TowerSpeedPlus => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.1..0.25,
                Rarity::Rare => 0.25..0.5,
                Rarity::Epic => 0.5..0.75,
                Rarity::Legendary => 0.75..1.0,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 4,
                Rarity::Epic => 6,
                Rarity::Legendary => 10,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::TowerSpeedPlus {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::TowerSpeedMultiply => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 1.1..1.2,
                Rarity::Rare => 1.2..1.5,
                Rarity::Epic => 1.5..1.75,
                Rarity::Legendary => 1.75..2.0,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 4,
                Rarity::Epic => 6,
                Rarity::Legendary => 10,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::TowerSpeedMultiply {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::TowerRangePlus => {
            let amount = thread_rng().gen_range(match rarity {
                Rarity::Common => 0.5..1.0,
                Rarity::Rare => 1.0..2.0,
                Rarity::Epic => 2.0..3.0,
                Rarity::Legendary => 3.0..5.0,
            });
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 4,
                Rarity::Epic => 6,
                Rarity::Legendary => 10,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::TowerRangePlus {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::WeakenMultiply => {
            let amount = match rarity {
                Rarity::Common => 0.9,
                Rarity::Rare => 0.8,
                Rarity::Epic => 0.6,
                Rarity::Legendary => 0.4,
            };
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 4,
                Rarity::Epic => 6,
                Rarity::Legendary => 10,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::WeakenMultiply {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::SlowdownMultiply => {
            let amount = match rarity {
                Rarity::Common => 0.9,
                Rarity::Rare => 0.8,
                Rarity::Epic => 0.6,
                Rarity::Legendary => 0.4,
            };
            let duration = Duration::from_secs(match rarity {
                Rarity::Common => 2,
                Rarity::Rare => 4,
                Rarity::Epic => 6,
                Rarity::Legendary => 10,
            });
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::SlowdownMultiply {
                amount,
                duration,
                radius,
            }
        }
        ItemCandidate::Attack => {
            let mut rng = thread_rng();
            let rank = *REVERSED_RANKS.choose(&mut rng).unwrap();
            let suit = *SUITS.choose(&mut rng).unwrap();
            let damage = match rarity {
                Rarity::Common => 25.0,
                Rarity::Rare => 500.0,
                Rarity::Epic => 2500.0,
                Rarity::Legendary => 5000.0,
            };
            let radius = match rarity {
                Rarity::Common => 5.0,
                Rarity::Rare => 7.0,
                Rarity::Epic => 10.0,
                Rarity::Legendary => 15.0,
            };
            ItemKind::Attack {
                rank,
                suit,
                damage,
                radius,
            }
        }
    }
}
fn generate_rarity_table(stage: usize) -> Vec<(Rarity, f32)> {
    let rarity_weight = match stage {
        1..=4 => [0.9, 0.1, 0.0, 0.0],
        5..=9 => [0.75, 0.25, 0.0, 0.0],
        10..=14 => [0.55, 0.3, 0.15, 0.0],
        15..=19 => [0.45, 0.33, 0.2, 0.02],
        20..=24 => [0.25, 0.4, 0.3, 0.05],
        25..=29 => [0.19, 0.3, 0.35, 0.15],
        30..=34 => [0.16, 0.2, 0.35, 0.25],
        35..=39 => [0.09, 0.15, 0.3, 0.3],
        40..=50 => [0.05, 0.1, 0.3, 0.4],
        _ => panic!("Invalid stage: {}", stage),
    };
    let rarity_table = vec![
        (Rarity::Common, rarity_weight[0]),
        (Rarity::Rare, rarity_weight[1]),
        (Rarity::Epic, rarity_weight[2]),
        (Rarity::Legendary, rarity_weight[3]),
    ];
    rarity_table
}
fn generate_item_candidate_table(rarity: Rarity) -> Vec<(ItemCandidate, f32)> {
    let candidate_weight = match rarity {
        Rarity::Common => [0.5, 0.3, 0.1, 0.3, 0.1, 0.2, 0.5, 0.5, 0.5],
        Rarity::Rare => [0.25, 0.2, 0.2, 0.2, 0.2, 0.2, 0.5, 0.5, 0.5],
        Rarity::Epic => [0.1, 0.1, 0.2, 0.1, 0.2, 0.2, 0.1, 0.3, 0.3],
        Rarity::Legendary => [0.1, 0.1, 0.3, 0.1, 0.3, 0.2, 0.1, 0.2, 0.2],
    };
    let candidate_table = vec![
        (ItemCandidate::Heal, candidate_weight[0]),
        (ItemCandidate::TowerDamagePlus, candidate_weight[1]),
        (ItemCandidate::TowerDamageMultiply, candidate_weight[2]),
        (ItemCandidate::TowerSpeedPlus, candidate_weight[3]),
        (ItemCandidate::TowerSpeedMultiply, candidate_weight[4]),
        (ItemCandidate::TowerRangePlus, candidate_weight[5]),
        (ItemCandidate::WeakenMultiply, candidate_weight[6]),
        (ItemCandidate::SlowdownMultiply, candidate_weight[7]),
        (ItemCandidate::Attack, candidate_weight[8]),
    ];
    candidate_table
}
enum ItemCandidate {
    Heal,
    TowerDamagePlus,
    TowerDamageMultiply,
    TowerSpeedPlus,
    TowerSpeedMultiply,
    TowerRangePlus,
    WeakenMultiply,
    SlowdownMultiply,
    Attack,
}
