use super::Item;
use crate::game_state::{
    card::{Card, RANKS, SUITS},
    item::{
        GrantBarricadesItem, GrantCardItem, ItemDiscriminants, LumpSugarItem, RiceBallItem,
        ShieldItem,
    },
};
use rand::{Rng, seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;

/// 외부에서 RNG를 주입할 수 있는 아이템 생성 함수 (테스트/결정성 보장 목적)
pub fn generate_item_with_rng<R: Rng + ?Sized>(rng: &mut R) -> Item {
    let candidates = generate_item_candidate_table();
    let candidate = candidates
        .choose_weighted(rng, |x| x.1)
        .expect("item candidate table should not be empty")
        .0;

    generate_item_from_discriminant(candidate, rng)
}

/// 기존 외부 API: thread_rng() 사용 (기존 호출 코드 호환성 유지)
#[allow(dead_code)]
pub fn generate_item() -> Item {
    let mut rng = thread_rng();
    generate_item_with_rng(&mut rng)
}

fn generate_item_candidate_table() -> Vec<(ItemDiscriminants, f32)> {
    ItemDiscriminants::iter()
        .map(|item| (item, item_generation_weight(item)))
        .collect()
}

fn item_generation_weight(item: ItemDiscriminants) -> f32 {
    match item {
        ItemDiscriminants::RiceBall => 100.0,
        ItemDiscriminants::LumpSugar => 10.0,
        ItemDiscriminants::Shield => 10.0,
        ItemDiscriminants::GrantBarricades => 45.0,
        ItemDiscriminants::GrantCard => 35.0,
    }
}

fn generate_item_from_discriminant<R: Rng + ?Sized>(item: ItemDiscriminants, rng: &mut R) -> Item {
    match item {
        ItemDiscriminants::RiceBall => RiceBallItem::standard().into_item(),
        ItemDiscriminants::LumpSugar => LumpSugarItem::standard().into_item(),
        ItemDiscriminants::Shield => ShieldItem::standard().into_item(),
        ItemDiscriminants::GrantBarricades => GrantBarricadesItem::standard().into_item(),
        ItemDiscriminants::GrantCard => generate_grant_card_item(rng),
    }
}

fn generate_grant_card_item<R: Rng + ?Sized>(rng: &mut R) -> Item {
    let suit = SUITS[rng.gen_range(0..SUITS.len())];
    let rank = RANKS[rng.gen_range(0..RANKS.len())];
    let card = Card::new(rank, suit);
    GrantCardItem::new(card).into_item()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::{
        card::{Rank, Suit},
        item::GrantCardItem,
    };
    use rand::{SeedableRng, rngs::StdRng};

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
}
