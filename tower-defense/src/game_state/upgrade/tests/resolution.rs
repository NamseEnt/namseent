use super::super::*;
use crate::game_state::tower::{Tower, TowerTemplate};
use namui::OneZero;

#[test]
fn resolution_applies_remaining_reroll_damage_and_consumes_it() {
    let mut game_state = super::support::create_mock_game_state();
    game_state.upgrade_state.upgrade(Upgrade {
        kind: UpgradeKind::Resolution(ResolutionUpgrade { damage_multiplier_per_reroll: 0.25, pending: true,
         }),
        value: OneZero::default(),
    });
    game_state.left_dice = 2;

    let template = TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Spades,
        crate::card::Rank::Ace,
    );
    game_state.goto_placing_tower(template);

    assert!(
        game_state
            .upgrade_state
            .upgrades
            .iter()
            .any(|upgrade| matches!(
                upgrade.kind,
                UpgradeKind::Resolution(ResolutionUpgrade { damage_multiplier_per_reroll, pending: false }) if (damage_multiplier_per_reroll - 0.25).abs() < f32::EPSILON
            ))
    );

    let placed_template = super::support::first_hand_tower_template(&game_state);
    super::support::assert_template_has_damage_mul(&placed_template, 1.5);

    let tower = Tower::new(
        &placed_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    super::support::assert_tower_has_damage_mul(&tower, 1.5);
}
