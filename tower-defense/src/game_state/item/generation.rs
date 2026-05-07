use super::Item;
use crate::card::Card;
use crate::game_state::effect::Effect;
use namui::*;
use rand::{Rng, seq::SliceRandom, thread_rng};

/// 외부에서 RNG를 주입할 수 있는 아이템 생성 함수 (테스트/결정성 보장 목적)
pub fn generate_item_with_rng<R: Rng + ?Sized>(rng: &mut R) -> Item {
    let candidates = generate_item_candidate_table();
    let candidate = &candidates
        .choose_weighted(rng, |x| x.1)
        .expect("item candidate table should not be empty")
        .0;

    let (kind, effect) = match candidate {
        ItemCandidate::Heal => {
            let amount = 14.0;
            (
                crate::game_state::item::ItemKind::RiceBall,
                Effect::Heal { amount },
            )
        }
        ItemCandidate::ExtraReroll => (
            crate::game_state::item::ItemKind::LumpSugar,
            Effect::ExtraDice,
        ),
        ItemCandidate::Shield => {
            let amount = 25.0;
            (
                crate::game_state::item::ItemKind::Shield,
                Effect::Shield { amount },
            )
        }
        ItemCandidate::DamageReduction => {
            let amount = 0.85;
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
            let count = 10;
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

    Item { kind, effect }
}

/// 기존 외부 API: thread_rng() 사용 (기존 호출 코드 호환성 유지)
#[allow(dead_code)]
pub fn generate_item() -> Item {
    let mut rng = thread_rng();
    generate_item_with_rng(&mut rng)
}

fn generate_item_candidate_table() -> Vec<(ItemCandidate, f32)> {
    vec![
        (ItemCandidate::Heal, 100.0),
        (ItemCandidate::ExtraReroll, 10.0),
        (ItemCandidate::Shield, 10.0),
        (ItemCandidate::DamageReduction, 10.0),
        (ItemCandidate::GrantBarricades, 45.0),
        (ItemCandidate::GrantCard, 35.0),
    ]
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
