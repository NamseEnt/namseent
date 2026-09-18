use super::*;

#[test]
fn test_three_of_a_kind() {
    let template = evaluate(
        &[
            (SPADES, ACE),
            (HEARTS, ACE),
            (CLUBS, ACE),
            (DIAMONDS, TEN),
            (SPADES, SEVEN),
        ],
        &[],
    );
    assert_eq!(template.kind, THREE_OF_A_KIND);
    assert_eq!(template.rank, Some(12));
}
