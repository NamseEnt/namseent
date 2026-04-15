use super::Item;
use crate::card::Card;
use crate::config::GameConfig;
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
pub fn generate_item_with_rng<R: Rng + ?Sized>(rng: &mut R, config: &GameConfig) -> Item {
    let candidates = generate_item_candidate_table(config);
    let candidate = &candidates
        .choose_weighted(rng, |x| x.1)
        .expect("item candidate table should not be empty")
        .0;

    let value = rng.gen_range(0.0..1.0);

    let (kind, effect) = match candidate {
        ItemCandidate::Heal => {
            let range = config.items.heal.min_value..config.items.heal.max_value;
            let amount = calculate_amount_from_value(value, range.start, range.end);
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
            let range = config.items.shield.min_value..config.items.shield.max_value;
            let amount = calculate_amount_from_value(value, range.start, range.end);
            (
                crate::game_state::item::ItemKind::Shield,
                Effect::Shield { amount },
            )
        }
        ItemCandidate::DamageReduction => {
            let range =
                config.items.damage_reduction.min_value..config.items.damage_reduction.max_value;
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
            let range =
                config.items.grant_barricades.min_value..config.items.grant_barricades.max_value;
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
#[allow(dead_code)]
pub fn generate_item() -> Item {
    let mut rng = thread_rng();
    generate_item_with_rng(&mut rng, &GameConfig::default_config())
}

fn generate_item_candidate_table(config: &GameConfig) -> Vec<(ItemCandidate, f32)> {
    let candidate_table = vec![
        (ItemCandidate::Heal, config.items.heal.weight),
        (ItemCandidate::ExtraReroll, config.items.extra_reroll.weight),
        (ItemCandidate::Shield, config.items.shield.weight),
        (
            ItemCandidate::DamageReduction,
            config.items.damage_reduction.weight,
        ),
        (
            ItemCandidate::GrantBarricades,
            config.items.grant_barricades.weight,
        ),
        (ItemCandidate::GrantCard, config.items.grant_card.weight),
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
            let item = generate_item_with_rng(&mut rng, &GameConfig::default_config());
            if let crate::game_state::item::ItemKind::GrantCard { card } = item.kind {
                assert!(crate::card::SUITS.contains(&card.suit));
                assert!(crate::card::RANKS.contains(&card.rank));
                assert_eq!(item.effect, Effect::AddCardToHand { card });
            }
        }
    }
}
