use super::*;

#[test]
fn test_one_pair() {
    let template = evaluate(
        &[
            (SPADES, ACE),
            (HEARTS, ACE),
            (CLUBS, SEVEN),
            (DIAMONDS, NINE),
            (SPADES, EIGHT),
        ],
        &[],
    );
    assert_eq!(template.kind, ONE_PAIR);
    assert_eq!(template.rank, Some(12));
}
