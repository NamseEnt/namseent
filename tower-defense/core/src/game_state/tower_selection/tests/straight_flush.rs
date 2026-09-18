use super::*;
use crate::UpgradeKind;

#[test]
fn test_straight_flush() {
    let template = evaluate(
        &[
            (HEARTS, NINE),
            (HEARTS, TEN),
            (HEARTS, JACK),
            (HEARTS, QUEEN),
            (HEARTS, KING),
        ],
        &[],
    );
    assert_eq!(template.kind, STRAIGHT_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(11));
}

#[test]
fn test_straight_flush_4cards_without_upgrade() {
    let template = evaluate(
        &[
            (HEARTS, TEN),
            (HEARTS, JACK),
            (HEARTS, QUEEN),
            (HEARTS, KING),
        ],
        &[],
    );
    assert_ne!(template.kind, STRAIGHT_FLUSH);
}

#[test]
fn test_straight_flush_4cards_with_upgrade() {
    let template = evaluate(
        &[
            (HEARTS, NINE),
            (HEARTS, TEN),
            (HEARTS, JACK),
            (HEARTS, QUEEN),
        ],
        &[UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, STRAIGHT_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(10));
}

#[test]
fn test_straight_flush_skip_rank() {
    let template = evaluate(
        &[
            (HEARTS, TEN),
            (HEARTS, JACK),
            (HEARTS, QUEEN),
            (HEARTS, ACE),
        ],
        &[UpgradeKind::Rabbit],
    );
    assert_eq!(template.kind, super::HIGH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_straight_flush_treat_suits_as_same() {
    let template = evaluate(
        &[
            (HEARTS, NINE),
            (DIAMONDS, TEN),
            (HEARTS, JACK),
            (DIAMONDS, QUEEN),
            (HEARTS, KING),
        ],
        &[UpgradeKind::BlackWhite],
    );
    assert_eq!(template.kind, STRAIGHT_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(11));
}

#[test]
fn test_straight_flush_treat_suits_as_same_and_shorten_4cards() {
    let template = evaluate(
        &[
            (HEARTS, NINE),
            (DIAMONDS, TEN),
            (HEARTS, JACK),
            (DIAMONDS, QUEEN),
        ],
        &[UpgradeKind::BlackWhite, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, STRAIGHT_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(10));
}

#[test]
fn test_straight_and_flush_must_use_the_same_cards() {
    let template = evaluate(
        &[
            (HEARTS, TWO),
            (HEARTS, THREE),
            (HEARTS, FOUR),
            (HEARTS, NINE),
            (SPADES, FIVE),
        ],
        &[UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, super::FLUSH);
    assert_eq!(template.used_cards.len(), 4);
}
