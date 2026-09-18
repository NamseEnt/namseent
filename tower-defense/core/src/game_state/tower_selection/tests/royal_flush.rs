use super::{ROYAL_FLUSH, evaluate};
use crate::UpgradeKind;

#[test]
fn test_royal_flush() {
    let template = evaluate(&[(1, 8), (1, 9), (1, 10), (1, 11), (1, 12)], &[]);
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_4cards_without_upgrade() {
    let template = evaluate(&[(1, 9), (1, 10), (1, 11), (1, 12)], &[]);
    assert_ne!(template.kind, ROYAL_FLUSH);
}

#[test]
fn test_royal_flush_4cards_with_upgrade() {
    let template = evaluate(
        &[(1, 9), (1, 10), (1, 11), (1, 12)],
        &[UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_skip_rank() {
    let template = evaluate(&[(1, 8), (1, 9), (1, 10), (1, 12)], &[UpgradeKind::Rabbit]);
    assert_eq!(template.kind, super::HIGH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_skip_rank_and_shorten_4cards() {
    let template = evaluate(
        &[(1, 8), (1, 10), (1, 12), (1, 9)],
        &[UpgradeKind::Rabbit, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_treat_suits_as_same() {
    let template = evaluate(
        &[(1, 8), (2, 9), (1, 10), (2, 11), (1, 12)],
        &[UpgradeKind::BlackWhite],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_treat_suits_as_same_and_shorten_4cards() {
    let template = evaluate(
        &[(2, 9), (2, 10), (1, 11), (1, 12)],
        &[UpgradeKind::BlackWhite, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_ten_through_king_with_treat_suits_as_same_and_shorten_4cards() {
    let template = evaluate(
        &[(1, 8), (2, 9), (1, 10), (2, 11)],
        &[UpgradeKind::BlackWhite, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
    assert_eq!(template.used_cards.len(), 4);
    assert!(
        template
            .used_cards
            .iter()
            .all(|card| matches!(card.suit, 1 | 2))
    );
}

#[test]
fn test_royal_flush_prefers_ten_through_king_over_lower_four_card_straight() {
    let template = evaluate(
        &[(1, 7), (2, 8), (1, 9), (2, 10), (1, 11)],
        &[UpgradeKind::BlackWhite, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_treat_suits_as_same_and_shorten_4cards_and_skip_rank_for_straight() {
    let template = evaluate(
        &[(1, 8), (2, 9), (2, 10), (1, 12)],
        &[
            UpgradeKind::BlackWhite,
            UpgradeKind::FourLeafClover,
            UpgradeKind::Rabbit,
        ],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_ten_jack_king_ace_with_skip_rank() {
    let template = evaluate(
        &[(1, 8), (1, 9), (1, 11), (1, 12)],
        &[UpgradeKind::FourLeafClover, UpgradeKind::Rabbit],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}
