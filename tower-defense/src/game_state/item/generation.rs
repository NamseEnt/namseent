use super::Item;
use crate::card::Card;
use crate::game_state::effect::Effect;
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

/// 외부에서 RNG를 주입할 수 있는 아이템 생성 함수 (테스트/결정성 보장 목적)
pub fn generate_item_with_rng<R: Rng + ?Sized>(rng: &mut R) -> Item {
    let candidates = generate_item_candidate_table();
    let candidate = &candidates
        .choose_weighted(rng, |x| x.1)
        .expect("item candidate table should not be empty")
        .0;

    let value = rng.gen_range(0.0..1.0);

    let (kind, effect) = match candidate {
        ItemCandidate::Heal => {
            let range = 10.0..14.0;
            let amount = calculate_amount_from_value(value, range.start, range.end);
            (
                crate::game_state::item::ItemKind::RiceCake,
                Effect::Heal { amount },
            )
        }
        ItemCandidate::ExtraReroll => (
            crate::game_state::item::ItemKind::EmergencyDice,
            Effect::ExtraDice,
        ),
        ItemCandidate::Shield => {
            let range = 15.0..25.0;
            let amount = calculate_amount_from_value(value, range.start, range.end);
            (
                crate::game_state::item::ItemKind::Shield,
                Effect::Shield { amount },
            )
        }
        ItemCandidate::DamageReduction => {
            let range = 0.8..0.85;
            let amount = calculate_reverse_amount_from_value(value, range.start, range.end);
            let duration = Duration::from_secs(4);
            (
                crate::game_state::item::ItemKind::Painkiller,
                Effect::UserDamageReduction {
                    multiply: amount,
                    duration,
                },
            )
        }
        ItemCandidate::GrantBarricades => {
            let range = 5.0..10.0;
            let count = calculate_amount_from_value(value, range.start, range.end) as usize;
            (
                crate::game_state::item::ItemKind::GrantBarricades,
                Effect::AddTowerCardToPlacementHand {
                    tower_kind: crate::game_state::tower::TowerKind::Barricade,
                    suit: crate::card::Suit::Spades,
                    rank: crate::card::Rank::Ace,
                    count,
                },
            )
        }
        ItemCandidate::GrantCard => {
            let card = Card::new_random();
            (
                crate::game_state::item::ItemKind::GrantCard { card },
                Effect::AddCardToHand { card },
            )
        }
    };

    Item {
        kind,
        effect,
        value: value.into(),
    }
}

/// 기존 외부 API: thread_rng() 사용 (기존 호출 코드 호환성 유지)
pub fn generate_item() -> Item {
    let mut rng = thread_rng();
    generate_item_with_rng(&mut rng)
}

fn generate_item_candidate_table() -> Vec<(ItemCandidate, f32)> {
    let candidate_weight = [100.0, 10.0, 10.0, 10.0, 45.0, 35.0];
    let candidate_table = vec![
        (ItemCandidate::Heal, candidate_weight[0]),
        (ItemCandidate::ExtraReroll, candidate_weight[1]),
        (ItemCandidate::Shield, candidate_weight[2]),
        (ItemCandidate::DamageReduction, candidate_weight[3]),
        (ItemCandidate::GrantBarricades, candidate_weight[4]),
        (ItemCandidate::GrantCard, candidate_weight[5]),
    ];
    candidate_table
}

enum ItemCandidate {
    Heal,
    ExtraReroll,
    Shield,
    DamageReduction,
    GrantBarricades,
    GrantCard,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn grant_card_item_constructor_preserves_card() {
        let card = Card {
            suit: Suit::Hearts,
            rank: Rank::Queen,
        };

        let item = Item::grant_card(card);

        assert_eq!(
            item.kind,
            crate::game_state::item::ItemKind::GrantCard { card }
        );
        assert_eq!(item.effect, Effect::AddCardToHand { card });
    }

    #[test]
    fn generate_item_with_rng_stays_in_valid_card_range() {
        let mut rng = StdRng::seed_from_u64(7);

        for _ in 0..128 {
            let item = generate_item_with_rng(&mut rng);
            if let crate::game_state::item::ItemKind::GrantCard { card } = item.kind {
                assert!(crate::card::SUITS.contains(&card.suit));
                assert!(crate::card::RANKS.contains(&card.rank));
                assert_eq!(item.effect, Effect::AddCardToHand { card });
            }
        }
    }
}
