mod candidate_table;

use self::candidate_table::{
    ItemRarityWeights, generate_item_candidate_table, generate_item_rarity_candidate_table,
};
use super::Item;
use crate::Rarity;
use rand::{Rng, seq::SliceRandom, thread_rng};

/// 외부에서 RNG를 주입할 수 있는 아이템 생성 함수 (테스트/결정성 보장 목적)
pub fn generate_item_with_rng<R: Rng>(rng: &mut R) -> Item {
    let candidates = generate_item_candidate_table(ItemRarityWeights::shop());
    let candidate = candidates
        .choose_weighted(rng, |(_, weight)| *weight)
        .expect("item candidate table should not be empty")
        .0;

    candidate.generate(rng)
}

pub fn generate_item_of_rarity_with_rng<R: Rng>(rarity: Rarity, rng: &mut R) -> Item {
    let candidates = generate_item_rarity_candidate_table(rarity);
    let candidate = candidates
        .choose(rng)
        .expect("item rarity should have at least one candidate");

    candidate.generate(rng)
}

/// 기존 외부 API: thread_rng() 사용 (기존 호출 코드 호환성 유지)
#[allow(dead_code)]
pub fn generate_item() -> Item {
    let mut rng = thread_rng();
    generate_item_with_rng(&mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::{
        card::{Card, RANKS, Rank, SUITS, Suit},
        item::{GrantCardItem, ItemDiscriminants},
    };
    use rand::{SeedableRng, rngs::StdRng};
    use strum::IntoEnumIterator;

    #[test]
    fn grant_card_item_constructor_preserves_card() {
        let card = Card::new(Rank::Queen, Suit::Hearts);

        let item = GrantCardItem::new(card).into_item();

        assert_eq!(
            item,
            crate::game_state::item::Item::GrantCard(GrantCardItem { card })
        );
    }

    #[test]
    fn generate_item_with_rng_stays_in_valid_card_range() {
        let mut rng = StdRng::seed_from_u64(7);

        for _ in 0..128 {
            let item = generate_item_with_rng(&mut rng);
            if let crate::game_state::item::Item::GrantCard(GrantCardItem { card }) = item {
                assert!(SUITS.contains(&card.suit));
                assert!(RANKS.contains(&card.rank));
            }
        }
    }

    #[test]
    fn every_definition_generates_its_own_item_variant() {
        let mut rng = StdRng::seed_from_u64(11);

        for discriminant in ItemDiscriminants::iter() {
            let item = discriminant.generate(&mut rng);
            assert_eq!(item.discriminant(), discriminant);
        }
    }

    #[test]
    fn generating_common_item_by_rarity_returns_common() {
        let mut rng = StdRng::seed_from_u64(20);

        for _ in 0..16 {
            let item = generate_item_of_rarity_with_rng(crate::Rarity::Common, &mut rng);
            assert_eq!(item.discriminant().rarity(), crate::Rarity::Common);
        }
    }
    #[test]
    fn generating_rare_item_by_rarity_returns_rare() {
        let mut rng = StdRng::seed_from_u64(21);

        for _ in 0..16 {
            let item = generate_item_of_rarity_with_rng(crate::Rarity::Rare, &mut rng);
            assert_eq!(item.discriminant().rarity(), crate::Rarity::Rare);
        }
    }
}
