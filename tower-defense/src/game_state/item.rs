use super::{GameState, MAX_HP};
use crate::{
    MapCoordF32,
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
    AttackPowerPlusBuff {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackPowerMultiplyBuff {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackSpeedPlusBuff {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackSpeedMultiplyBuff {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackRangePlus {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    MovementSpeedDebuff {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    RoundDamage {
        rank: Rank,
        suit: Suit,
        damage: f32,
        radius: f32,
    },
    RoundDamageOverTime {
        rank: Rank,
        suit: Suit,
        damage: f32,
        radius: f32,
        duration: Duration,
    },
    Lottery {
        amount: f32,
        probability: f32,
    },
    LinearDamage {
        rank: Rank,
        suit: Suit,
        damage: f32,
        thickness: f32,
    },
    LinearDamageOverTime {
        rank: Rank,
        suit: Suit,
        damage: f32,
        thickness: f32,
        duration: Duration,
    },
    ExtraReroll,
    Shield {
        amount: f32,
    },
    DamageReduction {
        amount: f32,
        duration: Duration,
    },
}

impl ItemKind {
    pub fn name(&self) -> &'static str {
        match self {
            ItemKind::Heal { .. } => "치유",
            ItemKind::AttackPowerPlusBuff { .. } => "공격력 증가 버프",
            ItemKind::AttackPowerMultiplyBuff { .. } => "공격력 배수 버프",
            ItemKind::AttackSpeedPlusBuff { .. } => "공격 속도 증가 버프",
            ItemKind::AttackSpeedMultiplyBuff { .. } => "공격 속도 배수 버프",
            ItemKind::AttackRangePlus { .. } => "공격 범위 증가",
            ItemKind::MovementSpeedDebuff { .. } => "이동 속도 감소 디버프",
            ItemKind::RoundDamage { .. } => "범위 피해",
            ItemKind::RoundDamageOverTime { .. } => "지속 범위 피해",
            ItemKind::Lottery { .. } => "복권",
            ItemKind::LinearDamage { .. } => "광선 피해",
            ItemKind::LinearDamageOverTime { .. } => "지속 광선 피해",
            ItemKind::ExtraReroll => "추가 리롤",
            ItemKind::Shield { .. } => "방어막",
            ItemKind::DamageReduction { .. } => "피해 감소",
        }
    }
    pub fn description(&self) -> String {
        match self {
            ItemKind::Heal { amount } => format!("체력을 {amount} 회복합니다"),
            ItemKind::AttackPowerPlusBuff {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격력을 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::AttackPowerMultiplyBuff {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격력을 {amount}배 만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::AttackSpeedPlusBuff {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격 속도를 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::AttackSpeedMultiplyBuff {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격 속도를 {amount}배 만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::AttackRangePlus {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 사거리를 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::MovementSpeedDebuff {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 적들의 이동 속도를 {amount}만큼 감소시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::RoundDamage {
                rank,
                suit,
                damage,
                radius,
            } => format!("{radius} 범위 내 적들에게 {damage}만큼의 {suit}{rank} 피해를 입힙니다."),
            ItemKind::RoundDamageOverTime {
                rank,
                suit,
                damage,
                radius,
                duration,
            } => format!(
                "{radius} 범위 내 적들에게 {damage}만큼의 {suit}{rank} 지속 피해를 입힙니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::Lottery {
                amount,
                probability,
            } => format!("{probability}% 확률로 {amount}의 보상을 획득합니다"),
            ItemKind::LinearDamage {
                rank,
                suit,
                damage,
                thickness,
            } => format!(
                "{thickness} 두께의 직선 범위 내 적들에게 {damage}만큼의 {suit}{rank} 피해를 입힙니다."
            ),
            ItemKind::LinearDamageOverTime {
                rank,
                suit,
                damage,
                thickness,
                duration,
            } => format!(
                "{thickness} 두께의 직선 범위 내 적들에게 {damage}만큼의 {suit}{rank} 지속 피해를 입힙니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::ExtraReroll => "추가 리롤을 획득합니다".to_string(),
            ItemKind::Shield { amount } => {
                format!("이번 라운드에 피해를 {amount}흡수하는 방어막을 획득합니다.")
            }
            ItemKind::DamageReduction { amount, duration } => {
                format!("{duration:?} 동안 받는 피해를 {amount}만큼 감소시킵니다")
            }
        }
    }
    pub fn usage(&self) -> ItemUsage {
        match self {
            ItemKind::Heal { .. } => ItemUsage::Instant,
            ItemKind::AttackPowerPlusBuff { radius, .. }
            | ItemKind::AttackPowerMultiplyBuff { radius, .. }
            | ItemKind::AttackSpeedPlusBuff { radius, .. }
            | ItemKind::AttackSpeedMultiplyBuff { radius, .. }
            | ItemKind::AttackRangePlus { radius, .. }
            | ItemKind::MovementSpeedDebuff { radius, .. }
            | ItemKind::RoundDamage { radius, .. }
            | ItemKind::RoundDamageOverTime { radius, .. } => {
                ItemUsage::CircularArea { radius: *radius }
            }
            ItemKind::Lottery { .. } => ItemUsage::Instant,
            ItemKind::LinearDamage { thickness, .. }
            | ItemKind::LinearDamageOverTime { thickness, .. } => ItemUsage::LinearArea {
                thickness: *thickness,
            },
            ItemKind::ExtraReroll => ItemUsage::Instant,
            ItemKind::Shield { .. } => ItemUsage::Instant,
            ItemKind::DamageReduction { .. } => ItemUsage::Instant,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItemUsage {
    Instant,
    CircularArea { radius: f32 },
    LinearArea { thickness: f32 },
}

pub fn use_item(game_state: &mut GameState, item: &Item, xy: Option<MapCoordF32>) {
    match item.kind {
        ItemKind::Heal { amount } => game_state.hp = (game_state.hp + amount).min(MAX_HP),
        ItemKind::AttackPowerPlusBuff {
            amount,
            duration,
            radius,
        } => todo!(),
        ItemKind::AttackPowerMultiplyBuff {
            amount,
            duration,
            radius,
        } => todo!(),
        ItemKind::AttackSpeedPlusBuff {
            amount,
            duration,
            radius,
        } => todo!(),
        ItemKind::AttackSpeedMultiplyBuff {
            amount,
            duration,
            radius,
        } => todo!(),
        ItemKind::AttackRangePlus {
            amount,
            duration,
            radius,
        } => todo!(),
        ItemKind::MovementSpeedDebuff {
            amount,
            duration,
            radius,
        } => todo!(),
        ItemKind::RoundDamage {
            rank,
            suit,
            damage,
            radius,
        } => todo!(),
        ItemKind::RoundDamageOverTime {
            rank,
            suit,
            damage,
            radius,
            duration,
        } => todo!(),
        ItemKind::Lottery {
            amount,
            probability,
        } => {
            let is_winner = thread_rng().gen_bool(probability as f64);
            if !is_winner {
                return;
            }
            game_state.gold += amount as usize;
            // TODO: Show effect on win
        }
        ItemKind::LinearDamage {
            rank,
            suit,
            damage,
            thickness,
        } => todo!(),
        ItemKind::LinearDamageOverTime {
            rank,
            suit,
            damage,
            thickness,
            duration,
        } => todo!(),
        ItemKind::ExtraReroll => {
            game_state.left_reroll_chance += 1;
        }
        ItemKind::Shield { amount } => {
            game_state.shield += amount;
        }
        ItemKind::DamageReduction { amount, duration } => todo!(),
    }
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
            ItemKind::DamageReduction { amount, duration }
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
