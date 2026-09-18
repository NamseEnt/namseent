use super::*;

#[test]
fn raw_selection_matches_basic_poker_tower_kinds() {
    let template = evaluate(
        &[
            (SPADES, ACE),
            (HEARTS, ACE),
            (DIAMONDS, SEVEN),
            (CLUBS, NINE),
            (SPADES, EIGHT),
        ],
        &[],
    );
    assert_eq!(template.kind, ONE_PAIR);
    assert_eq!(template.rank, Some(12));
}

#[test]
fn raw_template_derives_overcharge_interval_and_preserves_card_payload() {
    let mut overcharge = cards(&[(SPADES, ACE)]);
    overcharge[0].engraving = Some(1);
    let template = get_highest_tower_template(&overcharge, &upgrades(&[]), &super::config(), 0)
        .expect("template should be generated");
    assert_eq!(template.shoot_interval, 41);
    assert_eq!(template.used_cards[0].engraving, Some(1));
}

#[test]
fn uses_configured_tower_stats() {
    let template = get_highest_tower_template(
        &cards(&[(SPADES, TWO), (HEARTS, TWO)]),
        &upgrades(&[]),
        &crate::GameConfig::default_config(),
        0,
    )
    .expect("template should be generated");
    assert_eq!(template.kind, ONE_PAIR);
    assert_eq!(
        template.default_damage_raw,
        crate::Damage::from_integer(7).raw()
    );
    assert_eq!(
        template.default_attack_range_radius_raw,
        crate::WorldDistance::from_tiles(5).raw()
    );
    assert_eq!(
        template.shoot_interval,
        crate::SimTickSpan::from_millis_ceil(1_000).ticks()
    );
}
