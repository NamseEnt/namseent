use super::*;

#[test]
fn test_full_house() {
    let template = evaluate(
        &[
            (SPADES, ACE),
            (HEARTS, ACE),
            (CLUBS, ACE),
            (DIAMONDS, TEN),
            (SPADES, TEN),
        ],
        &[],
    );
    assert_eq!(template.kind, FULL_HOUSE);
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_two_triples_form_full_house() {
    let template = evaluate(
        &[
            (SPADES, KING),
            (HEARTS, KING),
            (DIAMONDS, KING),
            (SPADES, NINE),
            (HEARTS, NINE),
            (DIAMONDS, NINE),
        ],
        &[],
    );
    assert_eq!(template.kind, FULL_HOUSE);
    assert_eq!(template.rank, Some(11));
    assert_eq!(
        template
            .used_cards
            .iter()
            .filter(|card| card.rank == crate::Rank::King)
            .count(),
        3
    );
    assert_eq!(
        template
            .used_cards
            .iter()
            .filter(|card| card.rank == crate::Rank::Nine)
            .count(),
        2
    );
}
