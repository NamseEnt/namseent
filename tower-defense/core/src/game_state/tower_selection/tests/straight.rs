use super::{STRAIGHT, evaluate};
use crate::UpgradeKind;

#[test]
fn test_straight() {
    let template = evaluate(&[(0, 5), (1, 6), (3, 7), (2, 8), (0, 9)], &[]);
    assert_eq!(template.kind, STRAIGHT);
    assert_eq!(template.rank, Some(9));
}

#[test]
fn test_straight_4cards_without_upgrade() {
    let template = evaluate(&[(0, 5), (1, 6), (3, 7), (2, 8)], &[]);
    assert_ne!(template.kind, STRAIGHT);
}

#[test]
fn test_straight_4cards_with_upgrade() {
    let template = evaluate(
        &[(0, 5), (1, 6), (3, 7), (2, 8)],
        &[UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, STRAIGHT);
    assert_eq!(template.rank, Some(8));
}

#[test]
fn test_straight_skip_rank() {
    let template = evaluate(&[(0, 5), (1, 6), (3, 7), (2, 9)], &[UpgradeKind::Rabbit]);
    assert_eq!(template.kind, super::HIGH);
    assert_eq!(template.rank, Some(9));
}

#[test]
fn test_straight_skip_rank_and_shorten_4cards() {
    let template = evaluate(
        &[(0, 5), (1, 6), (3, 9), (2, 7)],
        &[UpgradeKind::Rabbit, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, STRAIGHT);
    assert_eq!(template.rank, Some(9));
}
