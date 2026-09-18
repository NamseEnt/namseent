use super::{FLUSH, evaluate};
use crate::UpgradeKind;

#[test]
fn test_flush() {
    let template = evaluate(&[(0, 5), (0, 6), (0, 7), (0, 8), (0, 10)], &[]);
    assert_eq!(template.kind, FLUSH);
    assert_eq!(template.suit, Some(0));
    assert_eq!(template.rank, Some(10));
}

#[test]
fn test_flush_4cards_without_upgrade() {
    let template = evaluate(&[(0, 5), (0, 6), (0, 7), (0, 8)], &[]);
    assert_ne!(template.kind, FLUSH);
}

#[test]
fn test_flush_4cards_with_upgrade() {
    let template = evaluate(
        &[(0, 5), (0, 6), (0, 7), (0, 9)],
        &[UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, FLUSH);
    assert_eq!(template.suit, Some(0));
    assert_eq!(template.rank, Some(9));
}

#[test]
fn test_flush_treat_suits_as_same() {
    let template = evaluate(
        &[(0, 5), (3, 6), (0, 7), (3, 8), (0, 10)],
        &[UpgradeKind::BlackWhite],
    );
    assert_eq!(template.kind, FLUSH);
    assert!(matches!(template.suit, Some(0 | 1)));
    assert_eq!(template.rank, Some(10));
}

#[test]
fn test_flush_treat_suits_as_same_and_shorten_4cards() {
    let template = evaluate(
        &[(0, 5), (3, 6), (0, 7), (3, 9)],
        &[UpgradeKind::BlackWhite, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, FLUSH);
    assert!(matches!(template.suit, Some(0 | 1)));
    assert_eq!(template.rank, Some(9));
}
