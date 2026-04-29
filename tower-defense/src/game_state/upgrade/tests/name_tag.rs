use super::super::*;
use crate::game_state::tower::{Tower, TowerTemplate};
use namui::OneZero;

#[test]
fn name_tag_applies_to_next_tower_and_consumes_it() {
    let mut game_state = super::support::create_mock_game_state();
    game_state.upgrade_state.upgrade(Upgrade {
        kind: UpgradeKind::NameTag(NameTagUpgrade { damage_multiplier: 3.0, pending: true,
         }),
        value: OneZero::default(),
    });
    game_state.left_dice = 0;

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
                UpgradeKind::NameTag(NameTagUpgrade { damage_multiplier, pending: false }) if (damage_multiplier - 3.0).abs() < f32::EPSILON
            ))
    );
    let placed_template = super::support::first_hand_tower_template(&game_state);
    super::support::assert_template_has_damage_mul(&placed_template, 3.0);

    let tower = Tower::new(
        &placed_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    super::support::assert_tower_has_damage_mul(&tower, 3.0);
}
