use super::*;

#[test]
fn test_two_pair() {
    let template = evaluate(
        &[
            (SPADES, ACE),
            (HEARTS, ACE),
            (CLUBS, TEN),
            (DIAMONDS, TEN),
            (SPADES, SEVEN),
        ],
        &[],
    );
    assert_eq!(template.kind, TWO_PAIR);
    assert_eq!(template.rank, Some(12));
}
