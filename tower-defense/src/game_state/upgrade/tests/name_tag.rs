use super::super::*;
use crate::game_state::tower::{Tower, TowerTemplate};

#[test]
fn name_tag_applies_to_next_tower_and_consumes_it() {
    let mut game_state = super::support::create_mock_game_state();
    game_state
        .upgrade_state
        .upgrade(crate::game_state::upgrade::NameTagUpgrade::into_upgrade(
            3.0,
        ));
    game_state.left_dice = 0;

    let template = TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Spades,
        crate::card::Rank::Ace,
    );
    game_state.goto_placing_tower(template);

    assert!(game_state.upgrade_state.upgrades.iter().any(|upgrade| {
        matches!(upgrade, Upgrade::NameTag(..))
            && (upgrade.damage_multiplier().unwrap_or_default() - 3.0).abs() < f32::EPSILON
    }));
    let placed_template = super::support::first_hand_tower_template(&game_state);
    super::support::assert_template_has_damage_mul(&placed_template, 3.0);

    let mut next_template = TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Spades,
        crate::card::Rank::Ace,
    );
    game_state
        .upgrade_state
        .apply_pending_placement_bonuses(&mut next_template, game_state.left_dice);
    assert!(!next_template.default_status_effects.iter().any(|effect| {
        matches!(effect.kind, crate::game_state::tower::TowerStatusEffectKind::DamageMul { mul }
            if (mul - 3.0).abs() < f32::EPSILON)
    }));

    let tower = Tower::new(
        &placed_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    super::support::assert_tower_has_damage_mul(&tower, 3.0);
}
