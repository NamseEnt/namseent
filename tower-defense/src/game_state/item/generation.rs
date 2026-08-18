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

pub fn generate_item_of_rarity_with_rng<R: Rng>(rarity: Rarity, rng: &mut R) -> Option<Item> {
    let candidates = generate_item_rarity_candidate_table(rarity);
    let candidate = candidates.choose(rng)?;

    Some(candidate.generate(rng))
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
    use crate::game_state::item::ItemDiscriminants;
    use rand::{SeedableRng, rngs::StdRng};
    use strum::IntoEnumIterator;

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
            let item = generate_item_of_rarity_with_rng(crate::Rarity::Common, &mut rng)
                .expect("common item candidate");
            assert_eq!(item.discriminant().rarity(), crate::Rarity::Common);
        }
    }
    #[test]
    fn generating_rare_item_by_rarity_returns_rare() {
        let mut rng = StdRng::seed_from_u64(21);

        for _ in 0..16 {
            let item = generate_item_of_rarity_with_rng(crate::Rarity::Rare, &mut rng)
                .expect("rare item candidate");
            assert_eq!(item.discriminant().rarity(), crate::Rarity::Rare);
        }
    }
    #[test]
    fn generating_epic_item_by_rarity_returns_epic() {
        let mut rng = StdRng::seed_from_u64(22);

        for _ in 0..16 {
            let item = generate_item_of_rarity_with_rng(crate::Rarity::Epic, &mut rng)
                .expect("epic item candidate");
            assert_eq!(item.discriminant().rarity(), crate::Rarity::Epic);
        }
    }
    #[test]
    fn generating_unrepresented_item_rarity_returns_none() {
        let mut rng = StdRng::seed_from_u64(23);

        assert!(generate_item_of_rarity_with_rng(crate::Rarity::Legendary, &mut rng).is_none());
    }
}
