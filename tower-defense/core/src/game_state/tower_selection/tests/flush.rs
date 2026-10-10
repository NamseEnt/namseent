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

#[test]
fn black_white_can_be_disabled_without_removing_the_treasure() {
    let cards = cards(&[
        (SPADES, SEVEN),
        (CLUBS, EIGHT),
        (SPADES, NINE),
        (CLUBS, TEN),
        (SPADES, QUEEN),
    ]);
    let upgrades =
        UpgradeCollection::from_entries(vec![crate::generated_upgrade(UpgradeKind::BlackWhite)], 0);
    let mut config = config();
    assert_eq!(
        get_highest_tower_template(&cards, &upgrades, &config, 0)
            .unwrap()
            .kind,
        FLUSH
    );
    config.treasures.black_white_enabled = false;
    let disabled = get_highest_tower_template(&cards, &upgrades, &config, 0).unwrap();
    let absent = get_highest_tower_template(
        &cards,
        &UpgradeCollection::from_entries(vec![], 0),
        &config,
        0,
    )
    .unwrap();
    assert_eq!(disabled, absent);
    assert_eq!(upgrades.entries()[0].kind(), UpgradeKind::BlackWhite);
}
