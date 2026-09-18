use super::{FULL_HOUSE, evaluate};

#[test]
fn test_full_house() {
    let template = evaluate(&[(0, 12), (1, 12), (3, 12), (2, 8), (0, 8)], &[]);
    assert_eq!(template.kind, FULL_HOUSE);
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_two_triples_form_full_house() {
    let template = evaluate(&[(0, 11), (1, 11), (2, 11), (0, 7), (1, 7), (2, 7)], &[]);
    assert_eq!(template.kind, FULL_HOUSE);
    assert_eq!(template.rank, Some(11));
    assert_eq!(
        template
            .used_cards
            .iter()
            .filter(|card| card.rank == 11)
            .count(),
        3
    );
    assert_eq!(
        template
            .used_cards
            .iter()
            .filter(|card| card.rank == 7)
            .count(),
        2
    );
}
