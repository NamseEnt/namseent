use super::{STRAIGHT_FLUSH, evaluate};
use crate::UpgradeKind;

#[test]
fn test_straight_flush() {
    let template = evaluate(&[(1, 7), (1, 8), (1, 9), (1, 10), (1, 11)], &[]);
    assert_eq!(template.kind, STRAIGHT_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(11));
}

#[test]
fn test_straight_flush_4cards_without_upgrade() {
    let template = evaluate(&[(1, 8), (1, 9), (1, 10), (1, 11)], &[]);
    assert_ne!(template.kind, STRAIGHT_FLUSH);
}

#[test]
fn test_straight_flush_4cards_with_upgrade() {
    let template = evaluate(
        &[(1, 7), (1, 8), (1, 9), (1, 10)],
        &[UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, STRAIGHT_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(10));
}

#[test]
fn test_straight_flush_skip_rank() {
    let template = evaluate(&[(1, 8), (1, 9), (1, 10), (1, 12)], &[UpgradeKind::Rabbit]);
    assert_eq!(template.kind, super::HIGH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_straight_flush_treat_suits_as_same() {
    let template = evaluate(
        &[(1, 7), (2, 8), (1, 9), (2, 10), (1, 11)],
        &[UpgradeKind::BlackWhite],
    );
    assert_eq!(template.kind, STRAIGHT_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(11));
}

#[test]
fn test_straight_flush_treat_suits_as_same_and_shorten_4cards() {
    let template = evaluate(
        &[(1, 7), (2, 8), (1, 9), (2, 10)],
        &[UpgradeKind::BlackWhite, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, STRAIGHT_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(10));
}

#[test]
fn test_straight_and_flush_must_use_the_same_cards() {
    let template = evaluate(
        &[(1, 0), (1, 1), (1, 2), (1, 7), (0, 3)],
        &[UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, super::FLUSH);
    assert_eq!(template.used_cards.len(), 4);
}
