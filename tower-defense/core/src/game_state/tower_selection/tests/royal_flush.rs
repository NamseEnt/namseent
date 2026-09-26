use super::*;
use crate::UpgradeKind;

#[test]
fn test_royal_flush() {
    let template = evaluate(
        &[
            (HEARTS, TEN),
            (HEARTS, JACK),
            (HEARTS, QUEEN),
            (HEARTS, KING),
            (HEARTS, ACE),
        ],
        &[],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_4cards_without_upgrade() {
    let template = evaluate(
        &[
            (HEARTS, JACK),
            (HEARTS, QUEEN),
            (HEARTS, KING),
            (HEARTS, ACE),
        ],
        &[],
    );
    assert_ne!(template.kind, ROYAL_FLUSH);
}

#[test]
fn test_royal_flush_4cards_with_upgrade() {
    let template = evaluate(
        &[
            (HEARTS, JACK),
            (HEARTS, QUEEN),
            (HEARTS, KING),
            (HEARTS, ACE),
        ],
        &[UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_skip_rank() {
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
fn test_royal_flush_skip_rank_and_shorten_4cards() {
    let template = evaluate(
        &[
            (HEARTS, TEN),
            (HEARTS, QUEEN),
            (HEARTS, ACE),
            (HEARTS, JACK),
        ],
        &[UpgradeKind::Rabbit, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_treat_suits_as_same() {
    let template = evaluate(
        &[
            (HEARTS, TEN),
            (DIAMONDS, JACK),
            (HEARTS, QUEEN),
            (DIAMONDS, KING),
            (HEARTS, ACE),
        ],
        &[UpgradeKind::BlackWhite],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_treat_suits_as_same_and_shorten_4cards() {
    let template = evaluate(
        &[
            (DIAMONDS, JACK),
            (DIAMONDS, QUEEN),
            (HEARTS, KING),
            (HEARTS, ACE),
        ],
        &[UpgradeKind::BlackWhite, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_ten_through_king_with_treat_suits_as_same_and_shorten_4cards() {
    let template = evaluate(
        &[
            (HEARTS, TEN),
            (DIAMONDS, JACK),
            (HEARTS, QUEEN),
            (DIAMONDS, KING),
        ],
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
            .all(|card| matches!(card.suit, crate::Suit::Hearts | crate::Suit::Diamonds))
    );
}

#[test]
fn test_royal_flush_prefers_ten_through_king_over_lower_four_card_straight() {
    let template = evaluate(
        &[
            (HEARTS, NINE),
            (DIAMONDS, TEN),
            (HEARTS, JACK),
            (DIAMONDS, QUEEN),
            (HEARTS, KING),
        ],
        &[UpgradeKind::BlackWhite, UpgradeKind::FourLeafClover],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_royal_flush_treat_suits_as_same_and_shorten_4cards_and_skip_rank_for_straight() {
    let template = evaluate(
        &[
            (HEARTS, TEN),
            (DIAMONDS, JACK),
            (DIAMONDS, QUEEN),
            (HEARTS, ACE),
        ],
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
        &[(HEARTS, TEN), (HEARTS, JACK), (HEARTS, KING), (HEARTS, ACE)],
        &[UpgradeKind::FourLeafClover, UpgradeKind::Rabbit],
    );
    assert_eq!(template.kind, ROYAL_FLUSH);
    assert_eq!(template.suit, Some(1));
    assert_eq!(template.rank, Some(12));
}
