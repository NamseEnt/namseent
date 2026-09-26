use super::*;
use crate::UpgradeKind;

#[test]
fn test_flush() {
    let template = evaluate(
        &[
            (SPADES, SEVEN),
            (SPADES, EIGHT),
            (SPADES, NINE),
            (SPADES, TEN),
            (SPADES, QUEEN),
        ],
        &[],
    );
    assert_eq!(template.kind, FLUSH);
    assert_eq!(template.suit, Some(0));
    assert_eq!(template.rank, Some(10));
}

#[test]
fn test_flush_4cards_without_upgrade() {
    let template = evaluate(
        &[
            (SPADES, SEVEN),
            (SPADES, EIGHT),
            (SPADES, NINE),
            (SPADES, TEN),
        ],
        &[],
    );
    assert_ne!(template.kind, FLUSH);
}

#[test]
fn test_flush_4cards_with_upgrade() {
    let template = evaluate(
        &[
            (SPADES, SEVEN),
            (SPADES, EIGHT),
            (SPADES, NINE),
            (SPADES, JACK),
        ],
        &[UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, FLUSH);
    assert_eq!(template.suit, Some(0));
    assert_eq!(template.rank, Some(9));
}

#[test]
fn test_flush_treat_suits_as_same() {
    let template = evaluate(
        &[
            (SPADES, SEVEN),
            (CLUBS, EIGHT),
            (SPADES, NINE),
            (CLUBS, TEN),
            (SPADES, QUEEN),
        ],
        &[UpgradeKind::BlackWhite],
    );
    assert_eq!(template.kind, FLUSH);
    assert!(matches!(template.suit, Some(0 | 1)));
    assert_eq!(template.rank, Some(10));
}

#[test]
fn test_flush_treat_suits_as_same_and_shorten_4cards() {
    let template = evaluate(
        &[
            (SPADES, SEVEN),
            (CLUBS, EIGHT),
            (SPADES, NINE),
            (CLUBS, JACK),
        ],
        &[UpgradeKind::BlackWhite, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, FLUSH);
    assert!(matches!(template.suit, Some(0 | 1)));
    assert_eq!(template.rank, Some(9));
}
