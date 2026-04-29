use super::super::*;
use crate::game_state::tower::TowerTemplate;
use namui::OneZero;

#[test]
fn mirror_duplicates_next_acquired_tower() {
    let mut game_state = super::support::create_mock_game_state();
    game_state.upgrade_state.upgrade(Upgrade {
        kind: UpgradeKind::NameTag(NameTagUpgrade { damage_multiplier: 2.0, pending: true,
         }),
        value: OneZero::default(),
    });
    game_state.upgrade_state.upgrade(Upgrade {
        kind: UpgradeKind::Mirror(MirrorUpgrade { pending: true  }),
        value: OneZero::default(),
    });
    game_state.upgrade_state.upgrade(Upgrade {
        kind: UpgradeKind::Mirror(MirrorUpgrade { pending: true  }),
        value: OneZero::default(),
    });
    game_state.left_dice = 0;

    let tower_template = TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Spades,
        crate::card::Rank::Ace,
    );
    game_state.goto_placing_tower(tower_template);

    let slot_ids = game_state.hand.active_slot_ids();
    assert_eq!(slot_ids.len(), 3);
    let pending_mirror_count = game_state
        .upgrade_state
        .upgrades
        .iter()
        .filter(|upgrade| matches!(upgrade.kind, UpgradeKind::Mirror(MirrorUpgrade { pending: true  })))
        .count();
    assert_eq!(pending_mirror_count, 0);
    assert!(
        game_state
            .upgrade_state
            .upgrades
            .iter()
            .any(|upgrade| matches!(
                upgrade.kind,
                UpgradeKind::NameTag(NameTagUpgrade { damage_multiplier, pending: false }) if (damage_multiplier - 2.0).abs() < f32::EPSILON
            ))
    );

    for slot_id in slot_ids {
        let template = game_state
            .hand
            .get_item(slot_id)
            .and_then(|item| item.as_tower())
            .expect("expected mirrored slot item to be tower template");
        super::support::assert_template_has_damage_mul(template, 2.0);
    }
}
