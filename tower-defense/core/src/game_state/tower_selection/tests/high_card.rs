use super::*;

#[test]
fn test_high_card() {
    let template = evaluate(
        &[
            (SPADES, ACE),
            (HEARTS, TEN),
            (CLUBS, SEVEN),
            (DIAMONDS, NINE),
            (SPADES, EIGHT),
        ],
        &[],
    );
    assert_eq!(template.kind, HIGH);
    assert_eq!(template.rank, Some(12));
}
