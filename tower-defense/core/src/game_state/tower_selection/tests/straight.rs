use super::*;
use crate::UpgradeKind;

#[test]
fn test_straight() {
    let template = evaluate(
        &[
            (SPADES, SEVEN),
            (HEARTS, EIGHT),
            (CLUBS, NINE),
            (DIAMONDS, TEN),
            (SPADES, JACK),
        ],
        &[],
    );
    assert_eq!(template.kind, STRAIGHT);
    assert_eq!(template.rank, Some(9));
}

#[test]
fn test_straight_4cards_without_upgrade() {
    let template = evaluate(
        &[
            (SPADES, SEVEN),
            (HEARTS, EIGHT),
            (CLUBS, NINE),
            (DIAMONDS, TEN),
        ],
        &[],
    );
    assert_ne!(template.kind, STRAIGHT);
}

#[test]
fn test_straight_4cards_with_upgrade() {
    let template = evaluate(
        &[
            (SPADES, SEVEN),
            (HEARTS, EIGHT),
            (CLUBS, NINE),
            (DIAMONDS, TEN),
        ],
        &[UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, STRAIGHT);
    assert_eq!(template.rank, Some(8));
}

#[test]
fn test_straight_skip_rank() {
    let template = evaluate(
        &[
            (SPADES, SEVEN),
            (HEARTS, EIGHT),
            (CLUBS, NINE),
            (DIAMONDS, JACK),
        ],
        &[UpgradeKind::Rabbit],
    );
    assert_eq!(template.kind, super::HIGH);
    assert_eq!(template.rank, Some(9));
}

#[test]
fn test_straight_skip_rank_and_shorten_4cards() {
    let template = evaluate(
        &[
            (SPADES, SEVEN),
            (HEARTS, EIGHT),
            (CLUBS, JACK),
            (DIAMONDS, NINE),
        ],
        &[UpgradeKind::Rabbit, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, STRAIGHT);
    assert_eq!(template.rank, Some(9));
}
