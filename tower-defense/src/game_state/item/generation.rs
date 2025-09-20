use super::{Item, ItemKind};
use crate::{
    rarity::Rarity,
};
use namui::*;
use rand::{Rng, seq::SliceRandom, thread_rng};

/// 주어진 value(0.0~1.0)를 범위에 맞는 실제 값으로 변환
fn calculate_amount_from_value(value: f32, min_value: f32, max_value: f32) -> f32 {
    let clamped_value = value.clamp(0.0, 1.0);
    min_value + (max_value - min_value) * clamped_value
}

/// MovementSpeedDebuff나 DamageReduction 같은 역효과 아이템용 변환
fn calculate_reverse_amount_from_value(value: f32, min_value: f32, max_value: f32) -> f32 {
    let clamped_value = value.clamp(0.0, 1.0);
    // value가 높을수록 더 좋은 효과를 원하므로, 더 낮은 amount를 반환
    max_value - (max_value - min_value) * clamped_value
}

pub fn generate_item(rarity: Rarity) -> Item {
    let candidates = generate_item_candidate_table(rarity);
    let candidate = &candidates
        .choose_weighted(&mut rand::thread_rng(), |x| x.1)
        .unwrap()
        .0;

    // 먼저 0~1 범위의 랜덤 value 생성
    let value = thread_rng().gen_range(0.0..1.0);

    let kind = match candidate {
        ItemCandidate::Heal => {
            let range = match rarity {
                Rarity::Common => 5.0..9.0,
                Rarity::Rare => 10.0..14.0,
                Rarity::Epic => 15.0..19.0,
                Rarity::Legendary => 20.0..25.0,
            };
            let amount = calculate_amount_from_value(value, range.start, range.end);
            ItemKind::Heal { amount }
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
        ItemCandidate::ExtraReroll => ItemKind::ExtraReroll,
        ItemCandidate::Shield => {
            let range = match rarity {
                Rarity::Common => 10.0..15.0,
                Rarity::Rare => 15.0..25.0,
                Rarity::Epic => 25.0..35.0,
                Rarity::Legendary => 35.0..50.0,
            };
            let amount = calculate_amount_from_value(value, range.start, range.end);
            ItemKind::Shield { amount }
        }
        ItemCandidate::DamageReduction => {
            let range = match rarity {
                Rarity::Common => 0.85..0.9,
                Rarity::Rare => 0.8..0.85,
                Rarity::Epic => 0.7..0.8,
                Rarity::Legendary => 0.55..0.7,
            };
            let amount = calculate_reverse_amount_from_value(value, range.start, range.end);
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

    Item {
        kind,
        rarity,
        value: value.into(),
    }
}

fn generate_item_candidate_table(rarity: Rarity) -> Vec<(ItemCandidate, f32)> {
    let candidate_weight = match rarity {
        Rarity::Common => [100.0, 10.0, 5.0, 5.0, 5.0],
        Rarity::Rare => [100.0, 30.0, 10.0, 10.0, 10.0],
        Rarity::Epic => [100.0, 30.0, 20.0, 30.0, 30.0],
        Rarity::Legendary => [100.0, 30.0, 30.0, 50.0, 30.0],
    };
    let candidate_table = vec![
        (ItemCandidate::Heal, candidate_weight[0]),
        (ItemCandidate::Lottery, candidate_weight[1]),
        (ItemCandidate::ExtraReroll, candidate_weight[2]),
        (ItemCandidate::Shield, candidate_weight[3]),
        (ItemCandidate::DamageReduction, candidate_weight[4]),
    ];
    candidate_table
}

enum ItemCandidate {
    Heal,
    Lottery,
    ExtraReroll,
    Shield,
    DamageReduction,
}
