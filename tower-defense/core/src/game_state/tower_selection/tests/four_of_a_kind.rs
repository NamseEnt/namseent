use super::*;

#[test]
fn test_four_of_a_kind() {
    let template = evaluate(
        &[
            (SPADES, ACE),
            (HEARTS, ACE),
            (CLUBS, ACE),
            (DIAMONDS, ACE),
            (SPADES, SEVEN),
        ],
        &[],
    );
    assert_eq!(template.kind, FOUR_OF_A_KIND);
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_five_of_a_kind_is_treated_as_four_of_a_kind() {
    let template = evaluate(
        &[
            (SPADES, ACE),
            (HEARTS, ACE),
            (CLUBS, ACE),
            (DIAMONDS, ACE),
            (SPADES, ACE),
        ],
        &[],
    );
    assert_eq!(template.kind, FOUR_OF_A_KIND);
    assert_eq!(template.rank, Some(12));
}
